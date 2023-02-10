use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidToken,
    StartedWithIdentifier
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::InvalidToken => "invalid token",
            Self::StartedWithIdentifier => "cannot start with an identifier"
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
            if let Some(past_token) = parse_token(word) {
                tokens.push(past_token);
            }

            tokens.push(token);

            word = String::new();
            continue;
        }        

        if char != ' ' {
            word.push(char);
        }

        if char == ' ' || code.len() == index + 1 {
            let token = parse_token(word.clone());

            if let Some(token) = token {
                if let Token::Identifier(_) = token {
                    if tokens.is_empty() { return Err(LexerError::InvalidToken); }
                }   

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
        "String" | "Int" | "Bool" | "Char" => Some(Token::Type(word)),
        _ => {
            if word.is_empty() {
                return None;
            }

            if !word.chars().all(|char| char.is_alphanumeric()) {
                return None;
            }

            Some(Token::Identifier(word))
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Type(String),
    TypeIndicator,
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
    fn it_lex_it_properly() {
        let tokens = vec![
            ("let poggers: Int = 15;", vec![
                Token::Let, 
                Token::Identifier(String::from("poggers")),
                Token::TypeIndicator,
                Token::Type(String::from("Int")),
                Token::Equal,
                Token::Identifier(String::from("15")),
                Token::SemiColon,
            ]),
            ("fn fn", vec![
                Token::Fn,
                Token::Fn,
            ])
        ];

        for (raw_token, tokens) in tokens {
            let lexed_tokens: Vec<Token> = lex(raw_token).unwrap();

            assert_eq!(lexed_tokens, tokens)
        }
    }

    #[test]
    fn it_stops_when_starts_with_identifier() {
        let invalid_tokens = vec![
            "poggers omegalul pogchamp kekw",
            "123pogg"
        ];

        for token in invalid_tokens {
            assert_eq!(Err(LexerError::InvalidToken), lex(token));
        }
    }
}
