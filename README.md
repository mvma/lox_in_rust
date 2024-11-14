Rlox - A Minimalistic Interpreter
Rlox is a minimal interpreter that implements a small subset of a programming language inspired by Lox. It supports a basic set of features including arithmetic expressions, conditionals, and loops, as well as user-defined variables.

Key Features
Tokenizer that converts source code into meaningful tokens
Basic arithmetic operations (+, -, *, /)
Support for keywords such as if, else, for, while, var, and more
Simple parser that converts tokens into an Abstract Syntax Tree (AST)
Support for strings and numbers
Can run both files and interactive prompts
How It Works
The interpreter works by scanning the source code and breaking it down into tokens. It then parses those tokens to produce an abstract syntax tree (AST) and evaluates the result.

Only ASCII Compatible
This interpreter currently only supports ASCII characters. Any source code containing non-ASCII characters will result in an error. This limitation is intentional and part of the current scope for development.