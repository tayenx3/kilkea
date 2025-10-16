use super::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ASTNode {
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    Bool(bool),
    Identifier(String),

    BinOp {
        lhs: Box<Node>,
        rhs: Box<Node>,
        op: (String, Span)
    },
    UnaOp {
        operand: Box<Node>,
        op: (String, Span)
    },
    If {
        condition: Box<Node>,
        then_body: Box<Node>,
        else_body: Box<Node>
    },
    Declaration {
        type_: (ParseType, Option<Span>),
        mutability: bool,
        name: (String, Span)
    },
    DeclarationWithValue {
        type_: (ParseType, Option<Span>),
        mutability: bool,
        name: (String, Span),
        value: Box<Node>
    },
    Block(Vec<Node>),
    Statement(Box<Node>),
    Mutation {
        name: (String, Span),
        value: Box<Node>
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Node {
    pub ast_repr: ASTNode,
    pub span: Span
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Module(pub Vec<Node>);

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Parser {
    pub(crate) pos: usize,
    pub(crate) tokens: Vec<Token>,
    pub(crate) src: String,
    pub(crate) path: String
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>, src: &String, path: &String) -> Self {
        Self {
            pos: 0,
            tokens,
            src: src.clone(),
            path: path.clone()
        }
    }

    
    fn get(&self, offset: i32) -> Option<&Token> {
        let i = self.pos as i32 + offset;

        if i < 0 {
            return None;
        }

        self.tokens.get(i as usize)
    }

    
    pub fn parse_program(&mut self) -> (Module, Vec<Error>) {
        let mut stmts: Vec<Node> = Vec::new();
        let mut errors: Vec<Error> = Vec::new();

        while self.get(0).cloned().is_some() {
            let start_pos = self.pos;
            match self.parse_statement() {
                Ok(n) => stmts.push(n),
                Err(e) => {
                    errors.push(e);
                    if self.pos == start_pos {
                        self.pos += 1;
                    }
                }
            }
        }

        (Module(stmts), errors)
    }

    
    pub fn parse_expression(&mut self, min_bp: i32) -> Result<Node, Error> {
        let span = if let Some(current_token) = self.get(0).cloned() {
            current_token.span
        } else {
            return Err(Error {
                code: ECode::UnexpectedEOF,
                details: String::from("unexpected end of input"),
                span: self.eof(),
                src: self.src.clone(),
                path: self.path.clone(),
                note: None,
                help: None
            })
        };

        let mut l = self.nud()?;

        while let Some(current_token) = self.get(0).cloned() {
            let ((lbp, rbp), op, s) = {
                if let TokenType::ColonEquals = current_token.token_type {
                    (prec(&current_token.lexeme).unwrap(), current_token.lexeme.clone(), current_token.span)
                } else {
                    if let TokenType::Operator = current_token.token_type {
                        (prec(&current_token.lexeme).unwrap(), current_token.lexeme.clone(), current_token.span)
                    } else {
                        break
                    }
                }
            };
            if lbp < min_bp {
                break
            }

            self.pos += 1;
            let r = self.parse_expression(rbp)?;
            let built_span = Span {
                start_pos: span.start_pos,
                end_pos: r.span.end_pos,
                line: span.line,
                column: span.column
            };
            l = Node { 
                ast_repr: ASTNode::BinOp {
                    lhs: Box::new(l),
                    rhs: Box::new(r),
                    op: (op, s)
                },
                span: built_span
            };
        }

        Ok(l)
    }

    
    pub fn nud(&mut self) -> Result<Node, Error> {
        if let Ok(v) = self.try_mutation() {
            self.pos = v.1.pos;
            return Ok(v.0)
        }
        if let Ok(v) = self.try_var_decl() {
            self.pos = v.1.pos;
            return Ok(v.0)
        }
        if let Some(current_token) = self.get(0).cloned() {
            let value = current_token.lexeme.clone();
            match current_token.token_type {
                TokenType::Int => {
                    self.pos += 1;
                    let i = ASTNode::IntLit(value.parse::<i64>().unwrap());
                    Ok( Node{ ast_repr: i, span: current_token.span } )
                },
                TokenType::Float => {
                    self.pos += 1;
                    let i = ASTNode::FloatLit(value.parse::<f64>().unwrap());
                    Ok( Node{ ast_repr: i, span: current_token.span } )
                },
                TokenType::String => {
                    self.pos += 1;
                    let i = ASTNode::StringLit(value.clone());
                    Ok( Node{ ast_repr: i, span: current_token.span } )
                },
                TokenType::Bool => {
                    self.pos += 1;
                    let i = ASTNode::Bool(value.parse::<bool>().unwrap());
                    Ok( Node{ ast_repr: i, span: current_token.span } )
                },
                TokenType::Identifier => {
                    self.pos += 1;
                    let i = ASTNode::Identifier(value.clone());
                    Ok( Node{ ast_repr: i, span: current_token.span } )
                },
                TokenType::LParen => {
                    self.pos += 1;
                    let expr = self.parse_expression(0)?;
                    self.expect(&TokenType::RParen)?;
                    Ok(expr)
                },
                TokenType::Operator => {
                    self.pos += 1;
                    let operand = self.nud()?;
                    let built_span = Span {
                        start_pos: current_token.span.start_pos,
                        end_pos: operand.span.end_pos,
                        line: current_token.span.line,
                        column: current_token.span.column
                    };
                    let un = ASTNode::UnaOp { 
                        operand: Box::new(operand), 
                        op: (current_token.lexeme, current_token.span)
                    };
                    Ok( Node { ast_repr: un, span: built_span } )
                },
                TokenType::Keyword => {
                    match &*value {
                        "if" => {
                            let i = self.parse_if()?;
                            Ok(i)
                        },
                        _ => return Err(Error {
                            code: ECode::UnexpectedToken,
                            details: format!("invalid keyword `{}`", value),
                            span: current_token.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: if value == "else" {
                                Some("add a `if` clause before the `else` clause".to_string())
                            } else { None }
                        })
                    }
                },
                TokenType::LBrace => {
                    let i = self.parse_block()?;
                    Ok(i)
                },
                _ => Err(Error {
                    code: ECode::UnexpectedToken,
                    details: format!("unexpected token {}", current_token),
                    span: self.eof(),
                    src: self.src.clone(),
                    path: self.path.clone(),
                    note: None,
                    help: None
                })
            }
        } else {
            Err(Error {
                code: ECode::UnexpectedEOF,
                details: String::from("unexpected end of input, expected expression"),
                span: self.eof(),
                src: self.src.clone(),
                path: self.path.clone(),
                note: None,
                help: None
            })
        }
    }

    
    fn parse_if(&mut self) -> Result<Node, Error> {        
        self.pos += 1;
        let p_if_span = if let Some(current_token) = self.get(0).cloned() {
            current_token.span
        } else {
            return Err(Error {
                code: ECode::UnexpectedEOF,
                details: String::from("expected expression, unexpected end of input"),
                span: self.eof(),
                src: self.src.clone(),
                path: self.path.clone(),
                note: None,
                help: None
            })
        };

        let condition = self.parse_expression(0)?;
        let then_body = self.parse_expression(0)?;
        let else_body = self.parse_else()?;

        let if_span = Span {
            start_pos: p_if_span.start_pos,
            line: p_if_span.start_pos,
            column: p_if_span.column,
            end_pos: condition.span.end_pos
        };
        
        Ok(Node {
            ast_repr: ASTNode::If {
                condition: Box::new(condition),
                then_body: Box::new(then_body),
                else_body: Box::new(else_body)
            },
            span: if_span
        })
    }

    
    fn parse_else(&mut self) -> Result<Node, Error> {
        if let Some(current_token) = self.get(0) {
            if let TokenType::Keyword = current_token.token_type {
                if current_token.lexeme == "else" {
                    self.pos += 1;
                    
                    if let Some(token) = self.get(0) {
                        if token.token_type == TokenType::LBrace {
                            self.parse_block()
                        } else {
                            let span = match self.get(0).cloned() {
                                Some(o) => o.span.clone(),
                                None => return Err(Error {
                                    code: ECode::UnexpectedEOF,
                                    details: String::from("expected expression, enexpected end of input"),
                                    span: self.eof(),
                                    src: self.src.clone(),
                                    path: self.path.clone(),
                                    note: None,
                                    help: None
                                })
                            };
                            let stmt = self.parse_statement()?;
                            Ok(Node { ast_repr: ASTNode::Block(vec![stmt]), span })
                        }
                    } else {
                        Err(Error {
                            code: ECode::UnexpectedEOF,
                            details: String::from("expected else body after `else`"),
                            span: self.eof(),
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        })
                    }
                } else {
                    Ok(Node { ast_repr: ASTNode::Block(vec![]), span: current_token.span })
                }
            } else {
                Ok(Node { ast_repr: ASTNode::Block(vec![]), span: current_token.span })
            }
        } else {
            Ok(Node { ast_repr: ASTNode::Block(vec![]), span: self.eof() })
        }
    }

    
    fn parse_block(&mut self) -> Result<Node, Error> {
        self.pos += 1;
        let mut block: Vec<Node> = Vec::new();
        let mut span = self.eof();

        while let Some(current_token) = self.get(0) {
            span = current_token.span.clone();
            if current_token.token_type == TokenType::RBrace {
                self.pos += 1;
                break;
            }
            
            match self.parse_statement() {
                Ok(stmt) => block.push(stmt),
                Err(e) => {
                    if let Some(next) = self.get(0) {
                        if next.token_type == TokenType::RBrace {
                            self.pos += 1;
                            break;
                        }
                    }
                    return Err(e);
                }
            }
        }

        Ok(Node { ast_repr: ASTNode::Block(block), span })
    }

    fn try_mutation(&self) -> Result<(Node, Parser), Error> {
        let mut mp = Parser {
            pos: self.pos,
            tokens: self.tokens.clone(),
            src: self.src.clone(),
            path: self.path.clone()
        };
        let t = mp.expect_and_take(&TokenType::Identifier)?;
        let name = (t.lexeme, t.span);
        
        mp.expect(&TokenType::Equals)?;

        let value = mp.parse_expression(0)?;

        let built_span = Span {
            line: name.1.line,
            column: name.1.column,
            start_pos: name.1.start_pos,
            end_pos: value.span.end_pos
        };
        Ok((Node { 
            ast_repr: ASTNode::Mutation {
                name, value: Box::new(value)
            },
            span: built_span
        }, mp))
    }

    fn try_var_decl(&self) -> Result<(Node, Parser), Error> {
        let mut vp = Parser {
            pos: self.pos,
            tokens: self.tokens.clone(),
            src: self.src.clone(),
            path: self.path.clone()
        };

        let (mutability, mut s1) = if let Some(t) = vp.get(0).cloned() {
            if t.lexeme == "mut" && t.token_type == TokenType::Keyword {
                vp.pos += 1;
                (true, t.span)
            } else {
                (false, t.span)
            }
        } else {
            return Err(Error {
                code: ECode::UnexpectedEOF,
                details: String::from("expected expression, unexpected end of input"),
                span: self.eof(),
                src: self.src.clone(),
                path: self.path.clone(),
                note: None,
                help: None
            })
        };

        let t = vp.expect_and_take(&TokenType::Identifier)?;
        let (var_name, n_span) = (t.lexeme, t.span);
        if let Some(Token { token_type: TokenType::Operator, span, .. }) = vp.get(0).cloned() {
            return Err(Error {
                code: ECode::UnexpectedToken, details: "Found operator, halting into expression".to_string(),
                span: span, src: vp.src, path: vp.path, note: Some("You shouldn't be seeing this. If you are, please report it".to_string()), help: None
            })
        }
        let var_type = if let Some(Token { token_type: TokenType::Colon, span, .. }) = vp.get(0).cloned() {
            vp.pos += 1;
            s1.end_pos = span.end_pos;
            (ParseType::Determined(vp.expect_and_take(&TokenType::Identifier)?.lexeme), Some(span))
        } else {
            (ParseType::Inferred, None)
        };
        let value = if let Some(Token { token_type: TokenType::ColonEquals, .. }) = vp.get(0).cloned() {
            vp.pos += 1;
            Some(vp.parse_expression(0)?)
        } else {
            None
        };
        Ok((Node {
            ast_repr: match value {
                Some(v) => ASTNode::DeclarationWithValue {
                    type_: var_type,
                    mutability,
                    name: (var_name, n_span),
                    value: Box::new(v)
                },
                None => ASTNode::Declaration {
                    type_: var_type,
                    mutability,
                    name: (var_name, n_span)
                },
            }, span: s1
        }, vp))
    }

    fn parse_statement(&mut self) -> Result<Node, Error> {
        let mut r = self.parse_expression(0)?;
        if let Some(Token { token_type, span, .. }) = self.get(0) {
            if *token_type == TokenType::Semicolon {
                r = Node { ast_repr: ASTNode::Statement(Box::new(r.clone())), span: Span {
                    line: r.span.line, 
                    column: r.span.column, 
                    start_pos: r.span.start_pos, 
                    end_pos: span.end_pos 
                } };
                self.pos += 1;
            }
        }

        Ok(r)
    }

    
    fn eof(&self) -> Span {
        let mut offset = 0;
        while self.get(offset).is_none() {
            offset -= 1;
        }

        self.get(offset).unwrap().span
    }

    
    fn expect_and_take(&mut self, expected: &TokenType) -> Result<Token, Error> {
        if let Some(current_token) = self.get(0).cloned() {
            if &current_token.token_type == expected {
                self.pos += 1;
                Ok(current_token)
            } else {
                Err(Error {
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
            Err(Error {
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

    
    fn expect(&mut self, expected: &TokenType) -> Result<(), Error> {
        if let Some(current_token) = self.get(0).cloned() {
            if &current_token.token_type == expected {
                self.pos += 1;
                Ok(())
            } else {
                Err(Error {
                    code: ECode::ExpectedToken,
                    details: format!("expected {} found {}", expected.to_error_repr(), current_token),
                    span: current_token.span,
                    src: self.src.clone(),
                    path: self.path.clone(),
                    note: None,
                    help: None
                })
            }
        } else {
            Err(Error {
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
}