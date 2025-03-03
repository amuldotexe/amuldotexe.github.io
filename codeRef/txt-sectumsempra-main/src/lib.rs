mod error;

use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub use error::{ChunkError, Result};

const BUFFER_SIZE: usize = 8192; // 8KB buffer

pub struct Chunker;

impl Chunker {
    pub fn split_file(path: &Path, size_mb: f64) -> Result<Vec<PathBuf>> {
        // Validate inputs
        if size_mb <= 0.0 {
            return Err(ChunkError::InvalidInput("Chunk size must be greater than 0"));
        }

        let input_file = File::open(path)?;
        let file_size = input_file.metadata()?.len();
        let chunk_size = (size_mb * 1024.0 * 1024.0) as u64; // Convert MB to bytes
        
        if file_size == 0 {
            return Err(ChunkError::InvalidInput("Empty file"));
        }

        // Create output directory with timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let input_name = path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| ChunkError::InvalidInput("Invalid input filename"))?;
        
        // Get base name without extension and limit to 20 chars
        let base_name = input_name.split('.').next().unwrap_or(input_name);
        let truncated_name = if base_name.len() > 20 {
            &base_name[..20]
        } else {
            base_name
        };
        
        let output_dir = path.parent().unwrap_or(Path::new("."))
            .join(format!("{}-{}", truncated_name, timestamp));
        
        // Create directory and handle cleanup on error
        fs::create_dir_all(&output_dir)?;
        let mut created_chunks = Vec::new();

        let result: Result<Vec<PathBuf>> = (|| {
            let mut reader = BufReader::with_capacity(BUFFER_SIZE, input_file);
            let mut chunk_paths = Vec::new();
            let mut chunk_index = 0;
            let mut bytes_remaining = file_size;
            let mut buffer = vec![0; BUFFER_SIZE];

            while bytes_remaining > 0 {
                let current_chunk_size = bytes_remaining.min(chunk_size);
                let chunk_path = output_dir.join(format!("{}-part-{}.txt", truncated_name, chunk_index));
                let chunk_file = File::create(&chunk_path)?;
                created_chunks.push(chunk_path.clone());
                
                let mut writer = BufWriter::with_capacity(BUFFER_SIZE, chunk_file);
                let mut bytes_to_write = current_chunk_size;

                while bytes_to_write > 0 {
                    let to_read = (bytes_to_write as usize).min(buffer.len());
                    let read = reader.read(&mut buffer[..to_read])?;
                    if read == 0 {
                        break;
                    }
                    writer.write_all(&buffer[..read])?;
                    bytes_to_write -= read as u64;
                }
                
                writer.flush()?;
                chunk_paths.push(chunk_path);
                bytes_remaining = bytes_remaining.saturating_sub(current_chunk_size);
                chunk_index += 1;
                
                // Update progress (clear line and show percentage)
                print!("\r\x1B[K"); // Clear line
                print!("Progress: {:>3}%", ((file_size - bytes_remaining) * 100 / file_size));
                io::stdout().flush()?;
            }
            println!(); // New line after progress

            Ok(chunk_paths)
        })();

        // Clean up on error
        if result.is_err() {
            for chunk in created_chunks {
                let _ = fs::remove_file(chunk);
            }
            let _ = fs::remove_dir(&output_dir);
        }

        result
    }

    pub fn validate(original: &Path, chunks: &[PathBuf]) -> Result<bool> {
        if chunks.is_empty() {
            return Err(ChunkError::InvalidInput("No chunks provided for validation"));
        }

        // Verify all chunks exist before starting validation
        for chunk in chunks {
            if !chunk.exists() {
                return Err(ChunkError::InvalidInput("Missing chunk file"));
            }
        }

        let mut original_hasher = Sha256::new();
        let mut chunk_hasher = Sha256::new();
        
        // Hash original file
        let mut reader = BufReader::with_capacity(BUFFER_SIZE, File::open(original)?);
        io::copy(&mut reader, &mut original_hasher)?;
        
        // Hash all chunks in order
        for chunk_path in chunks {
            let mut reader = BufReader::with_capacity(BUFFER_SIZE, File::open(chunk_path)?);
            io::copy(&mut reader, &mut chunk_hasher)?;
        }
        
        Ok(original_hasher.finalize() == chunk_hasher.finalize())
    }
}
