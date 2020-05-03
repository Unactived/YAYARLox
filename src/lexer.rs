use std::fmt;
use crate::errors;

pub fn scan(code: String) -> (Vec<Token>, bool) {

    let mut state = Lexer {
        length: code.chars().count(),
        source: code.chars().collect(),

        start: 0,
        current: 0,
        line: 1,

        had_error: false,
    };

    let mut tokens: Vec<Token> = Vec::new();

    while state.current < state.length {

        state.start = state.current;

        let c = state.source[state.current];

        let matched = match c {
            ' ' | '\r' | '\t' => None,

            '\n' =>{
                state.line += 1;
                None
            },

            '(' => Some(TokenVariant::LeftParen),
            ')' => Some(TokenVariant::RightParen),
            '{' => Some(TokenVariant::LeftBrace),
            '}' => Some(TokenVariant::RightBrace),
            ',' => Some(TokenVariant::Comma),
            '.' => Some(TokenVariant::Dot),
            '-' => Some(TokenVariant::Minus),
            '+' => Some(TokenVariant::Plus),
            ';' => Some(TokenVariant::Semicolon),
            '*' => Some(TokenVariant::Star),

            '!' => if check('=', &mut state) {
                Some(TokenVariant::BangEqual)
            } else {
                Some(TokenVariant::Bang)
            },

            '=' => if check('=', &mut state) {
                Some(TokenVariant::EqualEqual)
            } else {
                Some(TokenVariant::Equal)
            },

            '<' => if check('=', &mut state) {
                Some(TokenVariant::LessEqual)
            } else {
                Some(TokenVariant::Less)
            },

            '>' => if check('=', &mut state) {
                Some(TokenVariant::GreaterEqual)
            } else {
                Some(TokenVariant::Greater)
            },

            '/' => if check('/', &mut state) {
                // A comment. Advance until EOF or EOL.
                while state.current + 1 < state.length && peek(&state) != '\n' {
                    state.current += 1
                }
                None
            } else {
                Some(TokenVariant::Slash)
            },

            '"' => {
                let res = string(&mut state);
                match res {
                    Ok(token) => Some(token),
                    Err(_) => None,
                }
            },

            '0'..='9' => {
                let res = number(&mut state);
                match res {
                    Ok(token) => Some(token),
                    Err(_) => None,
                }
            },

            'A'..='Z' | 'a'..='z' | '_' => {
                let id = identifier(&mut state);
                match &id[..] {

                    // reserved keywords
                    "and"    => Some(TokenVariant::And),
                    "class"  => Some(TokenVariant::Class),
                    "else"   => Some(TokenVariant::Else),
                    "false"  => Some(TokenVariant::False),
                    "for"    => Some(TokenVariant::For),
                    "fun"    => Some(TokenVariant::Fun),
                    "if"     => Some(TokenVariant::If),
                    "nil"    => Some(TokenVariant::Nil),
                    "or"     => Some(TokenVariant::Or),
                    "print"  => Some(TokenVariant::Print),
                    "return" => Some(TokenVariant::Return),
                    "super"  => Some(TokenVariant::Super),
                    "this"   => Some(TokenVariant::This),
                    "true"   => Some(TokenVariant::True),
                    "var"    => Some(TokenVariant::Var),
                    "while"  => Some(TokenVariant::While),

                    _ => Some(TokenVariant::Identifier(id)),
                }
            },

            _ => {
                errors::error(state.line, &format!("Unexpected character: {}.", c));
                state.had_error = true;
                None
            },
        };

        match &matched {
            Some(_) => add_token(
                &mut tokens,
                matched.unwrap(), 
                state.source[state.start..=state.current].into_iter().collect(), 
                &state
            ),
            None => (),
        }

        state.current += 1;

    }

    tokens.push(Token::new(
        TokenVariant::Eof, 
        String::new(),
        state.line
    ));

    (tokens, state.had_error)

}

/// Used to compare the next character to an expected one.
/// Advances if the character is as expected
fn check(expected: char, state: &mut Lexer) -> bool {

    if state.current + 1 >= state.length || state.source[state.current+1] != expected {
        false
    } else {
        state.current += 1;
        true
    }

}

/// Returns the next character that will be read
/// Doesn't advance the lexer
/// Boundary checking should be done upstream
fn peek(state: &Lexer) -> char {
    state.source[state.current+1]
}

/// Like peek but two characters ahead
fn peek_next(state: &Lexer) -> char {
    state.source[state.current+2]
}

fn add_token(tokens: &mut Vec<Token>, variant: TokenVariant, text: String, state: &Lexer) {
    tokens.push(Token::new(
        variant,
        text,
        state.line
    ));
}

fn string(state: &mut Lexer) -> Result<TokenVariant, ()> {
    while state.current + 1 < state.length && peek(&state) != '"' {
        if peek(&state) == '\n' {
            state.line += 1;
        }
        state.current += 1;
    }

    if state.current + 1 >= state.length {
        errors::error(state.line, "Unterminated string.");
        state.had_error = true;
        return Err(());
    }

    // closing `"`
    state.current += 1;

    let literal = state.source[state.start+1..state.current-1].into_iter().collect();

    Ok(TokenVariant::String(literal))
}

fn number(state: &mut Lexer) -> Result<TokenVariant, ()> {

    while state.current + 1 < state.length && peek(&state).is_digit(10) {
        state.current += 1;
    }

    // Fractional part
    if peek(&state) == '.' && state.current + 2 < state.length && peek_next(&state).is_digit(10) {
        state.current += 1;

        while state.current + 1 < state.length && peek(&state).is_digit(10) {
            state.current += 1;
        } 

    }

    let literal: Result<f64, _> = state.source[state.start..=state.current]
                       .into_iter()
                       .collect::<String>()
                       .parse();

    match literal {
        Ok(num) => Ok(TokenVariant::Number(num)),
        Err(_) => {
            errors::error(state.line, "Error while parsing Number literal.");
            state.had_error = true;
            Err(())
        }
    }
}

fn identifier(state: &mut Lexer) -> String {
    while state.current + 1 < state.length && (peek(&state).is_alphanumeric() || peek(&state) == '_'){
        state.current += 1;
    }

    state.source[state.start..=state.current].into_iter().collect()
}

#[derive(Clone, Debug)]
pub enum TokenVariant {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier(String), String(String), Number(f64),

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug)]
pub struct Token {
    pub class: TokenVariant,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(class: TokenVariant, lexeme: String, line: usize) -> Token {
        Token {
            class,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.class, self.lexeme)
    }
}

struct Lexer {
    length: usize,
    source: Vec<char>,

    start: usize,
    current: usize,
    line: usize,

    had_error: bool,
}