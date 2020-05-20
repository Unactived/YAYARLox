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


struct Environment {
    // leads to the enclosing Environment, or is None if
    // it is the global scope
    enclosing: Option<Box<Environment>>,

    // Using String instead of &str is not just "simpler" but also
    // seems mandatory for mutable variables, which is about all variables.
    // We don't want to hold references to the lexemes of past values.
    scope: HashMap<String, types>
}

impl Environment {

    // fn new(environ: Environment) -> Environment

    fn define(&mut self, name: String, initializer: types) {
        self.scope.insert(
            name,
            initializer
        );
    }

    fn assign(&mut self, name: Token, value: types) -> Result<types, ()> {
        if self.scope.contains_key(&name.lexeme) {
            self.scope.insert(
                name.lexeme,
                value.clone()
            );
            Ok(value)
        } else if let Some(env) = &mut self.enclosing {
            (*env).assign(name, value)
        } else {
            error(&name, &format!("Undefined variable '{}'.", &name.lexeme));
            Err(())
        }
    }

    fn get(&self, name: &str) -> Result<types, ()> {

        match self.scope.get(name) {
            Some(val) => Ok((*val).clone()),
            None => {
                if let Some(env) = &self.enclosing {
                    return (*env).get(name);
                } else {
                    Err(())
                }
            }
        }
    }

}

pub struct Interpreter {

    environment: Environment
}

impl Interpreter {

    pub fn new() -> Self {
        Interpreter {
            environment: Environment {
                enclosing: None,
                scope: HashMap::new(),
            }
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<types, ()> {

        let mut last = types::nil;

        for stmt in statements.into_iter() {
            last = self.execute(stmt)?;
        }

        Ok(last)
    }

    // Interpreting

    fn execute(&mut self, stmt: Stmt) -> Result<types, ()> {

        match stmt {
            Stmt::Block(_)      => self.execute_block(stmt),
            Stmt::Expression(_) => self.execute_expr(stmt),
            Stmt::Print(_)      => self.execute_print(stmt),
            Stmt::Var(_,_)      => self.execute_var(stmt),
        }

    }

    #[allow(unused_must_use)]
    fn execute_block(&mut self, stmt: Stmt) -> Result<types, ()> {
        if let Stmt::Block(statements) = stmt {

            // let previous = self.environment;

            // self.environment = Environment {
            //     enclosing: Some(Box::new(previous)),
            //     scope: HashMap::new(),
            // };

            self.environment = Environment {
                enclosing: Some(Box::new(
                    std::mem::replace(&mut self.environment, Environment { enclosing: None, scope: HashMap::new() }))
                ),
                scope: HashMap::new(),
            };

            self.execution_bubble(*statements);

            let current = self.environment.enclosing.take().unwrap();
            self.environment = *current;

        }

        Ok(types::nil)
    }

    fn execution_bubble(&mut self, statements: Vec<Stmt>) -> Result<types, ()> {
        for stmt in statements.into_iter() {
            self.execute(stmt)?;
        }

        Ok(types::nil)
    }

    // #[allow(unused_must_use)]
    fn execute_expr(&mut self, stmt: Stmt) -> Result<types, ()> {
        if let Stmt::Expression(expr) = stmt {
            self.evaluate(*expr)
        } else {
            panic!("execute_expr expects Stmt::Expression");
        }

        // Ok(types::nil)
    }

    fn execute_print(&mut self, stmt: Stmt) -> Result<types, ()> {
        if let Stmt::Print(expr) = stmt {
            let value = self.evaluate(*expr)?;
            println!("{}", value);
        }

        Ok(types::nil)
    }

    fn execute_var(&mut self, stmt: Stmt) -> Result<types, ()> {
        if let Stmt::Var(name, initializer) = stmt {
            let name = name.lexeme;
            let initializer = self.evaluate(*initializer)?;

            self.environment.define(name, initializer);
        }

        Ok(types::nil)
    }

    fn evaluate(&mut self, expression: Expr) -> Result<types, ()> {
        match expression {
            Expr::Assign(_,_)   => self.evaluate_assign(expression),
            Expr::Literal(_)    => self.evaluate_literal(expression),
            Expr::Grouping(_)   => self.evaluate_parentheses(expression),
            Expr::Unary(_,_)    => self.evaluate_unary(expression),
            Expr::Binary(_,_,_) => self.evaluate_binary(expression),
            Expr::Variable(_)   => self.get_variable(expression),
        }
    }

    fn evaluate_assign(&mut self, expression: Expr) -> Result<types, ()> {
        if let Expr::Assign(name, value) = expression {
            let (name, value) = (*name, self.evaluate(*value)?);

            self.environment.assign(name, value)

        } else {
            panic!("expression should be an Assign");
        }
    }

    fn evaluate_literal(&self, expression: Expr) -> Result<types, ()> {
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

    fn evaluate_parentheses(&mut self, expression: Expr) -> Result<types, ()> {
        if let Expr::Grouping(val) = expression {
            Ok(self.evaluate(*val)?)
        } else {
            panic!("expression should be a Grouping");
        }
    }

    fn evaluate_unary(&mut self, expression: Expr) -> Result<types, ()> {
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

    fn evaluate_binary(&mut self, expression: Expr) -> Result<types, ()> {
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

    fn get_variable(&self, expression: Expr) -> Result<types, ()> {
        if let Expr::Variable(token) = expression {

            let original = token.clone();

            match token.class {
                TokenVariant::Identifier(ident) => {
                    let attempt = self.environment.get(&ident);

                    if let Ok(_) = attempt {
                        attempt
                    } else {
                        error(&original, &format!("Variable '{}' doesn't exist.", &ident));
                        Err(())
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