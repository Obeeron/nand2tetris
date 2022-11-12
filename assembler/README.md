# HACK assembler

The HACK assembler is a command-line application written in Rust used to compile HACK assembly code to binary.

## Building

The assembler can be built with the following command:

```bash
cargo build --release
```

The executable will be located at `target/release/assembler`.

## Usage:

```
Usage: assembler.exe [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>

Options:
  -o, --output <OUTPUT>   Compiled output file
  -F <OUTPUT_FORMAT>      Format of compiled output [default: binary] [possible values: text, binary]
  -i, --save-pp-file      Set this flag to save the intermediate preprocessed file
  -h, --help              Print help information
  -V, --version           Print version information
```

## Example

The Pong.asm file, part of the nand2tetris project can be compiled with the following command:

```bash
assembler Pong.asm
```

The output will be saved to `Pong.hack` by default.