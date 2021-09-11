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
  - [x] Native functions (clock)
  - [x] Declarations
  - [ ] Returning
  - [ ] Local functions and closures
- [ ] Resolving / Binding
- [ ] Classes
- [ ] Inheritance

### Considerations

function type should hold a Function struct with name and parameters and not only the Callable ;
this is notably necessary for properly printing them.

Though this is what the Java of the book does, should arity really be a function? This gives Python's @property
vibes but can a function's arity even change without being completely redefined? 

In the 'execute' family of functions (interpreter), these take ownership of statements, which is fine most of the
time since a statement is executed once, except that's not always the case for loops and if there's repetition in general.
The execution of while statement in particular clones the body of the statement at each iteration which shouts bad design.

Similar ownership-stealing consideration for environment keys.

Don't really understand why bother having the body of blocks and functions be explicitly a *list* of statements instead
of a single "statement of statements" like in control flow. Only difference I see between these 2 sets: new scope.
Does it correlate? How?

Didn't properly check yet but : the book seems to always define functions' scope as direct child of global scope.
Pretty sure I instead just made them a new nested scope, which thus *can* access parent scopes besides global, when
the book's standard would not. Is this closures? which are implemented afterwards.

## Known limitations

### Like in the book's implementation

* the REPL insist on adding semicolons

Dynamically added, implicit semicolons would require addition after handling comments, therefore
after tokenizing in the current implementation; or drift away from the "normal" implementation
used to run files by prohibiting comments completely in the REPL.

### **Unlike** the books's implementation

* Synchronizing, panic mode haven't been implemented.

The interpreter will stop at the first parsing error encountered.

* declared and native functions are two different Lox types

Couldn't wrap my head around this.

## Differences / Improvements

* **Currently**, the REPL automatically prints (as the name REPL should always imply) the
last expression evaluated, if it isn't `nil`. Therefore, statements will not produce
this kind of output.

## Potential future improvements / changes

* Consider expressions without semicolons as what they are, expressions, and print them in the REPL, if
not nil. Likewise, no abusive printing of expressions, if they appear in statements.

* Use Rustyline or an equivalent crate to add readline support to the REPL
