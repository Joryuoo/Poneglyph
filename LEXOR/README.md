# LEXOR (Agartha)

The interpreter and compiler backend for the **Agartha** scripting language (`.agt` files).
Written in Rust, it compiles to WebAssembly for use in the [Frontend](../Frontend) and can
also run as a native CLI tool.

---

## Prerequisites

| Tool | Notes |
|------|-------|
| [Rust + Cargo](https://rustup.rs) | Use the latest stable toolchain |
| [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) | Builds the WASM package |

Install `wasm-pack` if you don't have it:

```bash
cargo install wasm-pack
```

---

## Building

### WebAssembly (for the Frontend)

```bash
wasm-pack build --target web --out-dir pkg
```

This compiles `src/lib.rs` and produces the `pkg/` folder
(`Agartha.js`, `Agartha.d.ts`, `Agartha_bg.wasm`) that the
Frontend loads at runtime.  
Copy or symlink `pkg/` into `../Frontend/src/pkg/` to update the web client.

### Native CLI binary

```bash
cargo build --release
```

The binary is at `target/release/Agartha`.

---

## Running

### CLI — execute a script file

```bash
cargo run -- scripts/test.agt
```

If no path is given, the program prompts for a filename and looks inside `scripts/`.
Files must have the `.agt` extension.

### CLI — interactive REPL

```bash
cargo run -- --test
```

Type Agartha code line-by-line; enter `exit` to run the accumulated code.

---

## Source files

### [`src/token.rs`](src/token.rs)

Defines every token the lexer can produce.

| Type | Purpose |
|------|---------|
| **`Token`** (enum, 54 variants) | One variant per keyword (`Declare`, `Print`, `Scan`, `If`, `For`, `RepeatWhen`, …), data-type keyword (`IntType`, `FloatType`, `CharType`, `StringType`, `BoolType`), literal (`IntLiteral(i32)`, `FloatLiteral(f64)`, `CharLiteral(char)`, `StringLiteral(String)`, `BoolLiteral(bool)`, `Identifier(String)`), operator (`Add`, `Subtract`, `Multiply`, `Divide`, `Modulo`, `Exponentiate`, `Concat`, `Assign`), boolean operator (`And`, `Or`, `Not`), comparison operator (`Equal`, `NotEqual`, `LessThan`, `GreaterThan`, `LessThanOrEqual`, `GreaterThanOrEqual`), or structural symbol (`LeftParen`, `RightParen`, `LeftBracket`, `RightBracket`, `Comma`, `Colon`, `Dollar`). |

---

### [`src/lexer.rs`](src/lexer.rs)

Converts raw source text into a flat list of tokens.

| Function | Purpose |
|----------|---------|
| `tokenize(input: &str) -> Result<Vec<(Token, usize)>, String>` | Scans the source string character-by-character, recognises keywords, identifiers, literals, operators, and `%%`-style comments, and returns a `(token, line_number)` pair for every recognised token. Errors on unknown characters. |

---

### [`src/ast.rs`](src/ast.rs)

Describes the shape of every node in the Abstract Syntax Tree.

| Type | Purpose |
|------|---------|
| **`Expression`** (enum) | A value-producing node. Variants: `IntType(i32)`, `FloatType(f64)`, `StringType(String)`, `CharType(char)`, `BoolType(bool)` — literal values; `Identifier(String)` — a variable look-up; `BinaryOp { left, operator, right }` — arithmetic, comparison, or boolean binary expression; `UnaryOp { operator, right }` — unary `Not` or negation. |
| **`Statement`** (enum) | A side-effecting node. Variants: `Declaration { var_type, declarations }` — one or more variable declarations with optional initializers; `Assignment { targets, value }` — assignment to one or more variables (supports chaining); `Print(Expression)` — output a value; `Scan(Vec<String>)` — read one or more user inputs into variables; `If { condition, body, else_ifs, else_body }` — conditional block; `For { initialization, condition, update, body }` — counted loop; `Repeat { condition, body }` — condition-checked loop (`RepeatWhen`). |
| **`Program`** (struct) | Root node of the tree. Fields: `statements: Vec<Statement>` — the ordered list of top-level statements produced by the parser. |

---

### [`src/parser.rs`](src/parser.rs)

Walks the token stream produced by the lexer and builds the AST.

| Type | Purpose |
|------|---------|
| **`Parser`** (struct) | Holds the token stream (`tokens: Vec<(Token, usize)>`), the current position (`current: usize`), whether any executable statement has been seen yet (`seen_executable: bool`, used to reject late declarations), and the line of the last successfully parsed statement (`last_parsed_line: usize`, used to enforce the one-statement-per-line rule). |
| `Parser::new(tokens)` | Constructor. |
| `Parser::parse(&mut self) -> Result<Program, String>` | Entry point. Expects a `ScriptArea` / `StartScript` header and `EndScript` footer; delegates individual statements to `parse_statement`. |

Internal helpers route to dedicated methods for each statement kind (`parse_declaration`, `parse_assignment`, `parse_print`, `parse_scan`, `parse_if`, `parse_for`, `parse_repeat`) and a Pratt-style expression cascade (`parse_expression` → `parse_logical_or` → `parse_logical_and` → `parse_equality` → `parse_comparison` → `parse_term` → `parse_factor` → `parse_unary` → `parse_primary`).

---

### [`src/interpreter.rs`](src/interpreter.rs)

Walks the AST and executes it.

| Type | Purpose |
|------|---------|
| **`RuntimeVariable`** (struct, private) | A single live variable. Fields: `declared_type: Token` — the type it was declared as; `value: Expression` — its current value as an AST literal node. |
| **`Interpreter`** (struct) | The execution engine. Fields: `memory: HashMap<String, RuntimeVariable>` — all live variables; `output: String` — accumulated `Print` output; `scan_inputs: Vec<String>` — pre-supplied input lines (from CLI stdin or the WASM caller); `scan_cursor: usize` — next unread input line; `needs_input: bool` — set when a `Scan` finds the input queue empty (used by the WASM layer to pause and ask the browser for more); `scan_target_count: usize` — how many inputs are still needed. |
| `Interpreter::new(scan_inputs_raw: &str)` | Parses a newline-delimited string of inputs into `scan_inputs`. |
| `Interpreter::interpret(&mut self, program: Program)` | Entry point. Runs each statement in order; accumulates errors in `output`. |

---

### [`src/lib.rs`](src/lib.rs)

The WebAssembly boundary — the only file visible to JavaScript.

| Function | Purpose |
|----------|---------|
| `run_agartha_code(source_code: &str, scan_inputs: &str) -> String` | Runs the full compiler pipeline (tokenize → parse → interpret) on the given source. Returns a JSON string: `{ "status": "ok" \| "error", "output": "…" }`, plus `"scan_target_count"` when the interpreter paused waiting for input. |

---

### [`src/simulate.rs`](src/simulate.rs)

File-based execution for the CLI.

| Function | Purpose |
|----------|---------|
| `run_file(filepath: &str)` | Reads the `.agt` file at the given path, feeds it through the full compiler pipeline with stdin as the input source, and prints the result. |

---

### [`src/test.rs`](src/test.rs)

Interactive REPL mode.

| Function | Purpose |
|----------|---------|
| `run_test()` | Reads lines from stdin into a buffer until the user types `exit`, then runs the buffered code through the compiler pipeline and prints the result. Useful for quick experimentation without creating a file. |

---

### [`src/errors.rs`](src/errors.rs)

Reserved for structured error types. Currently a placeholder — errors are returned as
`String` values throughout the pipeline.

---

### [`src/main.rs`](src/main.rs)

CLI entry point. Parses command-line arguments, enforces the `.agt` extension, and
dispatches to either `simulate::run_file` (file mode) or `test::run_test` (REPL mode).

---

## Compiler pipeline

```
source (.agt)
    │
    ▼
lexer::tokenize          →  Vec<(Token, line)>
    │
    ▼
parser::Parser::parse    →  Program (AST)
    │
    ▼
interpreter::Interpreter →  output / errors
    │
    ├─── WASM  →  lib::run_agartha_code  →  JSON string  →  Frontend
    └─── CLI   →  simulate::run_file / test::run_test    →  stdout
```
