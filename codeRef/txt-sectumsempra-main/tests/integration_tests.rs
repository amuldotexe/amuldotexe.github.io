use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Once;
use txt_sectumsempra::Chunker;

// Used to ensure test files are created only once
static TEST_FILES_INIT: Once = Once::new();
static TEST_DIR: &str = "test_files";

// Helper function to generate or get existing test files
fn get_test_file(name: &str, size_mb: f64) -> PathBuf {
    let test_dir = PathBuf::from(TEST_DIR);
    let file_path = test_dir.join(format!("{}_{:.0}_mb.txt", name, size_mb));

    TEST_FILES_INIT.call_once(|| {
        // Create test directory if it doesn't exist
        if !test_dir.exists() {
            fs::create_dir_all(&test_dir).unwrap();
        }
    });

    if !file_path.exists() {
        gen_test_file(&file_path, size_mb);
    }

    file_path
}

// Helper function to clean up test files after all tests
fn cleanup_test_files() {
    let test_dir = PathBuf::from(TEST_DIR);
    if test_dir.exists() {
        let _ = fs::remove_dir_all(test_dir);
    }
}

#[test]
fn test_basic_split_and_validate() {
    let input_path = get_test_file("basic_test", 10.0); // 10MB file

    let chunks = Chunker::split_file(&input_path, 2.0).unwrap();
    assert_eq!(chunks.len(), 5);
    assert!(Chunker::validate(&input_path, &chunks).unwrap());

    // Cleanup chunks only, keep test file
    for chunk in chunks {
        fs::remove_file(chunk).unwrap();
    }
}

#[test]
fn test_chunk_size_exact() {
    let input_path = get_test_file("size_test", 5.0); // 5MB file

    let chunk_size_mb = 1.0;
    let chunks = Chunker::split_file(&input_path, chunk_size_mb).unwrap();
    
    // Check each chunk size except the last one
    for chunk in chunks.iter().take(chunks.len() - 1) {
        let size = fs::metadata(chunk).unwrap().len();
        assert_eq!(size, (chunk_size_mb * 1024.0 * 1024.0) as u64);
    }

    // Cleanup chunks only
    for chunk in chunks {
        fs::remove_file(chunk).unwrap();
    }
}

#[test]
fn test_output_directory_format() {
    let input_path = get_test_file("format_test", 1.0); // 1MB file
    let input_name = input_path.file_name().unwrap().to_str().unwrap();
    let base_name = input_name.split('.').next().unwrap_or(input_name);

    let chunks = Chunker::split_file(&input_path, 1.0).unwrap();
    
    // Check directory name format
    let dir_path = chunks[0].parent().unwrap();
    let dir_name = dir_path.file_name().unwrap().to_string_lossy();
    println!("Base name: {}", base_name);
    println!("Directory name: {}", dir_name);
    println!("Chunk name: {}", chunks[0].file_name().unwrap().to_string_lossy());
    assert!(dir_name.starts_with(&format!("{}-", base_name)));
    assert!(dir_name.split('-').nth(1).unwrap().parse::<u64>().is_ok());

    // Check chunk file name format
    let chunk_name = chunks[0].file_name().unwrap().to_string_lossy();
    let parts: Vec<&str> = chunk_name.split('-').collect();
    println!("Chunk name parts: {:?}", parts);
    assert!(chunk_name.starts_with(&format!("{}-part-", base_name)));
    assert!(parts.last().unwrap().split('.').next().unwrap().parse::<usize>().is_ok());

    // Cleanup chunks only
    for chunk in chunks {
        fs::remove_file(chunk).unwrap();
    }
}

#[test]
fn test_empty_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let input_path = temp_dir.path().join("empty.txt");
    fs::write(&input_path, "").unwrap();

    let result = Chunker::split_file(&input_path, 1.0);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("Empty file"));

    // Cleanup empty file
    fs::remove_file(input_path).unwrap();
}

#[test]
fn test_invalid_chunk_size() {
    let input_path = get_test_file("invalid_test", 1.0);
    assert!(Chunker::split_file(&input_path, 0.0).is_err());
    assert!(Chunker::split_file(&input_path, -1.0).is_err());

    // Test with chunk size larger than file
    let chunks = Chunker::split_file(&input_path, 2.0).unwrap();
    assert_eq!(chunks.len(), 1);
    assert!(Chunker::validate(&input_path, &chunks).unwrap());

    // Cleanup chunks only
    for chunk in chunks {
        fs::remove_file(chunk).unwrap();
    }
}

#[test]
fn test_missing_chunks_validation() {
    let input_path = get_test_file("validate_test", 2.0); // 2MB file

    let chunks = Chunker::split_file(&input_path, 1.0).unwrap();
    fs::remove_file(&chunks[0]).unwrap();

    // Test validation with missing chunk
    let result = Chunker::validate(&input_path, &chunks);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("Missing chunk file"));

    // Test validation with no chunks
    let result = Chunker::validate(&input_path, &[]);
    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("No chunks provided"));

    // Cleanup remaining chunks
    for chunk in chunks.iter().skip(1) {
        fs::remove_file(chunk).unwrap();
    }
}

// Helper function to generate test files
fn gen_test_file(path: &PathBuf, size_mb: f64) {
    let mut file = fs::File::create(path).unwrap();
    let size_bytes = (size_mb * 1024.0 * 1024.0) as u64;
    let mut remaining = size_bytes;
    let chunk_size = 8192; // 8KB buffer
    let pattern = b"0123456789";
    
    while remaining > 0 {
        let to_write = remaining.min(chunk_size as u64) as usize;
        let mut buffer = Vec::with_capacity(to_write);
        while buffer.len() < to_write {
            let remaining_space = to_write - buffer.len();
            if remaining_space >= pattern.len() {
                buffer.extend_from_slice(pattern);
            } else {
                buffer.extend_from_slice(&pattern[..remaining_space]);
            }
        }
        file.write_all(&buffer).unwrap();
        remaining -= to_write as u64;
    }
}

// Clean up test files when dropping the test binary
#[ctor::dtor]
fn cleanup() {
    cleanup_test_files();
}
