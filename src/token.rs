use std::fmt::{Display, Formatter, Result as FMTResult};
use regex::Regex;
use std::error::Error;

// acceptable operators
const OPERATORS: [char; 4] = ['*', '+', '-', '/'];
// acceptable symbols
const SYMBOLS: [char; 4] = ['(', ')', '{', '}'];

#[derive(Debug)]
pub enum TokenError {
    InvalidCharacter(char)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TokenType {
    Operator,
    Symbol,
    Number,
}

#[derive(Debug, Clone)]
enum TokenValue {
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Division,
    Power,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrackets,
    CloseBrackets,
}

#[derive(Debug, Clone)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    value: TokenValue,
}

impl TokenType {
    pub fn get_type(c: char) -> Result<TokenType, TokenError> {
        if c.is_numeric() {
            Ok(TokenType::Number)
        } else if OPERATORS.contains(&c) {
            Ok(TokenType::Operator)
        } else if SYMBOLS.contains(&c) {
            Ok(TokenType::Symbol)
        } else {
            Err(TokenError::InvalidCharacter(c))
        }
    }
}

impl TokenValue {
    fn from_string(value: &String) -> TokenValue {
        match value.as_str() {
            "+" => TokenValue::Plus,
            "-" => TokenValue::Minus,
            "*" => TokenValue::Multiply,
            "**" => TokenValue::Power,
            "(" => TokenValue::OpenParenthesis,
            ")" => TokenValue::CloseParenthesis,
            "{" => TokenValue::OpenBrackets,
            "}" => TokenValue::CloseBrackets,
            "/" => TokenValue::Division,
            _ => TokenValue::Number(value.parse::<i64>().expect(&format!("Failed parsing to i64: {}", value)))
        }
    }
}

impl Token {
    fn new(ttype: &TokenType, lexeme: &String) -> Token {
        Token {
            lexeme: lexeme.clone(),
            ttype: ttype.clone(),
            value: TokenValue::from_string(lexeme),
        }
    }

    pub fn print_token_table(table: Vec<Token>) {
        println!("|{:>20}|{:>20}|{:>20}", "Lexeme", "Token Type", "Value");
        let dashes = "-".repeat(20);
        println!("|{}|{}|{}", dashes, dashes, dashes);
        for token in table {
            println!("|{:>20}|{:>20}|{:>20}", token.lexeme, token.ttype.to_string(), token.value)
        }
    }

    fn is_next_token_numeric(stream: &String, current_index: usize) -> bool
    {
        match stream.chars().nth(current_index + 1) {
            Some(ch) => ch.is_numeric(),
            None => false
        }
    }

    pub fn process_token_stream(token_stream: &String) -> Result<Vec<Token>, TokenError> {
        let token_stream = Regex::new(r"([\n ])*").unwrap().replace_all(token_stream, "").to_string();

        let mut token_vector: Vec<Token> = vec![];
        // lexeme buffer
        let mut blexeme: String = String::new();
        // token type buffer
        let mut bttype: TokenType = TokenType::Symbol;


        for (index, ch) in token_stream.char_indices() {
            let token_type = TokenType::get_type(ch)?;

            // if token is - and last token inserted was not a number and next token is a number
            if ch == '-' && bttype != TokenType::Number && Token::is_next_token_numeric(&token_stream, index) {
                // save last buffer value
                if !blexeme.is_empty() {
                    token_vector.push(Token::new(&bttype, &blexeme));
                }
                // clear the buffer
                blexeme.clear();

                // push minus and declare as number
                blexeme.push(ch);
                bttype = TokenType::Number;
            } else if token_type == bttype && bttype == TokenType::Number {
                // if token is the same as before an still a number
                blexeme.push(ch);
            } else if ch == '*' && blexeme == "*".to_string() {
                // case found the symbol * check if is power symbol
                blexeme.push(ch);
            } else {
                // case buffer is not empty and the symbol is diferent as the one before
                // just push the current buffer and after that cleans to reset
                if !blexeme.is_empty() {
                    token_vector.push(Token::new(&bttype, &blexeme));
                }

                blexeme.clear();
                blexeme.push(ch);
                bttype = token_type;
            }
        }
        // push the last entries in the buffer
        if !blexeme.is_empty() {
            token_vector.push(Token::new(&bttype, &blexeme));
        }

        Ok(token_vector)
    }
}

impl Error for TokenError {}

impl Display for TokenError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        match self {
            TokenError::InvalidCharacter(c) => {
                write!(f, "An Error has occurred: Found invalid character '{}'!", c)
            }
        }
    }
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        if let TokenValue::Number(n) = self {
            return f.pad(&format!("{}", n));
        }

        let value: &str = match self {
            TokenValue::Plus => "Plus",
            TokenValue::Minus => "Minus",
            TokenValue::Multiply => "Multiply",
            TokenValue::Division => "Division",
            TokenValue::Power => "Power",
            TokenValue::OpenParenthesis => "OpenParenthesis",
            TokenValue::CloseParenthesis => "CloseParenthesis",
            TokenValue::OpenBrackets => "OpenBrackets",
            TokenValue::CloseBrackets => "CloseBrackets",
            _ => ""
        };

        f.pad(&format!("{}", value))
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FMTResult {
        write!(f, "{}", match self {
            TokenType::Operator => "Operator",
            TokenType::Number => "Number",
            TokenType::Symbol => "Symbol",
        })
    }
}