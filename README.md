# YAYARLox

Yet Another “Yet Another Rust implementation of the Lox language”

Tree-walk interpreter

- [x] Lexer
- [x] AST
- [x] Expression parser
- [x] Expression evaluation
- [x] Statements
  - [x] Variable declaration
  - [x] Variable assignment
  - [x] Static, lexical scope
- [x] Control flow
  - [x] If and If-else structures
  - [x] and, or conditional operators
  - [x] While loop
  - [x] For loop
- [ ] Functions
- [ ] Resolving / Binding
- [ ] Classes
- [ ] Inheritance


## Known limitations

### Like in the book's implementation

* the REPL insist on adding semicolons

Dynamically added, implicit semicolons would require addition after handling comments, therefore
after tokenizing in the current implementation; or drift away from the "normal" implementation
used to run files by prohibiting comments completely in the REPL.

### **Unlike** the books's implementation

* Synchronizing, panic mode haven't been implemented.

The interpreter will stop at the first parsing error encountered.

## Improvements

**Currently**, the REPL automatically prints (as the name REPL should always imply) the
last expression evaluated, if it isn't `nil`. Therefore, statements will not produce
this kind of output.

## Potential future improvements / changes

* Consider expressions without semicolons as what they are, expressions, and print them in the REPL, if
not nil. Likewise, no abusive printing of expressions, if they appear in statements.

* Use Rustyline or an equivalent crate to add readline support to the REPL