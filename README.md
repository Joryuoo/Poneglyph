# About Agartha Poneglyph

**Agartha Poneglyph** is a web-based implementation of **LEXOR**, a strongly typed educational programming language created for **CS322 – Programming Languages**. It is designed to help students understand the fundamentals of programming language design, including lexical analysis, parsing, abstract syntax trees, runtime execution, type checking, input/output handling, arithmetic evaluation, logical expressions, and control flow.

LEXOR follows a strict program structure. Every program must begin with `SCRIPT AREA`, place executable code inside `START SCRIPT` and `END SCRIPT`, and declare variables immediately after `START SCRIPT` before any executable statement. Each line is treated as a single statement, comments begin with `%%`, reserved words are written in uppercase, `$` represents a newline, `&` is used for concatenation, and square brackets are used as escape codes.

## Basic Syntax

```lexor
SCRIPT AREA
START SCRIPT
DECLARE INT x = 10
PRINT: "Value: " & x
END SCRIPT
```

## Program Structure

```lexor
SCRIPT AREA
START SCRIPT
%% declarations must come first
DECLARE INT x
DECLARE FLOAT y
DECLARE CHAR c
DECLARE BOOL flag

%% executable statements come after declarations
x = 10
PRINT: x
END SCRIPT
```

## Data Types

LEXOR supports four declared data types:

| Data Type | Description |
|---|---|
| `INT` | Whole number with no decimal part |
| `FLOAT` | Number with a decimal part |
| `CHAR` | A single symbol or character |
| `BOOL` | Boolean value represented by `TRUE` or `FALSE` |

Although string literals can be used in output and concatenation, `STRING` is not treated as a valid declared data type in the refactored interpreter. The official LEXOR data type list contains `INT`, `CHAR`, `BOOL`, and `FLOAT`.

## Reserved Keywords

Common LEXOR reserved keywords include:

```text
SCRIPT AREA
START SCRIPT
END SCRIPT
DECLARE
INT
FLOAT
CHAR
BOOL
PRINT
SCAN
IF
ELSE IF
ELSE
START IF
END IF
FOR
START FOR
END FOR
REPEAT WHEN
START REPEAT
END REPEAT
AND
OR
NOT
TRUE
FALSE
```

## Operators

LEXOR supports arithmetic, relational, logical, unary, and concatenation operators.

| Category | Operators |
|---|---|
| Arithmetic | `+`, `-`, `*`, `/`, `%` |
| Relational | `>`, `<`, `>=`, `<=`, `==`, `<>` |
| Logical | `AND`, `OR`, `NOT` |
| Unary | `+`, `-` |
| Concatenation | `&` |
| Newline | `$` |

The language specification defines arithmetic operators, comparison operators, logical operators, and unary operators for positive and negative values.

## Input and Output

Output is handled using `PRINT:`.

```lexor
PRINT: "Hello, LEXOR"
PRINT: "Age: " & 20
PRINT: "Line 1" & $ & "Line 2"
```

Input is handled using `SCAN:`.

```lexor
DECLARE INT x
DECLARE INT y
SCAN: x, y
PRINT: x & "," & y
```

`SCAN` accepts one or more variables separated by commas, and the user must provide input values in the same order.

## Control Flow

LEXOR supports conditional and looping structures.

### Conditional Statement

```lexor
IF (x > 10)
START IF
PRINT: "Greater"
END IF
ELSE
START IF
PRINT: "Not greater"
END IF
```

### FOR Loop

```lexor
FOR (x = 0, x < 5, x = x + 1)
START FOR
PRINT: x
END FOR
```

### REPEAT WHEN Loop

```lexor
REPEAT WHEN (x < 5)
START REPEAT
PRINT: x
x = x + 1
END REPEAT
```

The specification includes `IF`, `ELSE IF`, `ELSE`, `FOR`, and `REPEAT WHEN` as the main control flow structures.

## Purpose

Poneglyph was built as both an interpreter and a learning tool. It allows users to write and run LEXOR programs directly in the browser while demonstrating how a programming language moves from source code to tokens, from tokens to an Abstract Syntax Tree, and from the AST to runtime execution.
