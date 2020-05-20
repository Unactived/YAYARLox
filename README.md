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

Like in the book's implementation, the REPL insist on adding semicolons

Dynamically added, implicit semicolons would require addition after handling comments, therefore
after tokenizing in the current implementation; or drift away from the "normal" implementation
used to run files by prohibiting comments completely in the REPL.


## Improvements

**Currently**, the REPL automatically prints (as the name REPL should always imply) the
last expression evaluated, if it isn't `nil`. Therefore, statements will not produce
this kind of output.