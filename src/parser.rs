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

    // checks the next variant in two steps and advances if correct
    fn expect_next(&mut self, variant: TokenVariant, message: &str) {
        if self.peek().class == variant {
            self.advance();
        } else {
            self.error(message);
        }
    }

    // checks the current variant
    fn expect(&mut self, variant: TokenVariant, message: &str) {
        if self.get().class != variant {
            self.error(message);
        }
    }

    // checks the current value and advances if correct
    fn consume(&mut self, variant: TokenVariant, message: &str) {
        if self.get().class == variant {
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

    // fn previous(&self) -> &Token {
    //     &self.tokens[self.current+1]
    // }

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

        self.expect_next(TokenVariant::Semicolon, "Expect ';' after expression.");

        Stmt::Var(Box::new(name), Box::new(initializer))
    }

    fn statement(&mut self) -> Stmt {

        if self.fit_still(vec![TokenVariant::If]) {
            self.advance();
            self.if_stmt()
        } else if self.fit_still(vec![TokenVariant::Print]) {
            self.advance();
            self.print_stmt()
        } else if self.fit_still(vec![TokenVariant::While]) {
            self.advance();
            self.while_stmt()
        } else if self.fit_still(vec![TokenVariant::For]) {
            self.advance();
            self.for_stmt()
        } else if self.fit_still(vec![TokenVariant::LeftBrace]) {
            self.advance();
            self.block_stmt()
        } else {
            self.expr_stmt()
        }

    }

    fn expr_stmt(&mut self) -> Stmt {
        let expr = self.expression();

        self.expect_next(TokenVariant::Semicolon, "Expect ';' after expression.");

        Stmt::Expression(Box::new(expr))
    }

    fn if_stmt(&mut self) -> Stmt {

        self.expect(TokenVariant::LeftParen, "Expect '(' after 'if'.");

        let condition = self.expression();

        self.consume(TokenVariant::RightParen, "Expect ')' after if condition.");

        let then_branch = self.statement();

        let else_branch = if !self.is_over() && self.fit(vec![TokenVariant::Else]) {
            self.advance();
            self.statement()
        } else {
            Stmt::Block(Box::new(Vec::new()))
        };

        Stmt::If(Box::new(condition), Box::new(then_branch), Box::new(else_branch))

    }

    fn block_stmt(&mut self) -> Stmt {
        let mut statements = Vec::new();

        while !self.is_over() && !self.fit_still(vec![TokenVariant::RightBrace]) {
            statements.push(self.declaration());
            self.advance();
        }

        self.expect(TokenVariant::RightBrace, "Expect '}' after block.");

        Stmt::Block(Box::new(statements))
    }

    fn while_stmt(&mut self) -> Stmt {

        self.expect(TokenVariant::LeftParen, "Expect '(' after 'while'.");

        let condition = self.expression();

        self.consume(TokenVariant::RightParen, "Expect ')' after while condition.");

        let body = self.statement();

        Stmt::While(Box::new(condition), Box::new(body))

    }

    fn for_stmt(&mut self) -> Stmt {

        self.consume(TokenVariant::LeftParen, "Expect '(' after 'for'.");

        // here statements consume the semicolon
        // the expressions that are the condition and the increment,
        // following this, don't.
        let initializer = if self.fit_still(vec![TokenVariant::Semicolon]) {
            // Another empty block as a void statement
            Stmt::Block(Box::new(Vec::new()))
        } else if self.fit_still(vec![TokenVariant::Var]) {
            self.var_declaration()
        } else  {
            self.expr_stmt()
        };

        let condition: Expr;

        if self.fit(vec![TokenVariant::Semicolon]) {

            // if no condition is given, it's a while-true loop
            // although, this is stil defined by the lone ';'

            condition = Expr::Literal(Box::new(Token {
                lexeme: String::from(";"),
                line: self.get().line,
                class: TokenVariant::True
            }));

        } else {
            self.advance();
            condition = self.expression();
            self.advance();
        }

        // second semicolon
        self.consume(TokenVariant::Semicolon, "Expect ';' after loop condition.");

        let increment: Expr;

        if self.fit_still(vec![TokenVariant::RightParen]) {
            // the actual value of the increment will be discarded
            // if none is given, let it be nil

            increment = Expr::Literal(Box::new(Token {
                lexeme: String::from(";"),
                line: self.get().line,
                class: TokenVariant::Nil
            }));
        } else {
            increment = self.expression();
            self.advance();
        }

        self.consume(TokenVariant::RightParen, "Expect ')' after for clauses.");

        let mut body = self.statement();

        // the body of the while loop: what is actually done,
        // and the increment part
        body = Stmt::Block(Box::new(vec![
            body,
            Stmt::Expression(Box::new(increment))
        ]));

        // wrap it in an actual while-loop with its condition
        body = Stmt::While(Box::new(condition), Box::new(body));

        // finally it's a block starting by the initializer and
        // then doing the loop
        Stmt::Block(Box::new(vec![
            initializer,
            body
        ]))

    }

    fn print_stmt(&mut self) -> Stmt {
        let value = self.expression();

        self.expect_next(TokenVariant::Semicolon, "Expect ';' after value.");

        Stmt::Print(Box::new(value))
    }

    // Expression grammar

    fn expression(&mut self) -> Expr {
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {

        let expr = self.or();

        if self.fit(vec![TokenVariant::Equal]) {
            let equal_token = self.get().clone();
            self.advance();

            let value = self.assignment();

            match expr {
                Expr::Variable(name) => return Expr::Assign(name, Box::new(value)),

                _ => errors::report(equal_token.line, &equal_token.lexeme, "Invalid assignment target."),
            }
        }

        expr
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();

        while !self.is_over() && self.fit(vec![TokenVariant::Or]) {
            let operator = self.get().clone();

            self.advance();

            let right = self.and();

            expr = Expr::Logical(Box::new(expr), Box::new(operator), Box::new(right))
        }

        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();

        while !self.is_over() && self.fit(vec![TokenVariant::And]) {
            let operator = self.get().clone();

            self.advance();

            let right = self.and();

            expr = Expr::Logical(Box::new(expr), Box::new(operator), Box::new(right))
        }

        expr
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
                self.expect_next(TokenVariant::RightParen, "Expected ')' after expression.");

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