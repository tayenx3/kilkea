use std::collections::HashMap;
use strsim::jaro_winkler;
use super::*;
use super::Error;

pub struct TypeChecker {
    module: Module,
    scopes: Vec<HashMap<Symbol, ()>>,
    src: String,
    path: String,
    type_registry: TypeRegistry
}

impl TypeChecker {
    pub fn new(module: Module, src: String, path: String) -> Self {
        Self {
            module,
            scopes: vec![HashMap::new()],
            src,
            path,
            type_registry: TypeRegistry::new()
        }
    }

    #[allow(irrefutable_let_patterns)]
    pub fn find_identifier(&self, i: &String, span: Span) -> Result<Type, Error> {
        let mut reversed_scopes = self.scopes.clone();
        reversed_scopes.reverse();

        let mut available_names: Vec<String> = Vec::new();
        let mut top_contender: (Option<String>, f64) = (None, 0.0);

        for scope in reversed_scopes {
            for symbol in scope {
                if let Symbol::Variable { name, type_, mutability: _ } = symbol.0 {
                    available_names.push(name.clone());
                    if name == *i { return Ok(type_) }
                }
            }
        }

        for name in available_names {
            let score = jaro_winkler(i, &name);
            if score >= 0.7 && score > top_contender.1 {
                top_contender = (Some(name.clone()), score)
            }
        }

        Err(Error { 
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

    #[allow(irrefutable_let_patterns)]
    pub fn mutate_var(&mut self, i: &String, span: Span, new: Type) -> Result<(), Error> {
        let mut reversed_scopes = self.scopes.clone();
        reversed_scopes.reverse();

        let mut available_names: Vec<String> = Vec::new();
        let mut top_contender: (Option<String>, f64) = (None, 0.0);

        for scope in &mut reversed_scopes {
            for symbol in scope {
                if let Symbol::Variable { name, type_, mutability } = symbol.0 {
                    available_names.push(name.clone());
                    if name == i {
                        if *mutability {
                            if *type_ != new {
                                return Err(Error { 
                                    code: ECode::MutationError, 
                                    details: format!("`{}` has type `{}` but the new value has type `{}`", i, type_, new), 
                                    span, 
                                    src: self.src.clone(), 
                                    path: self.path.clone(),
                                    note: None,
                                    help: None
                                })
                            } else {
                                return Ok(())
                            }
                        } else {
                            return Err(Error { 
                                code: ECode::MutationError, 
                                details: format!("cannot mutate immutable variable `{}`", i), 
                                span, 
                                src: self.src.clone(), 
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        }
                    }
                }
            }
        }

        for name in available_names {
            let score = jaro_winkler(i, &name);
            if score >= 0.7 && score > top_contender.1 {
                top_contender = (Some(name.clone()), score)
            }
        }

        Err(Error { 
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

    pub fn check(&mut self) -> Vec<Error> {
        let mut errors: Vec<Error> = Vec::new();
        for node in self.module.0.clone() {
            let result = self.check_node(node.clone());
            if let Err(s) = result{
                errors.push(s)
            }
        }
        errors
    }

    pub fn check_node(&mut self, node: Node) -> Result<Type, Error> {
        match node.ast_repr {
            ASTNode::IntLit(_) => Ok(Type::Int32),
            ASTNode::FloatLit(_) => Ok(Type::Float64),
            ASTNode::StringLit(_) => Ok(Type::String),
            ASTNode::Bool(_) => Ok(Type::Boolean),
            ASTNode::Identifier(s) => self.find_identifier(&s, node.span),
            ASTNode::BinOp {
                op, lhs, rhs
            } => {
                let left = self.check_node(*lhs.clone())?;
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
                        _ => Err(Error {
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
                        Err(Error {
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
                        _ => Err(Error {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot concatenate types `{}` and `{}`", left, right),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        })
                    },
                    ":=" => {
                        if let ASTNode::Identifier(s) = lhs.ast_repr {
                            self.mutate_var(&s, node.span, right)?;
                            Ok(Type::Unit)
                        } else {
                            Err(Error {
                                code: ECode::MutationError,
                                details: format!("can only mutate variables"),
                                span: node.span,
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        }
                    }
                    _ => Err(Error {
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
                        _ => Err(Error {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot apply `+` to type `{}`", operand_type),
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
                        | Type::UInt32 | Type::UInt64 => Err(Error {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot negate unsigned integers"),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: None,
                            help: None
                        }),
                        _ => Err(Error {
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
                        _ => Err(Error {
                            code: ECode::MismatchedTypes,
                            details: format!("cannot apply `!` to type `{}`", operand_type),
                            span: node.span,
                            src: self.src.clone(),
                            path: self.path.clone(),
                            note: Some("the `!` operator can be applied to `bool` and integer types as a bitwise NOT".to_string()),
                            help: None
                        })
                    },
                    _ => Err(Error {
                        code: ECode::MismatchedTypes,
                        details: format!("invalid operator `{}`", op.0),
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
                        return Err(Error {
                            code: ECode::MismatchedTypes,
                            details: format!("`if` and `else` bodies have mismatched types: `{}`, `{}`", then_type, else_type),
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
                    Err(Error {
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
                if self.find_identifier(&name.0, node.span).is_ok() {
                    return Err(Error {
                        code: ECode::MismatchedTypes,
                        details: format!("`{}` is already declared", name.0),
                        span: name.1,
                        src: self.src.clone(),
                        path: self.path.clone(),
                        note: None,
                        help: None
                    })
                }
                let scope = self.scopes.last_mut();
                if let Some(s) = scope {
                    if let ParseType::Determined(t) = type_.0 {
                        if self.type_registry.is_registered(&name.0) {
                            return Err(Error {
                                code: ECode::MismatchedTypes,
                                details: format!("`{}` is already registed as a type", name.0),
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
                            return Err(Error {
                                code: ECode::MismatchedTypes,
                                details: format!("unregistered type `{}`", t),
                                span: type_.1.unwrap(),
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        };
                        s.insert(Symbol::Variable {
                            name: name.0, type_: t, mutability
                        }, ());
                    } else {
                        if self.type_registry.is_registered(&name.0) {
                            return Err(Error {
                                code: ECode::MismatchedTypes,
                                details: format!("`{}` is already registed as a type", name.0),
                                span: name.1,
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        }
                        s.insert(Symbol::Variable {
                            name: name.0, type_: Type::Undetermined, mutability
                        }, ());
                    }
                }

                Ok(Type::Unit)
            },
            ASTNode::DeclarationWithValue {
                type_, mutability, name, value
            } => {
                let value_type = self.check_node(*value)?;
                if self.find_identifier(&name.0, node.span).is_ok() {
                    return Err(Error {
                        code: ECode::MismatchedTypes,
                        details: format!("`{}` is already declared", name.0),
                        span: name.1,
                        src: self.src.clone(),
                        path: self.path.clone(),
                        note: None,
                        help: None
                    })
                }
                let scope = self.scopes.last_mut();
                if let Some(s) = scope {
                    if let ParseType::Determined(t) = type_.0 {
                        if self.type_registry.is_registered(&name.0) {
                            return Err(Error {
                                code: ECode::MismatchedTypes,
                                details: format!("`{}` is already registed as a type", name.0),
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
                            return Err(Error {
                                code: ECode::MismatchedTypes,
                                details: format!("unregistered type `{}`", t),
                                span: type_.1.unwrap(),
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        };
                        s.insert(Symbol::Variable {
                            name: name.0, type_: t, mutability
                        }, ());
                    } else {
                        if self.type_registry.is_registered(&name.0) {
                            return Err(Error {
                                code: ECode::MismatchedTypes,
                                details: format!("`{}` is already registed as a type", name.0),
                                span: name.1,
                                src: self.src.clone(),
                                path: self.path.clone(),
                                note: None,
                                help: None
                            })
                        }
                        s.insert(Symbol::Variable {
                            name: name.0, type_: value_type, mutability
                        }, ());
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
            },
        }
    }
}