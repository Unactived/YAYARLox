/// Generate public Expressions and Statements

use crate::lexer::Token;

macro_rules! define_ast {
    ( $Category:ident := 
        $( $expr:ident : $( $type:ident $name:ident ),* );*
    ) => {

        #[derive(Debug)]
        pub enum $Category {
        $(
            $expr(
                $(Box<$type>,)*
            ),
        )*
        }
    };
}

define_ast!(
    Expr :=
        Binary   : Expr left, Token operator, Expr right ;
        Grouping : Expr expr ;
        Literal  : Token value ;
        Unary    : Token operator, Expr right ;
        Variable : Token name
);

define_ast!(
    Stmt :=
        Expression : Expr expression ;
        Print      : Expr expression ;
        Var        : Token name, Expr initializer
);
