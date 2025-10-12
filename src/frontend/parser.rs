use super::*;
use std::fmt;
use colored::Colorize;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub code: ECode,
    pub details: String,
    pub span: Span,
    pub src: String,
    pub path: String,
    pub note: Option<String>,
    pub help: Option<String>
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SPREAD: usize = 2;
        
        let mut output = String::new();
        
        // Header
        output.push_str(&format!("{}", format!("error[{}]:\n", self.code).red().bold()));
        
        // Location line
        let location_info = self.format_location(SPREAD);
        output.push_str(&location_info);
        
        // Error details
        let error_line = self.format_error_line(SPREAD);
        output.push_str(&error_line);

        if let Some(note) = &self.note {
            output.push_str(
                &format!(
                    "\n {:width$} = {}: {}", 
                    "", 
                    "note".cyan(),
                    note, 
                    width = self.calculate_max_digits(
                        self.span.line + SPREAD
                    )
                )
            )
        }
        if let Some(help) = &self.help {
            output.push_str(
                &format!(
                    "\n {:width$} = {}: {}", 
                    "", 
                    "help".cyan(),
                    help, 
                    width = self.calculate_max_digits(
                        self.span.line + SPREAD
                    )
                )
            )
        }
        
        write!(f, "{}", output)
    }
}

impl ParseError {
    fn format_location(&self, spread: usize) -> String {
        let line = self.span.line;
        let digits = self.calculate_max_digits(line + spread);
        
        format!(
            "{:width$}> {}:{}:{}\n",
            "-".repeat(digits + 2),
            self.path,
            line + 1,
            self.span.column,
            width = digits + 2
        )
    }
    
    fn format_error_line(&self, spread: usize) -> String {
        let line = self.span.line;
        let lines: Vec<&str> = self.src.split('\n').collect();
        let digits = self.calculate_max_digits(line + spread);
        
        let mut result = String::new();
        
        // Previous lines
        result.push_str(&self.format_context_lines(line, spread, false, &lines, digits));
        
        // Main line
        result.push_str(&format!(
            " {:width$} {} {}\n",
            (line + 1).to_string().cyan().bold(),
            "|".cyan().bold(),
            lines[line],
            width = digits
        ));
        result.push_str(&format!(
            " {:width$} {} {}{} {}",
            "",
            "|".cyan().bold(),
            " ".repeat(self.span.column),
            "^".repeat(self.span.end_pos + 1 - self.span.start_pos).red().bold(),
            self.details.red().bold(),
            width = digits
        ));
        
        // Next lines
        result.push_str(&self.format_context_lines(line, spread, true, &lines, digits));
        
        result
    }
    
    fn format_context_lines(
        &self,
        current_line: usize,
        spread: usize,
        is_after: bool,
        lines: &[&str],
        digits: usize,
    ) -> String {
        let mut context = String::new();
        
        let range = if is_after {
            (1..=spread).collect::<Vec<_>>()
        } else {
            (1..=spread).rev().collect::<Vec<_>>()
        };
        
        for i in range {
            let target_line = if is_after {
                current_line.checked_add(i)
            } else {
                current_line.checked_sub(i)
            };
            
            if let Some(line_num) = target_line {
                if line_num < lines.len() {
                    let line_content = if is_after {
                        format!("\n {:width$} {} {}", (line_num + 1).to_string().cyan().bold(), "|".cyan().bold(), lines[line_num], width = digits)
                    } else {
                        format!(" {:width$} {} {}\n", (line_num + 1).to_string().cyan().bold(), "|".cyan().bold(), lines[line_num], width = digits)
                    };
                    context.push_str(&line_content);
                }
            }
        }
        
        context
    }
    
