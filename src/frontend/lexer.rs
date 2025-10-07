use super::*;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Int,
    Float,
    String,
    Bool,
    Identifier,
    Keyword,
    Equals,
    
    Operator,
    LParen,
    RParen,
    LBrace,
    RBrace,

    Semicolon
}

impl TokenType {
    pub fn to_error_repr(&self) -> String {
        let str = self.to_string();
        match str {
            _ if str.is_empty() => match self {
                TokenType::LParen => "'('".to_string(),
                TokenType::RParen => "')'".to_string(),
                TokenType::LBrace => "'{'".to_string(),
                TokenType::RBrace => "'}'".to_string(),
                TokenType::Semicolon => "';'".to_string(),
                _ => unreachable!()
            },
            _ => str
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "integer "),
            Self::Float => write!(f, "float "),
            Self::String => write!(f, "string "),
            Self::Bool => write!(f, "boolean "),
            Self::Identifier => write!(f, "identifier "),
            Self::Keyword => write!(f, "keyword "),

            Self::Operator => write!(f, "operator "),
            _ => write!(f, "")
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
    pub lexeme: String
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}'{}'", self.token_type, self.lexeme)
    }
}

fn sel_token(lexeme: &String, span: &Span) -> Token {
    Token {
        token_type: match &**lexeme {
            _ if KEYWORDS.contains(&(&**lexeme)) => TokenType::Keyword,
            "+" | "-" | "*" | "/" | "==" | ">" | "<" | ">=" | "<=" | "!=" | "++"
            | "!" => TokenType::Operator,
            "=" => TokenType::Equals,
            "(" => TokenType::LParen,
            ")" => TokenType::RParen,
            "{" => TokenType::LBrace,
            "}" => TokenType::RBrace,
            ";" => TokenType::Semicolon,
            _ => if lexeme.parse::<i64>().is_ok() {
                TokenType::Int
            } else if lexeme.parse::<f64>().is_ok() {
                TokenType::Float
            } else if ["true", "false"].contains(&(&**lexeme)) {
                TokenType::Bool
            } else {
                TokenType::Identifier
            }
        },
        span: span.clone(),
        lexeme: lexeme.clone()
    }
}

fn into_string_token(lexeme: &String, span: &Span) -> Token {
    Token {
        token_type: TokenType::String,
        span: span.clone(),
        lexeme: lexeme.clone()
    }
}


pub fn tokenize(source: &String) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = Vec::new();
    let mut current: String = String::new();

    let mut start_pos: usize = 0;
    let mut line: usize = 0;
    let mut column: usize = 0;

    let mut in_string = false;
    
    let mut i = 0;
    while i < chars.len() {
        if in_string {
            if chars[i] == '"' {
                if !current.is_empty() {
                    let span = Span { 
                        line, 
                        column: column - current.len(), 
                        start_pos: start_pos, 
                        end_pos: i - 1
                    };
                    tokens.push(into_string_token(&current, &span));
                    current.clear();
                }
                in_string = false;
                i += 1;
                column += 1;
            } else {
                current.push(chars[i]);
                i += 1;
                if chars[i] == '\n' {
                    line += 1;
                    column = 0;
                } else {
                    column += 1;
                }
            }
        } else {
            if chars[i] == '"' {
                if !current.is_empty() {
                    let span = Span { 
                        line, 
                        column: column - current.len(), 
                        start_pos: start_pos, 
                        end_pos: i - 1
                    };
                    tokens.push(sel_token(&current, &span));
                    current.clear();
                }
                in_string = true;
                i += 1;
            } else {
                let ch = chars[i];
        
                match ch {
                    _ if " \t".contains(ch) => {
                        if !current.is_empty() {
                            let span = Span { 
                                line, 
                                column: column - current.len(), 
                                start_pos: start_pos, 
                                end_pos: i - 1
                            };
                            tokens.push(sel_token(&current, &span));
                            current.clear();
                        }
                        column += 1;
                        i += 1;
                    }
                    '\n' => {
                        if !current.is_empty() {
                            let span = Span { 
                                line, 
                                column: column - current.len(), 
                                start_pos: start_pos, 
                                end_pos: i - 1
                            };
                            tokens.push(sel_token(&current, &span));
                            current.clear();
                        }
                        line += 1;
                        column = 0;
                        i += 1;
                    },
                    '0'..='9' => {
                        if current.is_empty() {
                            start_pos = i;
                        }
                        current.push(ch);
                        column += 1;
                        i += 1;
                    },
                    'a'..='z' | 'A'..='Z' | '_' => {
                        if !current.is_empty() && (current.parse::<i64>().is_ok() || current.parse::<f64>().is_ok()) {
                            let span = Span { 
                                line, 
                                column: column - current.len(), 
                                start_pos: start_pos, 
                                end_pos: i - 1
                            };
                            tokens.push(sel_token(&current, &span));
                            current.clear();
                            start_pos = i;
                        }
                        
                        current.push(ch);
                        column += 1;
                        i += 1;
                    },
                    _ => {
                        if !current.is_empty() {
                            let span = Span { 
                                line, 
                                column: column - current.len(), 
                                start_pos: start_pos, 
                                end_pos: i - 1
                            };
                            tokens.push(sel_token(&current, &span));
                            current.clear();
                        }
                        
                        if i + 1 < chars.len() {
                            let combined = format!("{}{}", ch, chars[i + 1]);
                            if COMBINED_SYMBOLS.contains(&combined.as_str()) {
                                let span = Span { 
                                    line, 
                                    column, 
                                    start_pos: i, 
                                    end_pos: i + 2 
                                };
                                tokens.push(sel_token(&combined, &span));
                                column += 2;
                                i += 2;
                                continue;
                            }
                        }

                        let span = Span { 
                            line, 
                            column, 
                            start_pos: i, 
                            end_pos: i 
                        };
                        tokens.push(sel_token(&ch.to_string(), &span));
                        column += 1;
                        i += 1;
                    }
                }
            }
        }
    }

    // Don't forget the last token if there is one
    if !current.is_empty() {
        let span = Span { 
            line, 
            column: column - current.len(), 
            start_pos: start_pos, 
            end_pos: chars.len() 
        };
        tokens.push(sel_token(&current, &span));
    }

    tokens
}