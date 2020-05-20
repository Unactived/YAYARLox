/// Generate public Expressions and Statements

use crate::lexer::Token;

macro_rules! define_ast {
    ( $Category:ident := 
        $( $expr:ident : $( $type:ident$(<$($thing:ident),*>)? $name:ident ),* );*
    ) => {

        #[derive(Debug)]
        pub enum $Category {
        $(
            $expr(
                $(Box<$type$(<$($thing),*>)?>,)*
            ),
        )*
        }
    };
}

define_ast!(
    Expr :=
        Assign   : Token name, Expr value ;
        Binary   : Expr left, Token operator, Expr right ;
        Grouping : Expr expr ;
        Literal  : Token value ;
        Unary    : Token operator, Expr right ;
        Variable : Token name
);

define_ast!(
    Stmt :=
        Block      : Vec<Stmt> statements ;
        Expression : Expr expression ;
        If         : Expr condition, Stmt then_branch, Stmt else_branch ;
        Print      : Expr expression ;
        Var        : Token name, Expr initializer
);
