use std::fmt;
use colored::Colorize;
use strsim::jaro_winkler;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
    pub code: ECode,
    pub details: String,
    pub span: Span,
    pub src: String,
    pub path: String,
    pub note: Option<String>,
    pub help: Option<String>
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SPREAD: usize = 2;
        
        let mut output = String::new();
        
        // Header
        output.push_str(&format!("{}", format!("error[{}]\n", self.code).red().bold()));
        
        // Location line
        let location_info = self.format_location(SPREAD);
        output.push_str(&location_info);
        
        // Error details
        let error_line = self.format_error_line(SPREAD);
        output.push_str(&error_line);

        if let Some(note) = &self.note {
            output.push_str(
                &format!(
                    " {:width$} = note: {}\n", 
                    "", 
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
                    " {:width$} = help: {}\n", 
                    "", 
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

impl TypeError {
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
            " {:width$} | {}\n",
            line + 1,
            lines[line],
            width = digits
        ));
        result.push_str(&format!(
            " {:width$} | {}{}",
            "",
            " ".repeat(self.span.column),
            "^".repeat(self.span.end_pos + 1 - self.span.start_pos).red().bold(),
            width = digits
        ));
        
        // Next lines
        result.push_str(&self.format_context_lines(line, spread, true, &lines, digits));
        
        // Error indicator
        result.push_str(&format!(
            "\n {:width$} | {}",
            ">".repeat(digits).red().bold(),
            self.details.red().bold(),
            width = digits
        ));
        
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
                        format!("\n {:width$} | {}", line_num + 1, lines[line_num], width = digits)
                    } else {
                        format!(" {:width$} | {}\n", line_num + 1, lines[line_num], width = digits)
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

pub struct TypeChecker {
    module: Module,
    scopes: Vec<Vec<Symbol>>,
    src: String,
    path: String,
    type_registry: TypeRegistry
}

impl TypeChecker {
    pub fn new(module: Module, src: String, path: String) -> Self {
        Self {
            module,
            scopes: vec![vec![]],
            src,
            path,
            type_registry: TypeRegistry::new()
        }
    }

    #[allow(irrefutable_let_patterns)]
    pub fn find_identifier(&self, i: &String, span: Span) -> Result<Type, TypeError> {
        let mut reversed_scopes = self.scopes.clone();
        reversed_scopes.reverse();

        let mut available_names: Vec<String> = Vec::new();
        let mut top_contender: (Option<String>, f64) = (None, 0.0);

        for scope in reversed_scopes {
            for symbol in scope {
                if let Symbol::Variable { name, type_, mutability: _ } = symbol {
                    available_names.push(name.clone());
                    if &name == i { return Ok(type_) }
                }
            }
        }

        for name in available_names {
            let score = jaro_winkler(i, &name);
            if score >= 0.7 && score > top_contender.1 {
                top_contender = (Some(name.clone()), score)
            }
        }

        Err(TypeError { 
            code: ECode::UndefinedIdentifier, 
            details: format!("cannot find `{}` in scope", i), 
            span, 
            src: self.src.clone(), 
            path: self.path.clone(),
            note: None,
            help: if let Some(name) = top_contender.0 {
                Some(format!("did you mean: `{}`?", name))
            } else {
                None
            }
        })
    }

    pub fn check(&mut self) -> Vec<TypeError> {
        let mut errors: Vec<TypeError> = Vec::new();
        for node in self.module.0.clone() {
            let result = self.check_node(node.clone());
            if let Err(s) = result{
                errors.push(s)
            }
        }
        errors
    }

    pub fn check_node(&mut self, node: Node) -> Result<Type, TypeError> {
        match node.ast_repr {
            ASTNode::IntLit(_) => Ok(Type::Int32),
            ASTNode::FloatLit(_) => Ok(Type::Float64),
            ASTNode::StringLit(_) => Ok(Type::String),
            ASTNode::Bool(_) => Ok(Type::Boolean),
            ASTNode::Identifier(s) => self.find_identifier(&s, node.span),
            ASTNode::BinOp {
                op, lhs, rhs
            } => {
                let left = self.check_node(*lhs)?;
                let right = self.check_node(*rhs)?;
                match &*op.0 {
                    "+" | "-" | "*" | "/" | ">" | "<" | ">=" | "<=" => match (left.clone(), right.clone()) {
                        (Type::Int8, Type::Int8) => Ok(Type::Int8),
                        (Type::Int16, Type::Int16) => Ok(Type::Int16),
                        (Type::Int32, Type::Int32) => Ok(Type::Int32),
                        (Type::Int64, Type::Int64) => Ok(Type::Int64),
                        (Type::Float32, Type::Float32) => Ok(Type::Float32),
                        (Type::Float64, Type::Float64) => Ok(Type::Float64),
                        (Type::UInt8, Type::UInt8) => Ok(Type::UInt8),
                        (Type::UInt16, Type::UInt16) => Ok(Type::UInt16),
                        (Type::UInt32, Type::UInt32) => Ok(Type::UInt32),
                        (Type::UInt64, Type::UInt64) => Ok(Type::UInt64),
                        _ => Err(TypeError {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot do `{}` operation on types `{}`, `{}`", op.0, left, right),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        })
                    },
                    "==" | "!=" => if left == right { Ok(left) } else {
                        Err(TypeError {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot do `{}` operation on types `{}`, `{}`", op.0, left, right),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        })
                    },
                    "++" => match (left.clone(), right.clone()) {
                        (Type::String, Type::String) => Ok(Type::String),
                        _ => Err(TypeError {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot do `{}` operation on types `{}`, `{}`", op.0, left, right),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        })
                    },
                    _ => Err(TypeError {
                        code: ECode::MismatchedTypes,
                        details: format!("invalid operator - `{}`", op.0),
                        span: op.1,
                        src: self.src.clone(),
                        path: self.path.clone(),
                        note: None,
                        help: None
                    })
                }
            },
            ASTNode::UnaOp {
                operand, op
            } => {
                let operand_type = self.check_node(*operand)?;
                match &*op.0 {
                    "+" => match operand_type {
                        Type::Int8 => Ok(Type::Int8),
                        Type::Int16 => Ok(Type::Int16),
                        Type::Int32 => Ok(Type::Int32),
                        Type::Int64 => Ok(Type::Int64),
                        Type::Float32 => Ok(Type::Float32),
                        Type::Float64 => Ok(Type::Float64),
                        Type::UInt8 => Ok(Type::UInt8),
                        Type::UInt16 => Ok(Type::UInt16),
                        Type::UInt32 => Ok(Type::UInt32),
                        Type::UInt64 => Ok(Type::UInt64),
                        _ => Err(TypeError {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot do apply `{}` to type `{}`", op.0, operand_type),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        })
                    },
                    "-" => match operand_type {
                        Type::Int8 => Ok(Type::Int8),
                        Type::Int16 => Ok(Type::Int16),
                        Type::Int32 => Ok(Type::Int32),
                        Type::Int64 => Ok(Type::Int64),
                        Type::Float32 => Ok(Type::Float32),
                        Type::Float64 => Ok(Type::Float64),
                        Type::UInt8 | Type::UInt16 
                        | Type::UInt32 | Type::UInt64 => Err(TypeError {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot negate type `{}`", operand_type),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        }),
                        _ => Err(TypeError {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot negate type `{}`", operand_type),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        })
                    },
                    "!" => match operand_type {
                        Type::Boolean => Ok(Type::Boolean),
                        Type::Int8 => Ok(Type::Int8),
                        Type::Int16 => Ok(Type::Int16),
                        Type::Int32 => Ok(Type::Int32),
                        Type::Int64 => Ok(Type::Int64),
                        Type::UInt8 => Ok(Type::UInt8),
                        Type::UInt16 => Ok(Type::UInt16),
                        Type::UInt32 => Ok(Type::UInt32),
                        Type::UInt64 => Ok(Type::UInt64),
                        _ => Err(TypeError {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot do apply `{}` to type `{}`", op.0, operand_type),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: Some("the `!` operator can be applied to `bool` and integer types".to_string()),
                            help: None
                        })
                    },
                    _ => Err(TypeError {
                        code: ECode::MismatchedTypes,
                        details: format!("invalid operator - `{}`", op.0),
                        span: op.1,
                        src: self.src.clone(),
                        path: self.path.clone(),
                        note: None,
                        help: None
                    })
                }
            },
            ASTNode::If {
                condition, then_body, else_body
            } => {
                let then_type = self.check_node(*then_body.clone())?;
                let else_type = self.check_node(*else_body.clone())?;
                let condition_type = self.check_node(*condition)?;

                if let Type::Boolean = condition_type {
                    if let ASTNode::Block(v) = (*then_body).ast_repr {
                        if v.is_empty() {
                            return Ok(else_type)
                        }
                    }
                    if let ASTNode::Block(v) = (*else_body).ast_repr {
                        if v.is_empty() {
                            return Ok(then_type)
                        }
                    }
                    if then_type != else_type {
                        return Err(TypeError {
                            code: ECode::MismatchedTypes,
                            details: format!("`then` and `else` bodies have mismatched types: `{}`, `{}`", then_type, else_type),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        })
                    } else {
                        Ok(then_type)
                    }
                } else {
                    Err(TypeError {
                        code: ECode::MismatchedTypes,
                        details: format!("expected `bool`, found `{}`", condition_type),
                        span: node.span,
                        src: self.src.clone(),
                        path: self.path.clone(),
                        note: None,
                        help: None
                    })
                }
            },
            ASTNode::Declaration {
                type_, mutability, name
            } => {
                let scope = self.scopes.last_mut();
                if let Some(s) = scope {
                    if let ParseType::Determined(t) = type_.0 {
                        if self.type_registry.is_registered(&name.0) {
                            return Err(TypeError {
                                code: ECode::MismatchedTypes,
                                details: format!("identifier `{}` is already registed as a type", name.0),
                                span: name.1,
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        }
                        let t = if let Some(type_) = self.type_registry.get(&t) {
                            type_
                        } else {
                            return Err(TypeError {
                                code: ECode::MismatchedTypes,
                                details: format!("unregistered type - `{}`", t),
                                span: type_.1.unwrap(),
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        };
                        s.push(Symbol::Variable {
                            name: name.0, type_: t, mutability
                        })
                    } else {
                        if self.type_registry.is_registered(&name.0) {
                            return Err(TypeError {
                                code: ECode::MismatchedTypes,
                                details: format!("identifier `{}` is already registed as a type", name.0),
                                span: name.1,
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        }
                        s.push(Symbol::Variable {
                            name: name.0, type_: Type::Undetermined, mutability
                        })
                    }
                }

                Ok(Type::Unit)
            },
            ASTNode::DeclarationWithValue {
                type_, mutability, name, value: _
            } => {
                let scope = self.scopes.last_mut();
                if let Some(s) = scope {
                    if let ParseType::Determined(t) = type_.0 {
                        if self.type_registry.is_registered(&name.0) {
                            return Err(TypeError {
                                code: ECode::MismatchedTypes,
                                details: format!("identifier `{}` is already registed as a type", name.0),
                                span: name.1,
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        }
                        let t = if let Some(type_) = self.type_registry.get(&t) {
                            type_
                        } else {
                            return Err(TypeError {
                                code: ECode::MismatchedTypes,
                                details: format!("unregistered type - `{}`", t),
                                span: type_.1.unwrap(),
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        };
                        s.push(Symbol::Variable {
                            name: name.0, type_: t, mutability
                        })
                    } else {
                        if self.type_registry.is_registered(&name.0) {
                            return Err(TypeError {
                                code: ECode::MismatchedTypes,
                                details: format!("identifier `{}` is already registed as a type", name.0),
                                span: name.1,
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        }
                        s.push(Symbol::Variable {
                            name: name.0, type_: Type::Undetermined, mutability
                        })
                    }
                }

                Ok(Type::Unit)
            },
            ASTNode::Block(stmts) => {
                let mut type_ = Type::Unit;
                for node in stmts {
                    type_ = self.check_node(node)?;
                }
                Ok(type_)
            },
            ASTNode::Statement(s) => {
                self.check_node(*s)?;
                Ok(Type::Unit)
            }
        }
    }
}