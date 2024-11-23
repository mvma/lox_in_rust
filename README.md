# Rlox - A Minimalistic Interpreter  

Rlox is a personal project designed as a learning exercise, inspired by the Lox programming language. It supports a basic set of features such as arithmetic expressions, conditionals, loops, and user-defined variables.  

This project **does not aim to teach**, serve as a reference, or inspire others. It prioritizes progress and exploration over strict adherence to best practices or complete feature purity.  

## Key Features  

- **Tokenizer**: Converts source code into meaningful tokens.  
- **Arithmetic Operations**: Supports basic arithmetic (`+`, `-`, `*`, `/`).  
- **Control Flow**: Includes keywords like `if`, `else`, `for`, `while`, `var`, and more.  
- **Parser**: A simple parser that transforms tokens into an Abstract Syntax Tree (AST).  
- **Data Types**: Handles `strings` and `numbers`.  
- **Execution**: Runs both source code files and interactive prompts.  

## How It Works  

1. **Tokenization**: Scans source code to break it into tokens (e.g., operators, variables, keywords).  
2. **Parsing**: Produces an Abstract Syntax Tree (AST) from the tokens.  
3. **Evaluation**: Executes the AST to run the code and produce results.  

## Purpose and Limitations  

Rlox is **not** intended for production use, teaching others, or as a model implementation. It is a playground for learning and improving my understanding of interpreters and programming language concepts.  

### Current Limitations  

- **ASCII Only**: Rlox supports only ASCII characters; non-ASCII characters will result in errors. This constraint reflects the project’s current scope and priorities.  
