# LEXOR Interpreter Architecture

## 1. Project Overview
This project is a custom interpreter for **LEXOR**, a strongly typed educational programming language. It is implemented entirely in Rust and follows a classic tree-walk interpreter architecture. 

The pipeline translates raw LEXOR source code into lexical tokens, builds an Abstract Syntax Tree (AST), and executes the nodes directly while maintaining runtime memory.

## 2. System Pipeline
The execution of a LEXOR program follows a strict, unidirectional data flow:

```mermaid
graph TD
    A[Source Code text] -->|lexer.rs| B(Tokens Vec<Token>)
    B -->|parser.rs| C(Abstract Syntax Tree)
    C -->|interpreter.rs| D{Execution Environment}
    D --> E[Standard Output / Terminal] 