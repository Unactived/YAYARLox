use std::{fmt, collections::HashMap};
use crate::ast::*;
use crate::errors;
use crate::lexer::{Token, TokenVariant};

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum types {
    nil,
    boolean(bool),
    number(f64),
    string(String),
}

impl fmt::Display for types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
            match &self {
                types::nil => String::from("nil"),
                types::boolean(val) => val.to_string(),
                types::number(val) => val.to_string(),
                types::string(val) => val.to_string(),
            }
        )
    }
}

impl fmt::Debug for types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
            match &self {
                types::nil => String::from("nil"),
                types::string(val) => format!("\"{}\"", val.to_string()),


                types::boolean(val) => val.to_string(),
                types::number(val) => val.to_string(),                
            }
        )
    }
}


pub fn interpret(statements: Vec<Stmt>) -> Result<types, ()> {

    let mut interpreter = Interpreter { environment: HashMap::new() };

    let mut last = types::nil;

    for stmt in statements.into_iter() {
        last = interpreter.execute(stmt)?;
    }

    Ok(last)
}

pub struct Interpreter {
    // Using String instead of &str is not just "simpler" but also
    // seems mandatory for mutable variables, which is about all variables.
    // We don't want to hold references to the lexemes of past values.
    pub environment: HashMap<String, types>,
}

impl Interpreter {

    // Interpreting

    fn execute(self: &mut Interpreter, stmt: Stmt) -> Result<types, ()> {

        match stmt {
            Stmt::Expression(_) => self.execute_expr(stmt),
            Stmt::Print(_)      => self.execute_print(stmt),
            Stmt::Var(_,_)      => self.execute_var(stmt),
        }

    }

    // #[allow(unused_must_use)]
    fn execute_expr(self: &mut Interpreter, stmt: Stmt) -> Result<types, ()> {
        if let Stmt::Expression(expr) = stmt {
            self.evaluate(*expr)
        } else {
            panic!("execute_expr expects Stmt::Expression");
        }

        // Ok(types::nil)
    }

    fn execute_print(self: &mut Interpreter, stmt: Stmt) -> Result<types, ()> {
        if let Stmt::Print(expr) = stmt {
            let value = self.evaluate(*expr)?;
            println!("{}", value);
        }

        Ok(types::nil)
    }

    fn execute_var(self: &mut Interpreter, stmt: Stmt) -> Result<types, ()> {
        if let Stmt::Var(name, initializer) = stmt {
            let name = name.lexeme;
            let initializer = self.evaluate(*initializer)?;

            self.environment.insert(
                name,
                initializer,
            );
        }

        Ok(types::nil)
    }

    fn evaluate(self: &Interpreter, expression: Expr) -> Result<types, ()> {
        match expression {
            Expr::Literal(_)    => self.evaluate_literal(expression),
            Expr::Grouping(_)   => self.evaluate_parentheses(expression),
            Expr::Unary(_,_)    => self.evaluate_unary(expression),
            Expr::Binary(_,_,_) => self.evaluate_binary(expression),
            Expr::Variable(_)   => self.get_variable(expression),
        }
    }

    fn evaluate_literal(self: &Interpreter, expression: Expr) -> Result<types, ()> {
        if let Expr::Literal(val) = expression {
            let boxed = val;

            match (*boxed).class {
                TokenVariant::True        => Ok(types::boolean(true)),
                TokenVariant::False       => Ok(types::boolean(false)),
                TokenVariant::Number(val) => Ok(types::number(val)),
                TokenVariant::String(val) => Ok(types::string(val)),
                TokenVariant::Nil         => Ok(types::nil),

                _ => panic!("Literal holds illegal TokenVariant"),
            }
        } else {
            panic!("expression should be a Literal");
        }
    }

    fn evaluate_parentheses(self: &Interpreter, expression: Expr) -> Result<types, ()> {
        if let Expr::Grouping(val) = expression {
            Ok(self.evaluate(*val)?)
        } else {
            panic!("expression should be a Grouping");
        }
    }

