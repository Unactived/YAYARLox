# YAYARLox

Yet Another “Yet Another Rust implementation of the Lox language”

Tree-walk interpreter

- [x] Lexer
- [x] AST
- [x] Expression parser
- [x] Expression evaluation
- [x] Statements
- [ ] Control flow
- [ ] Functions
- [ ] Resolving / Binding
- [ ] Classes
- [ ] Inheritance


## Known limitations

* Like in the book's implementation, the REPL insist on adding semicolons

Dynamically added, implicit semicolons would require addition after handling comments, therefore
after tokenizing in the current implementation; or drift away from the "normal" implementation
used to run files by prohibiting comments completely in the REPL.

* **Currently**, the scope of global variables lasts for one iteration of the loop

This is not the case in the book's implementation, and in fact this makes a REPL useless.

The current implementation of the REPL is similar to the book: just like running a file
calls the function `run` with its contents, the REPL calls `run` with the line
entered by the user.

In the future, this might be solved by transmitting an Interpreter struct, to create the
idea of shared session. Maybe run could become a method of the Interpreter struct instead.


## Improvements

**Currently**, the REPL automatically prints (as the name REPL should always imply) the
last expression evaluated, if it isn't `nil`. Therefore, statements will not produce
this kind of output.