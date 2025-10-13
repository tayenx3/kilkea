use super::*;

#[allow(dead_code)]
pub trait ParserPlugin {
    fn try_parse(&mut self) -> Result<Node, ParseError>;
    fn get(&self, offset: i32) -> Option<&Token>;
    fn expect(&mut self, expected: &TokenType) -> Result<(), ParseError>;
    fn expect_and_take(&mut self, expected: &TokenType) -> Result<Token, ParseError>;
    fn eof(&self) -> Span;
}

pub struct VariableDeclParser {
    pub(crate) pos: usize,
    pub(crate) tokens: Vec<Token>,
    pub(crate) src: String,
    pub(crate) path: String
}

impl VariableDeclParser {
    pub fn new(parser: &Parser) -> Self {
        Self {
            pos: parser.pos,
            tokens: parser.tokens.clone(),
            src: parser.src.clone(),
            path: parser.path.clone(),
        }
    }
}


impl ParserPlugin for VariableDeclParser {
    fn try_parse(&mut self) -> Result<Node, ParseError> {        
        if let Some(token) = self.get(0).cloned() {
            let mutability = if token.lexeme == "mut" && token.token_type == TokenType::Keyword {
                self.pos += 1;
                true
            } else {
                false
            };
            let name = self.expect_and_take(&TokenType::Identifier)?;
            if let Some(Token { token_type: TokenType::Colon, .. }) = self.get(0).cloned() {
                self.pos += 1;
                let vtype = self.expect_and_take(&TokenType::Identifier)?;
                Ok(Node {
                    ast_repr: ASTNode::Declaration {
                        type_: (ParseType::Determined(vtype.lexeme), Some(vtype.span)),
                        mutability,
                        name: (name.lexeme, name.span)
                    },
                    span: token.span
                })
            } else {
                Ok(Node {
                    ast_repr: ASTNode::Declaration {
                        type_: (ParseType::Inferred, None),
                        mutability,
                        name: (name.lexeme, name.span)
                    },
                    span: token.span
                })
            }
        } else {
            Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: format!("unexpected end of input, expected expression"),
                span: self.eof(),
                src: self.src.clone(),
                path: self.path.clone(),
                note: None,
                help: None
            })
        }
    }

    fn get(&self, offset: i32) -> Option<&Token> {
        let i = self.pos as i32 + offset;

        if i < 0 {
            return None;
        }

        self.tokens.get(i as usize)
    }

    fn expect(&mut self, expected: &TokenType) -> Result<(), ParseError> {
        if let Some(current_token) = self.get(0).cloned() {
            if &current_token.token_type == expected {
                self.pos += 1;
                Ok(())
            } else {
                Err(ParseError {
                    code: ECode::ExpectedToken,
                    details: format!("expected {}, found {}", expected.to_error_repr(), current_token),
                    span: current_token.span,
                    src: self.src.clone(),
                    path: self.path.clone(),
                    note: None,
                    help: None
                })
            }
        } else {
            Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: format!("unexpected end of input, expected {}", expected.to_error_repr()),
                span: self.eof(),
                src: self.src.clone(),
                path: self.path.clone(),
                note: None,
                help: None
            })
        }
    }

    fn expect_and_take(&mut self, expected: &TokenType) -> Result<Token, ParseError> {
        if let Some(current_token) = self.get(0).cloned() {
            if &current_token.token_type == expected {
                self.pos += 1;
                Ok(current_token)
            } else {
                Err(ParseError {
                    code: ECode::ExpectedToken,
                    details: format!("expected {}, found {}", expected.to_error_repr(), current_token),
                    span: current_token.span,
                    src: self.src.clone(),
                    path: self.path.clone(),
                    note: None,
                    help: None
                })
            }
        } else {
            Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: format!("unexpected end of input, expected {}", expected.to_error_repr()),
                span: self.eof(),
                src: self.src.clone(),
                path: self.path.clone(),
                note: None,
                help: None
            })
        }
    }

    fn eof(&self) -> Span {
        let mut offset = -1;
        while self.get(offset).is_none() {
            offset -= 1;
        }

        self.get(offset).unwrap().span
    }
}

























