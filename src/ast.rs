/// Generate public Expressions and Statements

use crate::lexer::Token;

#[macro_export]
macro_rules! make_struct {
    ( $name:ident, $( $field:ident : $type:ident ),* ) => {
        //#[derive(Debug)]
        pub struct $name {

        $(
            $field: $type,
        )*

        }
    };
}

/// $expr: expression name
#[macro_export]
macro_rules! define_ast {
    ( $( $expr:ident $( $type:ident $name:ident )* ),* ) => {

        $(
        make_struct!($expr, $( $name : $type ),*);
        )*

        //#[derive(Debug)]
        pub enum Expr {
            Expr(Box<Expr>),
        $(
            $expr(Box<$expr>),
        )*
        }
    };
}

define_ast!(
    Binary Expr left Token operator Expr right,
    Grouping Expr expression,
    Literal Token value,
    Unary  Token operator Expr right
);
