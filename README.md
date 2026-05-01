# LEXOR - Agartha Programming Language Interpreter

A Rust-based interpreter for the **LEXOR** (Agartha) programming language, a strongly-typed language designed to teach high school students the fundamentals of programming. This project implements a classic interpreter architecture with a lexer, parser, and interpreter using a hybrid compilation approach.

The interpreter is available as both a **native CLI application** and a **WebAssembly module** for integration with the web-based frontend.

## Overview

LEXOR is a pure interpreter that processes Agartha-language (`*.agt`) source files. The language supports essential programming concepts including:
- Variable declarations with multiple data types
- Arithmetic and logical operations
- Control flow structures (if/else, loops)
- Input/output operations
- String concatenation

## Project Architecture

This is part of the larger **Poneglyph** project with two main components:

- **LEXOR (Backend)**: Rust interpreter compiled to WebAssembly and CLI
- **[Frontend](../Frontend)**: React + Vite web interface for writing and running LEXOR programs

The interpreter can be used standalone via CLI or integrated into the frontend web application.

## Getting Started

### Prerequisites

- **Rust** (1.56+): Install from [https://rustup.rs/](https://rustup.rs/)
- **wasm-pack** (for WebAssembly): `cargo install wasm-pack`
- Linux/macOS/Windows environment with bash/command line support

### Building the Project

#### Build as Native CLI

```bash
cd /path/to/LEXOR
cargo build --release
```

This creates an executable in `target/release/agartha` (or `agartha.exe` on Windows).

#### Build as WebAssembly Module

```bash
cd Poneglyph/LEXOR
wasm-pack build --target web --release
```

This generates WebAssembly bindings in the `pkg/` directory for use with the frontend.

### Running Programs

#### CLI Mode (Native Execution)

The interpreter can run in two modes:

**Mode 1: Run a specific script from the `scripts/` folder**

```bash
cargo run --release code.agt
# or directly
./target/release/agartha code.agt
```

The interpreter automatically looks for files in the `scripts/` folder if no path is specified.

**Mode 2: Run a script with a full path**

```bash
cargo run --release path/to/your/file.agt
./target/release/agartha path/to/your/file.agt
```

**Mode 3: Run built-in tests**

```bash
cargo run
```

Running without arguments executes the test suite defined in `src/test.rs`.

#### Web Frontend Mode

```bash
cd Poneglyph/Frontend
npm run dev
```

### File Requirements

- All source files **must** have the `.agt` extension
- The interpreter will reject files with other extensions

## Project Structure

The interpreter follows a hybrid compilation model:

```
main.rs → Lexer → Tokens → Parser → AST → Interpreter → Output
```

### Core Modules

| Module | File | Purpose |
|--------|------|---------|
| **token** | `src/token.rs` | Defines all token types recognized by the language (keywords, operators, literals) |
| **lexer** | `src/lexer.rs` | Lexical analyzer that converts source code into tokens |
| **ast** | `src/ast.rs` | Abstract Syntax Tree data structures representing the program's syntax |
| **parser** | `src/parser.rs` | Syntax analyzer that builds the AST from tokens, validates grammar |
| **interpreter** | `src/interpreter.rs` | Executes the AST by evaluating expressions and statements |
| **simulate** | `src/simulate.rs` | File I/O handler and orchestrates the compilation pipeline |
| **test** | `src/test.rs` | Test suite for validating interpreter functionality |

## Test Scripts

The `scripts/` folder contains example Agartha programs with the `.agt` extension:

| Script | Purpose |
|--------|---------|
| `test.agt` | Comprehensive test with variable declaration, arithmetic operations, and string concatenation |
| `code.agt` | Basic example program with declarations and print statements |
| `controltest.agt` | Tests control flow structures (if/else, conditional logic) |
| `testloops.agt` | Demonstrates for loops and repeat/when loop structures |
| `iotest.agt` | Tests input/output operations (SCAN and PRINT commands) |
| `inc1.agt`, `inc2.agt`, `inc3.agt` | Increment and variable assignment tests |

Run any script with:
```bash
cargo run --release script_name.agt
```

## Language Syntax

### Program Structure

Every LEXOR program must follow this structure:

```
SCRIPT AREA
START SCRIPT
  DECLARE <type> <variables>
  <executable statements>
END SCRIPT
```

### Data Types

- **INT**: Integer numbers (4 bytes)
- **FLOAT**: Floating-point numbers (4 bytes)
- **CHAR**: Single character
- **BOOL**: Boolean values (TRUE/FALSE)
- **STRING**: Text literals

### Variable Declaration

```
DECLARE INT x, y, z
DECLARE CHAR letter='a'
DECLARE BOOL flag="TRUE"
DECLARE FLOAT pi=3.14
```

### Operators

**Arithmetic:**
- `+`, `-`, `*`, `/`, `%`
- `()` for grouping

**Comparison:**
- `>`, `<`, `>=`, `<=`, `==`, `<>`

**Logical:**
- `AND`, `OR`, `NOT`

**Special:**
- `&` - String concatenation
- `$` - Newline/carriage return
- `=` - Assignment

### Control Flow

**If/Else:**
```
IF (condition)
START IF
  <statements>
END IF
ELSE
START IF
  <statements>
END IF
```

**For Loop:**
```
FOR (initialization; condition; update)
START FOR
  <statements>
END FOR
```

**Repeat Loop:**
```
REPEAT WHEN (condition)
START REPEAT
  <statements>
END REPEAT
```

### I/O Operations

**Print output:**
```
PRINT: "Hello" & variable & $
```

**Scan input:**
```
SCAN: x, y, z
```

### Comments

```
%% This is a comment
```

## Example Program

```
SCRIPT AREA
START SCRIPT
DECLARE INT x=10, y=20
DECLARE BOOL result=

result = (x < y AND x <> 0)
PRINT: "x is " & x & $ & "Result: " & result
END SCRIPT
```

**Output:**
```
x is 10
Result: TRUE
```

## Interpreter Pipeline

1. **Lexical Analysis** (`lexer.rs`): Converts source code into tokens
2. **Syntax Analysis** (`parser.rs`): Validates grammar and builds AST
3. **Interpretation** (`interpreter.rs`): Executes the AST and manages runtime state
4. **I/O Handling** (`simulate.rs`): Manages file operations and pipeline orchestration

## Error Handling

The interpreter provides clear error messages:

- **Compiler Errors**: Wrong file extensions, missing files
- **File Errors**: Cannot read specified file
- **Parser Errors**: Invalid syntax, grammar violations
- **Runtime Errors**: Type mismatches, undefined variables, invalid operations

## Building and Testing

```bash
# Build the project as native CLI
cargo build --release

# Build as WebAssembly module
wasm-pack build --target web --release

# Run tests
cargo run

# Run a specific script
cargo run --release code.agt

# Clean build artifacts
cargo clean
```

## Integration with Frontend

The LEXOR interpreter is integrated into the web frontend through WebAssembly. To use it with the frontend:

1. **Build the WASM module** from this directory:
   ```bash
   wasm-pack build --target web --release
   ```

2. **Copy the generated module** to the frontend if needed:
   ```bash
   cp -r pkg/* ../Frontend/src/wasm/
   ```

3. **Run the frontend** (see [Frontend README](../Frontend/README.md)):
   ```bash
   cd ../Frontend
   npm install
   npm run dev
   ```

The frontend will import and use the WebAssembly module to execute LEXOR programs in the browser.

## Cargo Configuration

The project is configured for both CLI and WebAssembly compilation:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```

This allows the Rust code to be compiled as:
- A library (`rlib`) for the CLI application
- A WebAssembly C-compatible dynamic library (`cdylib`) for browser integration

## Language Specification

For detailed language specifications, refer to `01_LanguageSpecs.md` in the project root.

## Project Information

- **Name**: Agartha (LEXOR)
- **Version**: 0.1.0
- **Edition**: 2024
- **Type**: Pure Interpreter (CLI + WebAssembly)
- **Created**: Programming Languages Course (Senior High School)

## Notes

- All variable names are **case-sensitive**
- Variable names must start with a letter or underscore, followed by letters, underscores, or digits
- All reserved keywords are in **UPPERCASE**
- One statement per line
- Variable declarations must come immediately after `START SCRIPT`
- The `.agt` file extension is required

## Troubleshooting

**Error: "Unsa mani? I only read Agartha files (.agt) dong!"**
- Ensure your file has the `.agt` extension

**Error: "File Error: Cannot read '...' dong!"**
- Verify the file path and that the file exists
- For files in `scripts/`, you can omit the folder path

**Error: Parser or Runtime errors**
- Check the language specification in `01_LanguageSpecs.md`
- Review example scripts for correct syntax

**Error: wasm-pack command not found**
- Install wasm-pack: `cargo install wasm-pack`

**WebAssembly build fails**
- Ensure your Rust target includes `wasm32-unknown-unknown`:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
