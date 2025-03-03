use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file path
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Size of each chunk in MB (minimum 0.1)
    #[arg(short, long, default_value_t = 1.0)]
    size: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    if args.size < 0.1 {
        eprintln!("Error: Chunk size must be at least 0.1 MB");
        std::process::exit(1);
    }

    if !args.input.exists() {
        eprintln!("Error: Input file does not exist");
        std::process::exit(1);
    }

    if !args.input.is_file() {
        eprintln!("Error: Input path is not a file");
        std::process::exit(1);
    }

    println!("Splitting file: {}", args.input.display());
    let chunks = match txt_sectumsempra::Chunker::split_file(&args.input, args.size) {
        Ok(chunks) => chunks,
        Err(e) => {
            eprintln!("Error splitting file: {}", e);
            std::process::exit(1);
        }
    };
    println!("Created {} chunks", chunks.len());
    
    println!("Validating chunks...");
    match txt_sectumsempra::Chunker::validate(&args.input, &chunks) {
        Ok(true) => println!("✅ Validation successful!"),
        Ok(false) => {
            eprintln!("❌ Validation failed!");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error during validation: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}