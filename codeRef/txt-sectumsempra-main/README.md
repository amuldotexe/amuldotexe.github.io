# txt-sectumsempra

A minimalistic file chunking utility that splits large text files into smaller chunks of equal size.

## Goal

Split a large text file into smaller chunks of specified size (supports both integer and decimal sizes in MB).

## Product Requirements Document (PRD)

1. **Input**: Accept any text file regardless of size.
2. **Output**: Generate files of equal size of N MB each (N can be a decimal value, minimum 0.1 MB).
3. **CLI Interface**: `cargo run absolute-path-input.txt --size size-in-MB` & output files will be placed in a new folder `input-start-timestamp` with names `input-part-0` to `input-part-N`.
4. **Validation**: Verify output files match input checksum.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/txt-sectumsempra.git
cd txt-sectumsempra

# Build the project
cargo build --release
```

## Usage

```bash
# Basic usage (supports both integer and decimal values)
cargo run -- <absolute-path-input.txt> --size <size-in-MB>

# Example with 1 MB chunks
cargo run -- /home/amuldotexe/Downloads/threads_amuldotexe_1731404941.txt --size 1

# Example with 0.3 MB chunks
cargo run -- /home/amuldotexe/Downloads/threads_amuldotexe_1731404941.txt --size 0.3
```

The output files will be placed in a new folder named `input-start-timestamp` with names ranging from `input-part-0` to `input-part-N`.

### Features

- **Universal Input**: Accepts any text file regardless of size
- **Flexible Chunk Sizes**: Supports both integer (1, 2, 3...) and decimal (0.1, 0.3, 1.5...) MB sizes
- **Equal-sized Chunks**: Generates output files of exactly N MB each
- **Data Integrity**: Validates output files against input checksum

### Project Structure

```
txt-sectumsempra/
├── src/           # Source code
├── tests/         # Test files
├── txt-ref/       # Reference files
└── Cargo.toml     # Project dependencies
```

### Dependencies

- `sha2`: For checksum calculation
- `clap`: Command-line argument parsing

## Development

To run tests:
```bash
cargo test
```

## License

This project is open source and available under the MIT License.