    fn calculate_max_digits(&self, max_line_num: usize) -> usize {
        if max_line_num == 0 {
            1
        } else {
            max_line_num.ilog10() as usize + 1
        }
    }
}

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
    Statement(Box<Node>)
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

    
    pub fn parse_program(&mut self) -> (Module, Vec<ParseError>) {
        let mut stmts: Vec<Node> = Vec::new();
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

        (Module(stmts), errors)
    }

    
    pub fn parse_expression(&mut self, min_bp: i32) -> Result<Node, ParseError> {
        let span = if let Some(current_token) = self.get(0).cloned() {
            current_token.span
        } else {
            return Err(ParseError {
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

    
    pub fn nud(&mut self) -> Result<Node, ParseError> {
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
                        _ => return Err(ParseError {
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
                _ => Err(ParseError {
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
            Err(ParseError {
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

    
    fn parse_if(&mut self) -> Result<Node, ParseError> {        
        self.pos += 1;
        let p_if_span = if let Some(current_token) = self.get(0).cloned() {
            current_token.span
        } else {
            return Err(ParseError {
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

    
    fn parse_else(&mut self) -> Result<Node, ParseError> {
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
                                None => return Err(ParseError {
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
                        Err(ParseError {
                            code: ECode::UnexpectedEOF,
                            details: String::from("epected else body after `else`"),
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

    
    fn parse_block(&mut self) -> Result<Node, ParseError> {
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

    fn parse_statement(&mut self) -> Result<Node, ParseError> {
        let mut var_decl_parser = VariableDeclParser::new(&self);
        if let Ok(v) = var_decl_parser.try_parse() {
            self.pos = var_decl_parser.pos;
            if let Some(Token { token_type: TokenType::Semicolon, .. }) = self.get(0).cloned() {
                self.pos += 1;
                return Ok(Node { ast_repr: ASTNode::Statement(Box::new(v.clone())), span: v.span })
            } else if let Some(Token { token_type: TokenType::Equals, .. }) = self.get(0).cloned() {
                self.pos += 1;
                let value = self.parse_expression(0)?;
                let v_ast = v.clone().ast_repr;
                if let ASTNode::Declaration { name, type_, mutability } = v_ast {
                    let r = Node {
                        ast_repr: ASTNode::DeclarationWithValue {
                            type_,
                            mutability,
                            name,
                            value: Box::new(value)
                        },
                        span: v.span
                    };
                    if let Some(Token { token_type: TokenType::Semicolon, .. }) = self.get(0).cloned() {
                        self.pos += 1;
                        return Ok(Node { ast_repr: ASTNode::Statement(Box::new(r.clone())), span: r.span })
                    } else {
                        return Ok(r)
                    }
                }
            } else if let Some(Token { token_type: TokenType::Operator, .. }) = self.get(0).cloned() {
                self.pos -= 1;
                let mut r = self.parse_expression(0)?;
                if let Some(Token { token_type, .. }) = self.get(0) {
                    if *token_type == TokenType::Semicolon {
                        r = Node { ast_repr: ASTNode::Statement(Box::new(r.clone())), span: r.span };
                        self.pos += 1;
                    }
                }
                return Ok(r)
            } else if let Some(Token { token_type: TokenType::ColonEquals, .. }) = self.get(0).cloned() {
                self.pos -= 1;
                let mut r = self.parse_expression(0)?;
                if let Some(Token { token_type, .. }) = self.get(0) {
                    if *token_type == TokenType::Semicolon {
                        r = Node { ast_repr: ASTNode::Statement(Box::new(r.clone())), span: r.span };
                        self.pos += 1;
                    }
                }
                return Ok(r)
            }
            return Ok(v)
        }
        let mut r = self.parse_expression(0)?;
        if let Some(Token { token_type, .. }) = self.get(0) {
            if *token_type == TokenType::Semicolon {
                r = Node { ast_repr: ASTNode::Statement(Box::new(r.clone())), span: r.span };
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

    
    fn expect(&mut self, expected: &TokenType) -> Result<(), ParseError> {
        if let Some(current_token) = self.get(0).cloned() {
            if &current_token.token_type == expected {
                self.pos += 1;
                Ok(())
            } else {
                Err(ParseError {
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
}