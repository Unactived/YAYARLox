use crate::ast::*;
use crate::errors;
use crate::lexer::{Token, TokenVariant};

pub fn parse(tokens: Vec<Token>) -> (Vec<Stmt>, bool) {

    let mut state = Parser {
        length: tokens.len(),
        tokens,

        current: 0,

        had_error: false,
    };

    let mut statements = Vec::new();

    while !state.is_over() {
        statements.push(state.declaration());
        state.advance();
    }

    (statements, state.had_error)

}

// Could be improved by taking all the chain at once,
// to avoid repeating function names.

macro_rules! binary {
    (
        $name:ident, $next:ident, [$($variant:ident),*]
    ) => {
        fn $name(&mut self) -> Expr {
            let mut left = self.$next();

            while !self.is_over() && self.fit(vec![$(TokenVariant::$variant),*]) {
                let operator = self.get()
                                   .clone();
                self.advance();
                let right = self.$next();
                left = Expr::Binary(Box::new(left), Box::new(operator), Box::new(right));
            }

            left
        }
    };
}


#[derive(Debug)]
struct Parser {
    length: usize,
    tokens: Vec<Token>,

    current: usize,

    had_error: bool,
}

impl Parser {

    // Progress

    fn is_over(&self) -> bool {
        self.current + 1 >= self.length
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume(&mut self, variant: TokenVariant, message: &str) {
        if self.peek().class == variant {
            self.advance();
        } else {
            self.error(message);
        }
    }

    // Context
    // Boundary checking should be done beforehand

    fn peek(&self) -> &Token {
        &self.tokens[self.current+1]
    }

    fn get(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn fit(&mut self, variants: Vec<TokenVariant>) -> bool {
        if variants.contains(&self.peek().class) {
            self.advance();
            return true;
        }

        false
    }

    fn fit_still(&mut self, variants: Vec<TokenVariant>) -> bool {
        if variants.contains(&self.get().class) {
            return true;
        }

        false
    }

    // Statement grammar

    fn declaration(&mut self) -> Stmt {
        if self.fit_still(vec![TokenVariant::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Stmt {
        match self.peek().class {
            TokenVariant::Identifier(_) => self.advance(),

            _ => self.error("Expect variable name."),
        }

        let name = self.get().clone();

        let initializer = if self.fit(vec![TokenVariant::Equal]) {
            self.advance();
            self.expression()
        } else {
            Expr::Literal(Box::new(Token::new(
                TokenVariant::Nil,
                String::from(""),
                name.line,
            )))
        };  

        self.consume(TokenVariant::Semicolon, "Expect ';' after expression.");

        Stmt::Var(Box::new(name), Box::new(initializer))
    }

    fn statement(&mut self) -> Stmt {

        if self.fit_still(vec![TokenVariant::Print]) {
            self.advance();
            self.print_stmt()
        } else {
            self.expr_stmt()
        }

    }

    fn expr_stmt(&mut self) -> Stmt {
        let expr = self.expression();

        self.consume(TokenVariant::Semicolon, "Expect ';' after expression.");

        Stmt::Expression(Box::new(expr))
    }

    fn print_stmt(&mut self) -> Stmt {
        let value = self.expression();

        self.consume(TokenVariant::Semicolon, "Expect ';' after value.");

        Stmt::Print(Box::new(value))
    }

    // Expression grammar

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // Recursive binary expression chain

    binary!(equality, comparison, [EqualEqual, BangEqual]);
    binary!(comparison, addition, [Greater, GreaterEqual, Less, LessEqual]);
    binary!(addition, multiplication, [Minus, Plus]);
    binary!(multiplication, unary, [Star, Slash]);

    fn unary(&mut self) -> Expr {

        if !self.is_over() && self.fit_still(vec![TokenVariant::Bang, TokenVariant::Minus]) {
            let operator = self.get()
                               .clone();
            self.advance();
            let right = self.unary();
            Expr::Unary(Box::new(operator), Box::new(right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        // if self.is_over() {
        //     self.error("Expected expression");
        //     return Expr::Literal(Box::new(Token::new(
        //         TokenVariant::Nil,
        //         String::from(""),
        //         0)));
        // }

        let current = self.get();

        match current.class {
            TokenVariant::False
            | TokenVariant::True
            | TokenVariant::Nil
            | TokenVariant::Number(_)
            | TokenVariant::String(_) => Expr::Literal(Box::new(current.clone())),

            TokenVariant::LeftParen => {
                self.advance();

                let expr = self.expression();
                self.consume(TokenVariant::RightParen, "Expected ')' after expression.");

                Expr::Grouping(Box::new(expr))
            },

            TokenVariant::Identifier(_) => {
                Expr::Variable(Box::new(current.clone()))
            },

            _ => {
                println!("{:?}", current);
                panic!("Illegal TokenVariant.");
            }

        }
    }

    fn error(&mut self, message: &str) {
        let token = &self.tokens[self.current];

        if token.class == TokenVariant::Eof {
            errors::report(token.line, " at end", message);
        } else {
            errors::report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
        self.had_error = true;
    }
}