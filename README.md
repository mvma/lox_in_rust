# Rlox - A Minimalistic Interpreter

Rlox is a minimal interpreter inspired by the Lox programming language. It supports a basic set of features including arithmetic expressions, conditionals, loops, and user-defined variables.

## Key Features

- **Tokenizer**: Converts source code into meaningful tokens.
- **Arithmetic Operations**: Supports basic arithmetic (`+`, `-`, `*`, `/`).
- **Control Flow**: Includes support for keywords like `if`, `else`, `for`, `while`, `var`, and more.
- **Parser**: A simple parser that converts tokens into an Abstract Syntax Tree (AST).
- **Data Types**: Supports `strings` and `numbers`.
- **Execution**: Can execute both source code files and interactive prompts.

## How It Works

1. **Tokenization**: The interpreter scans the source code and breaks it down into tokens (e.g., operators, variables, keywords).
2. **Parsing**: Tokens are then parsed to produce an Abstract Syntax Tree (AST).
3. **Evaluation**: The AST is evaluated to execute the code and produce the result.

## Limitations

- **ASCII Only**: Currently, the interpreter only supports ASCII characters. Any source code containing non-ASCII characters will result in an error. This limitation is intentional and part of the current scope for development.
