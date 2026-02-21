# Rust-Calc

A simple desktop calculator built in Rust with a graphical user interface.

## Features

- Basic arithmetic operations (addition, subtraction, multiplication, division)
- Chain operations support
- Keyboard support for quick input
- Error handling (division by zero protection)
- Clean, intuitive UI with visual feedback

## Building

### Requirements

- Rust 1.70 or later

### Build Instructions

To build the application as a release executable:

```bash
cargo build --release
```

The compiled executable will be located at:
```
target/release/rust-calc.exe
```

### Development Build

For development with debug output:

```bash
cargo build
```

Executable: `target/debug/rust-calc.exe`

## Running

After building, execute the generated EXE file directly or run:

```bash
cargo run --release
```