    fn evaluate_unary(self: &Interpreter, expression: Expr) -> Result<types, ()> {
        if let Expr::Unary(operator, val) = expression {

            let operator = *operator;

            match (&operator.class, self.evaluate(*val)?) {

                (TokenVariant::Minus, target) => {
                    let target = check_number_operand(operator, target)?;
                    Ok(types::number(-target))
                },
                (TokenVariant::Bang, target) => Ok(types::boolean(!is_truthy(target))),

                _ => panic!("Unary should hold Minus and a number, or Bang and any type"),

            }

        } else {
            panic!("expression should be an Unary");
        }
    }

    fn evaluate_binary(self: &Interpreter, expression: Expr) -> Result<types, ()> {
        if let Expr::Binary(left, operator, right) = expression {

            let (left, right) = (self.evaluate(*left)?, self.evaluate(*right)?);
            let operator = *operator;

            match operator.class {

                TokenVariant::Plus => {
                    match (left, right) {
                        (types::number(val1), types::number(val2)) => Ok(types::number(val1 + val2)),
                        (types::string(val1), types::string(val2)) => Ok(types::string(val1 + &val2)),

                        _ => {
                            error(&operator, "Operands must be two numbers or two strings");
                            Err(())
                        }
                    }
                },
                TokenVariant::Minus => {
                    let (a, b) = check_number_operands(&operator, left, right)?;
                    Ok(types::number(a - b))
                },
                TokenVariant::Slash => {
                    let (a, b) = check_number_operands(&operator, left, right)?;
                    Ok(types::number(a / b))
                },
                TokenVariant::Star => {
                    let (a, b) = check_number_operands(&operator, left, right)?;
                    Ok(types::number(a * b))
                },
                TokenVariant::Greater => {
                    let (a, b) = check_number_operands(&operator, left, right)?;
                    Ok(types::boolean(a > b))
                },
                TokenVariant::GreaterEqual => {
                    let (a, b) = check_number_operands(&operator, left, right)?;
                    Ok(types::boolean(a >= b))
                },
                TokenVariant::Less => {
                    let (a, b) = check_number_operands(&operator, left, right)?;
                    Ok(types::boolean(a < b))
                },
                TokenVariant::LessEqual => {
                    let (a, b) = check_number_operands(&operator, left, right)?;
                    Ok(types::boolean(a <= b))
                },
                TokenVariant::BangEqual => {
                    Ok(types::boolean(left != right))
                },
                TokenVariant::EqualEqual => {
                    Ok(types::boolean(left == right))
                },

                _ => {
                    println!("{:?}", operator);
                    panic!("Illegal TokenVariant for Binary")
                },
            }

        } else {
            panic!("expression should be a Binary");
        }
    }

    fn get_variable(self: &Interpreter, expression: Expr) -> Result<types, ()> {
        if let Expr::Variable(token) = expression {

            let original = token.clone();

            match token.class {
                TokenVariant::Identifier(ident) => {

                    match self.environment.get(&ident) {
                        Some(val) => Ok((*val).clone()),
                        None => {
                            error(&original, &format!("Variable '{}' doesn't exist.", &ident));
                            Err(())
                        }
                    }

                },

                _ => panic!("Variable should hold an Identifier"), 
            }

        } else {
            panic!("expression should be a Variable");
        }
    }

}

fn check_number_operand(operator: Token, operand: types) -> Result<f64, ()> {
    if let types::number(val) = operand {
        Ok(val)
    } else {
        error(&operator, "Operand must be a number");
        Err(())
    }
}

fn check_number_operands(operator: &Token, left: types, right: types) -> Result<(f64, f64), ()> {
    if let (types::number(val1), types::number(val2)) = (left, right) {
        Ok((val1, val2))
    } else {
        error(operator, "Operands must be numbers");
        Err(())
    }
}

/// Ruby: are falsey false and nil
/// everything else is truthy
fn is_truthy(object: types) -> bool {
    match object {
        types::boolean(false) | types::nil => false,
        _ => true,
    }
}

fn error(token: &Token, message: &str) {
    errors::report(token.line, &format!(" at '{}'", &token.lexeme), message);
}