// code -> List<Token>

/*
fn sum x y {
    x + y
}
*/

use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    Unreachable,
    InvalidToken,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::Unreachable => "unreachable code. you shouldn't be reading this.",
            Self::InvalidToken => "invalid token"
        };

        write!(f, "{message}")
    }
}

impl std::error::Error for LexerError {}

fn lex(code: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();

    let mut word = String::new();

    for (index, char) in code.chars().enumerate() {
        let token = match char {
            '=' => Some(Token::Equal),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ':' => Some(Token::TypeIndicator),
            ';' => Some(Token::SemiColon),
            _ => None
        };

        if let Some(token) = token {
            dbg!(token.clone());

            if let Some(past_token) = parse_token(word) {
                tokens.push(past_token);
            }

            tokens.push(token);

            word = String::new();
            continue;
        }        

        word.push(char);

        if char == ' ' || code.len() == index + 1 {
            let token = parse_token(word.trim().to_string());

            if let Some(token) = token {
                tokens.push(token);

                word = String::new();
            }
        }
    }

    Ok(tokens)
}

fn parse_token(word: String) -> Option<Token> {
    match word.as_str() {
        "fn" => Some(Token::Fn),
        "let" => Some(Token::Let),
        "string" | "int" | "bool" | "char" => Some(Token::Type(word)),
        _ => {
            if word.is_empty() {
                return None;
            }

            if !word.chars().all(|char| char.is_alphabetic()) {
                return None;
            }

            Some(Token::Identifier(word))
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    TypeIndicator,
    Type(String),
    RightBrace,
    LeftBrace,
    SemiColon,    
    Equal,
    Let,
    Fn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_it_properly() {
        let tokens = vec![
            ("let", vec![Token::Let]),
            (
                "let poggers: bool = true;", 
                vec![
                    Token::Let, 
                    Token::Identifier(String::from("poggers")),
                    Token::TypeIndicator,
                    Token::Type(String::from("bool")),
                    Token::Equal,
                    Token::Identifier(String::from("true")),
                    Token::SemiColon,
            ])
        ];

        for (raw_token, tokens) in tokens {
            let lexed_tokens: Vec<Token> = lex(raw_token).unwrap();

            assert_eq!(lexed_tokens, tokens)
        }
    }
}
