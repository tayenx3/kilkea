use super::*;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParseError {
    pub code: ECode,
    pub details: String,
    pub span: Span,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error[{}]: {}", self.code, self.details)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ASTNode {
    Module(Vec<ASTNode>),
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    Bool(bool),
    Identifier(String),

    BinOp {
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>,
        op: String
    },
    UnaOp {
        operand: Box<ASTNode>,
        op: String
    },
    If {
        condition: Box<ASTNode>,
        then_body: Box<ASTNode>,
        else_body: Box<ASTNode>
    },
    Block(Vec<ASTNode>),
    Statement(Box<ASTNode>)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
    scopes: Vec<Scope>
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            pos: 0,
            tokens,
            scopes: vec![ Scope::new() ]
        }
    }

    
    fn get(&self, offset: i32) -> Option<&Token> {
        let i = self.pos as i32 + offset;

        if i < 0 {
            return None;
        }

        self.tokens.get(i as usize)
    }

    
    pub fn parse_program(&mut self) -> (ASTNode, Vec<ParseError>) {
        let mut stmts: Vec<ASTNode> = Vec::new();
        let mut errors: Vec<ParseError> = Vec::new();

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

        (ASTNode::Module(stmts), errors)
    }

    
    pub fn parse_expression(&mut self, min_bp: i32) -> Result<ASTNode, ParseError> {
        let span = if let Some(current_token) = self.get(0).cloned() {
            current_token.span
        } else {
            return Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: String::from("Unexpected end of input"),
                span: self.eof()
            })
        };

        let mut lhs: ASTNode = self.nud()?;
        self.full_check(&lhs, span)?;

        while let Some(current_token) = self.get(0).cloned() {
            let span = current_token.span;

            let ((lbp, rbp), op) = {
                if let TokenType::Operator = current_token.token_type {
                    (prec(&current_token.lexeme).unwrap(), current_token.lexeme.clone())
                } else {
                    break
                }
            };
            if lbp < min_bp {
                break
            }

            self.pos += 1;
            let rhs = self.parse_expression(rbp)?;
            lhs = ASTNode::BinOp {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op: op
            };
            self.full_check(&lhs, span)?;
        }

        Ok(lhs)
    }

    
    pub fn nud(&mut self) -> Result<ASTNode, ParseError> {
        if let Some(current_token) = self.get(0).cloned() {
            let value = current_token.lexeme.clone();
            match current_token.token_type {
                TokenType::Int => {
                    self.pos += 1;
                    let i = ASTNode::IntLit(value.parse::<i64>().unwrap());
                    self.full_check(&i, current_token.span)?;
                    Ok(i)
                },
                TokenType::Float => {
                    self.pos += 1;
                    let i = ASTNode::FloatLit(value.parse::<f64>().unwrap());
                    self.full_check(&i, current_token.span)?;
                    Ok(i)
                },
                TokenType::String => {
                    self.pos += 1;
                    let i = ASTNode::StringLit(value.clone());
                    self.full_check(&i, current_token.span)?;
                    Ok(i)
                },
                TokenType::Bool => {
                    self.pos += 1;
                    let i = ASTNode::Bool(value.parse::<bool>().unwrap());
                    self.full_check(&i, current_token.span)?;
                    Ok(i)
                },
                TokenType::Identifier => {
                    self.pos += 1;
                    let i = ASTNode::Identifier(value.clone());
                    self.full_check(&i, current_token.span)?;
                    Ok(i)
                },
                TokenType::LParen => {
                    self.pos += 1;
                    let expr = self.parse_expression(0)?;
                    self.expect(&TokenType::RParen)?;
                    self.full_check(&expr, current_token.span)?;
                    Ok(expr)
                },
                TokenType::Operator => {
                    self.pos += 1;
                    let operand = self.nud()?;
                    let un = ASTNode::UnaOp { 
                        operand: Box::new(operand), 
                        op: current_token.lexeme
                    };
                    self.full_check(&un, current_token.span)?;
                    Ok(un)
                },
                TokenType::Keyword => {
                    match &*value {
                        "if" => {
                            let i = self.parse_if()?;
                            self.full_check(&i, current_token.span)?;
                            Ok(i)
                        },
                        "let" => Ok(ASTNode::IntLit(0)), // Variable declarations later
                        _ => return Err(ParseError {
                            code: ECode::UnexpectedToken,
                            details: format!("Invalid keyword: '{}'", value),
                            span: current_token.span
                        })
                    }
                },
                TokenType::LBrace => {
                    let i = self.parse_block()?;
                    self.full_check(&i, current_token.span)?;
                    Ok(i)
                },
                _ => todo!("{}", current_token)
            }
        } else {
            Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: String::from("Unexpected end of input"),
                span: self.eof()
            })
        }
    }

    
    fn parse_if(&mut self) -> Result<ASTNode, ParseError> {
        let if_span = if let Some(current_token) = self.get(0).cloned() {
            current_token.span
        } else {
            return Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: String::from("Unexpected end of input"),
                span: self.eof()
            })
        };
        
        self.pos += 1;

        let condition = self.parse_expression(0)?;
        
        match self.full_check(&condition, if_span)? {
            CompileValue::Boolean => {
                let then_body = if let Some(token) = self.get(0) {
                    if token.token_type == TokenType::LBrace {
                        self.parse_block()?
                    } else {
                        let stmt = self.parse_statement()?;
                        ASTNode::Block(vec![stmt])
                    }
                } else {
                    return Err(ParseError {
                        code: ECode::UnexpectedEOF,
                        details: String::from("Expected then body after if condition"),
                        span: self.eof()
                    })
                };

                let else_body = self.parse_else()?;

                Ok(ASTNode::If {
                    condition: Box::new(condition),
                    then_body: Box::new(then_body),
                    else_body: Box::new(else_body)
                })
            },
            other => Err(ParseError {
                code: ECode::MismatchedTypes,
                details: format!("If condition must be boolean, found {}", other.to_type()),
                span: if_span
            })
        }
    }

    
    fn parse_else(&mut self) -> Result<ASTNode, ParseError> {
        if let Some(current_token) = self.get(0) {
            if let TokenType::Keyword = current_token.token_type {
                if current_token.lexeme == "else" {
                    self.pos += 1;
                    
                    if let Some(token) = self.get(0) {
                        if token.token_type == TokenType::LBrace {
                            self.parse_block()
                        } else {
                            let stmt = self.parse_statement()?;
                            Ok(ASTNode::Block(vec![stmt]))
                        }
                    } else {
                        Err(ParseError {
                            code: ECode::UnexpectedEOF,
                            details: String::from("Expected else body after 'else'"),
                            span: self.eof()
                        })
                    }
                } else {
                    Ok(ASTNode::Block(vec![]))
                }
            } else {
                Ok(ASTNode::Block(vec![]))
            }
        } else {
            Ok(ASTNode::Block(vec![]))
        }
    }

    
    fn parse_block(&mut self) -> Result<ASTNode, ParseError> {
        self.pos += 1;
        let mut block: Vec<ASTNode> = Vec::new();

        while let Some(current_token) = self.get(0) {
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

        Ok(ASTNode::Block(block))
    }

    fn parse_statement(&mut self) -> Result<ASTNode, ParseError> {
        let span = if let Some(current_token) = self.get(0).cloned() {
            current_token.span
        } else {
            return Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: String::from("Unexpected end of input"),
                span: self.eof()
            })
        };
        let mut r = self.parse_expression(0)?;
        if let Some(Token { token_type, .. }) = self.get(0) {
            if *token_type == TokenType::Semicolon {
                r = ASTNode::Statement(Box::new(r));
                self.pos += 1;
            }
        }
        self.full_check(&r, span)?;

        Ok(r)
    }

    
    fn eof(&self) -> Span {
        let mut offset = -1;
        while self.get(offset).is_none() {
            offset -= 1;
        }

        self.get(offset).unwrap().span
    }

    
    fn expect_and_take(&mut self, expected: &TokenType) -> Result<Token, ParseError> {
        if let Some(current_token) = self.get(0).cloned() {
            if &current_token.token_type == expected {
                self.pos += 1;
                Ok(current_token)
            } else {
                Err(ParseError {
                    code: ECode::ExpectedToken,
                    details: format!("Expected {} found {}", expected.to_error_repr(), current_token),
                    span: current_token.span
                })
            }
        } else {
            Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: format!("Unexpected end of input, expected {}", expected.to_error_repr()),
                span: self.eof()
            })
        }
    }

    
    fn expect(&mut self, expected: &TokenType) -> Result<(), ParseError> {
        if let Some(current_token) = self.get(0).cloned() {
            if &current_token.token_type == expected {
                self.pos += 1;
                Ok(())
            } else {
                Err(ParseError {
                    code: ECode::ExpectedToken,
                    details: format!("Expected {} found {}", expected.to_error_repr(), current_token),
                    span: current_token.span
                })
            }
        } else {
            Err(ParseError {
                code: ECode::UnexpectedEOF,
                details: format!("Unexpected end of input, expected {}", expected.to_error_repr()),
                span: self.eof()
            })
        }
    }

    fn full_check(&self, node: &ASTNode, span: Span) -> Result<CompileValue, ParseError> {
        match node {
            ASTNode::Module(stmts) => {
                for stmt in stmts {
                    self.full_check(stmt, span)?;
                }
                Ok(CompileValue::Unit)
            },
            ASTNode::IntLit(_) | ASTNode::FloatLit(_) 
            | ASTNode::StringLit(_) | ASTNode::Bool(_) 
            | ASTNode::Identifier(_) => self.to_compiled_value(node, span),
            ASTNode::Statement(s) => {
                self.full_check(&**s, span)?; // discarded but check anyway
                Ok(CompileValue::Unit)
            }
            ASTNode::BinOp { 
                lhs, rhs, op 
            } => {
                let left = self.full_check(&**lhs, span)?;
                let right = self.full_check(&**rhs, span)?;

                let result = match &**op {
                    "+" | "-" | "*" | "/" => match (&left, &right) {
                        (CompileValue::Integer, CompileValue::Integer) => CompileValue::Integer,
                        (CompileValue::Float, CompileValue::Float) => CompileValue::Float,
                        _ => return Err(ParseError {
                            code: ECode::MismatchedTypes,
                            details: format!("Mismatched types: {} and {}", left.to_type(), right.to_type()),
                            span
                        })
                    },
                    "==" | "!=" | ">" | "<" | ">=" | "<=" => if left == right {
                        CompileValue::Boolean
                    } else {
                        return Err(ParseError {
                            code: ECode::MismatchedTypes,
                            details: format!("Mismatched types: {} and {}", left.to_type(), right.to_type()),
                            span
                        })
                    },
                    "++" => if let (CompileValue::String, CompileValue::String) = (&left, &right) {
                        CompileValue::String
                    } else {
                        return Err(ParseError {
                            code: ECode::MismatchedTypes,
                            details: format!("Cannot do '++' operation on types: {} and {}", left.to_type(), right.to_type()),
                            span
                        })
                    },
                    _ => return Err(ParseError {
                        code: ECode::MismatchedTypes,
                        details: format!("Invalid operator: {}", op),
                        span
                    })
                };

                Ok(result)
            },
            ASTNode::UnaOp {
                operand, op
            } => {
                let operand = self.full_check(&**operand, span)?;
                match &**op {
                    "+" | "-" => match operand {
                        CompileValue::Integer | CompileValue::Float => Ok(operand),
                        _ => return Err(ParseError {
                            code: ECode::MismatchedTypes,
                            details: format!("Cannot do '{}' unary operation on type: {} ", op, operand.to_type()),
                            span
                        })
                    },
                    "!" => match operand {
                        CompileValue::Boolean => Ok(operand),
                        _ => return Err(ParseError {
                            code: ECode::MismatchedTypes,
                            details: format!("Cannot do '!' unary operation on type: {} ", operand.to_type()),
                            span
                        })
                    },
                    _ => return Err(ParseError {
                        code: ECode::MismatchedTypes,
                        details: format!("Invalid unary operator: {}", op),
                        span
                    })
                }
            }
            ASTNode::If {
                condition: _, then_body, else_body
            } => {
                // the condition's type is already checked in parse_if()
                let then_result = self.full_check(&**then_body, span)?;
                let else_result = self.full_check(&**else_body, span)?;
                if !(then_result == else_result) {
                    return Err(ParseError {
                        code: ECode::MismatchedTypes,
                        details: format!("Mismatched types: {} and {}", then_result.to_type(), else_result.to_type()),
                        span
                    })
                }
                Ok(then_result)
            },
            ASTNode::Block(stmts) => {
                let mut r: CompileValue = CompileValue::Unit;
                for stmt in stmts {
                    r = self.full_check(stmt, span)?;
                }
                Ok(r)
            },
        }
    }

    fn to_compiled_value(&self, node: &ASTNode, span: Span) -> Result<CompileValue, ParseError> {
        match node {
            ASTNode::IntLit(_) => Ok(CompileValue::Integer),
            ASTNode::FloatLit(_) => Ok(CompileValue::Float),
            ASTNode::StringLit(_) => Ok(CompileValue::String),
            ASTNode::Bool(_) => Ok(CompileValue::Boolean),
            ASTNode::Identifier(i) => if let Some(value) = self.find_in_scopes(i) {
                Ok(value)
            } else {
                Err(ParseError {
                    code: ECode::UndefinedIdentifier,
                    details: format!("Undefined identifier: {}", i),
                    span
                })
            },
            _ => Ok(CompileValue::Unit)
        }
    }

    fn find_in_scopes(&self, s: &String) -> Option<CompileValue> {
        let mut reversed = self.scopes.clone();
        reversed.reverse();

        for scope in reversed {
            match scope.find(s) {
                Some(s) => return Some(s),
                None => continue
            }
        }

        None
    }
}