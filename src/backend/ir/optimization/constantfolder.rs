use std::collections::HashMap;

use crate::backend::ir::{entities::*, inst::*};

use super::*;

pub struct ConstantFolder {
    constants: HashMap<usize, Const>
}

impl ConstantFolder {
    pub fn new() -> Self {
        Self {
            constants: HashMap::new()
        }
    }

    pub fn run(&mut self, module: &mut Module) {
        let functions = &mut module.functions;

        for function in functions {
            self.constants.clear();
            let blocks = &mut function.blocks;

            for block in blocks {
                let consts_to_remove: Vec<usize> = block.insts
                    .iter()
                    .enumerate()
                    .filter_map(|(i, inst)| {
                        if let Inst::Assign { dest, op: Op::Const(c) } = inst {
                            self.constants.insert(dest.0, c.clone());
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect();
                for (i, inst) in block.insts.iter_mut().enumerate() {
                    match inst {
                        Inst::Assign { dest, op } => {
                            match op {
                                Op::Const(c) => {
                                    self.constants.insert(dest.0, c.clone());
                                },
                                Op::IAdd { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::I8(l), Const::I8(r)) => *op = Op::Const(Const::I8(l + r)),
                                        (Const::I16(l), Const::I16(r)) => *op = Op::Const(Const::I16(l + r)),
                                        (Const::I32(l), Const::I32(r)) => *op = Op::Const(Const::I32(l + r)),
                                        (Const::I64(l), Const::I64(r)) => *op = Op::Const(Const::I64(l + r)),
                                        (Const::U8(l), Const::U8(r)) => *op = Op::Const(Const::U8(l + r)),
                                        (Const::U16(l), Const::U16(r)) => *op = Op::Const(Const::U16(l + r)),
                                        (Const::U32(l), Const::U32(r)) => *op = Op::Const(Const::U32(l + r)),
                                        (Const::U64(l), Const::U64(r)) => *op = Op::Const(Const::U64(l + r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::ISub { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::I8(l), Const::I8(r)) => *op = Op::Const(Const::I8(l - r)),
                                        (Const::I16(l), Const::I16(r)) => *op = Op::Const(Const::I16(l - r)),
                                        (Const::I32(l), Const::I32(r)) => *op = Op::Const(Const::I32(l - r)),
                                        (Const::I64(l), Const::I64(r)) => *op = Op::Const(Const::I64(l - r)),
                                        (Const::U8(l), Const::U8(r)) => *op = Op::Const(Const::U8(l - r)),
                                        (Const::U16(l), Const::U16(r)) => *op = Op::Const(Const::U16(l - r)),
                                        (Const::U32(l), Const::U32(r)) => *op = Op::Const(Const::U32(l - r)),
                                        (Const::U64(l), Const::U64(r)) => *op = Op::Const(Const::U64(l - r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::IMul { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::I8(l), Const::I8(r)) => *op = Op::Const(Const::I8(l * r)),
                                        (Const::I16(l), Const::I16(r)) => *op = Op::Const(Const::I16(l * r)),
                                        (Const::I32(l), Const::I32(r)) => *op = Op::Const(Const::I32(l * r)),
                                        (Const::I64(l), Const::I64(r)) => *op = Op::Const(Const::I64(l * r)),
                                        (Const::U8(l), Const::U8(r)) => *op = Op::Const(Const::U8(l * r)),
                                        (Const::U16(l), Const::U16(r)) => *op = Op::Const(Const::U16(l * r)),
                                        (Const::U32(l), Const::U32(r)) => *op = Op::Const(Const::U32(l * r)),
                                        (Const::U64(l), Const::U64(r)) => *op = Op::Const(Const::U64(l * r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::SDiv { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::I8(l), Const::I8(r)) => *op = Op::Const(Const::I8(l / r)),
                                        (Const::I16(l), Const::I16(r)) => *op = Op::Const(Const::I16(l / r)),
                                        (Const::I32(l), Const::I32(r)) => *op = Op::Const(Const::I32(l / r)),
                                        (Const::I64(l), Const::I64(r)) => *op = Op::Const(Const::I64(l / r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::UDiv { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::U8(l), Const::U8(r)) => *op = Op::Const(Const::U8(l / r)),
                                        (Const::U16(l), Const::U16(r)) => *op = Op::Const(Const::U16(l / r)),
                                        (Const::U32(l), Const::U32(r)) => *op = Op::Const(Const::U32(l / r)),
                                        (Const::U64(l), Const::U64(r)) => *op = Op::Const(Const::U64(l / r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::SRem { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::I8(l), Const::I8(r)) => *op = Op::Const(Const::I8(l % r)),
                                        (Const::I16(l), Const::I16(r)) => *op = Op::Const(Const::I16(l % r)),
                                        (Const::I32(l), Const::I32(r)) => *op = Op::Const(Const::I32(l % r)),
                                        (Const::I64(l), Const::I64(r)) => *op = Op::Const(Const::I64(l % r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::URem { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::U8(l), Const::U8(r)) => *op = Op::Const(Const::U8(l % r)),
                                        (Const::U16(l), Const::U16(r)) => *op = Op::Const(Const::U16(l % r)),
                                        (Const::U32(l), Const::U32(r)) => *op = Op::Const(Const::U32(l % r)),
                                        (Const::U64(l), Const::U64(r)) => *op = Op::Const(Const::U64(l % r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },

                                Op::FAdd { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::F32(l), Const::F32(r)) => *op = Op::Const(Const::F32(l + r)),
                                        (Const::F64(l), Const::F64(r)) => *op = Op::Const(Const::F64(l + r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::FSub { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::F32(l), Const::F32(r)) => *op = Op::Const(Const::F32(l - r)),
                                        (Const::F64(l), Const::F64(r)) => *op = Op::Const(Const::F64(l - r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::FMul { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::F32(l), Const::F32(r)) => *op = Op::Const(Const::F32(l * r)),
                                        (Const::F64(l), Const::F64(r)) => *op = Op::Const(Const::F64(l * r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::FDiv { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::F32(l), Const::F32(r)) => *op = Op::Const(Const::F32(l / r)),
                                        (Const::F64(l), Const::F64(r)) => *op = Op::Const(Const::F64(l / r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::FRem { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::F32(l), Const::F32(r)) => *op = Op::Const(Const::F32(l % r)),
                                        (Const::F64(l), Const::F64(r)) => *op = Op::Const(Const::F64(l % r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },

                                Op::Lsh(v) => {
                                    let value = match self.constants.get(&v.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match value {
                                        Const::I8(l) => *op = Op::Const(Const::I8(l << 1)),
                                        Const::I16(l) => *op = Op::Const(Const::I16(l << 1)),
                                        Const::I32(l) => *op = Op::Const(Const::I32(l << 1)),
                                        Const::I64(l) => *op = Op::Const(Const::I64(l << 1)),
                                        Const::U8(l) => *op = Op::Const(Const::U8(l << 1)),
                                        Const::U16(l) => *op = Op::Const(Const::U16(l << 1)),
                                        Const::U32(l) => *op = Op::Const(Const::U32(l << 1)),
                                        Const::U64(l) => *op = Op::Const(Const::U64(l << 1)),
                                        Const::F32(l) => *op = Op::Const(Const::F32(f32::from_bits(l.to_bits() << 1))),
                                        Const::F64(l) => *op = Op::Const(Const::F64(f64::from_bits(l.to_bits() << 1))),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::LRsh(v) => {
                                    let value = match self.constants.get(&v.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match value {
                                        Const::I8(l) => *op = Op::Const(Const::I8((*l as u8 >> 1) as i8)),
                                        Const::I16(l) => *op = Op::Const(Const::I16((*l as u16 >> 1) as i16)),
                                        Const::I32(l) => *op = Op::Const(Const::I32((*l as u32 >> 1) as i32)),
                                        Const::I64(l) => *op = Op::Const(Const::I64((*l as u64 >> 1) as i64)),
                                        Const::U8(l) => *op = Op::Const(Const::U8(l >> 1)),
                                        Const::U16(l) => *op = Op::Const(Const::U16(l >> 1)),
                                        Const::U32(l) => *op = Op::Const(Const::U32(l >> 1)),
                                        Const::U64(l) => *op = Op::Const(Const::U64(l >> 1)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::ARsh(v) => {
                                    let value = match self.constants.get(&v.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match value {
                                        Const::I8(l) => *op = Op::Const(Const::I8(l >> 1)),
                                        Const::I16(l) => *op = Op::Const(Const::I16(l >> 1)),
                                        Const::I32(l) => *op = Op::Const(Const::I32(l >> 1)),
                                        Const::I64(l) => *op = Op::Const(Const::I64(l >> 1)),
                                        Const::U8(l) => *op = Op::Const(Const::U8((*l as i8 >> 1) as u8)),
                                        Const::U16(l) => *op = Op::Const(Const::U16((*l as i16 >> 1) as u16)),
                                        Const::U32(l) => *op = Op::Const(Const::U32((*l as i32 >> 1) as u32)),
                                        Const::U64(l) => *op = Op::Const(Const::U64((*l as i64 >> 1) as u64)),
                                        Const::F32(l) => *op = Op::Const(Const::F32(l / 2.0)),
                                        Const::F64(l) => *op = Op::Const(Const::F64(l / 2.0)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::BNot(v) => {
                                    let value = match self.constants.get(&v.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match value {
                                        Const::I8(l) => *op = Op::Const(Const::I8(!l)),
                                        Const::I16(l) => *op = Op::Const(Const::I16(!l)),
                                        Const::I32(l) => *op = Op::Const(Const::I32(!l)),
                                        Const::I64(l) => *op = Op::Const(Const::I64(!l)),
                                        Const::U8(l) => *op = Op::Const(Const::U8(!l)),
                                        Const::U16(l) => *op = Op::Const(Const::U16(!l)),
                                        Const::U32(l) => *op = Op::Const(Const::U32(!l)),
                                        Const::U64(l) => *op = Op::Const(Const::U64(!l)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::BOr { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::I8(l), Const::I8(r)) => *op = Op::Const(Const::I8(l | r)),
                                        (Const::I16(l), Const::I16(r)) => *op = Op::Const(Const::I16(l | r)),
                                        (Const::I32(l), Const::I32(r)) => *op = Op::Const(Const::I32(l | r)),
                                        (Const::I64(l), Const::I64(r)) => *op = Op::Const(Const::I64(l | r)),
                                        (Const::U8(l), Const::U8(r)) => *op = Op::Const(Const::U8(l | r)),
                                        (Const::U16(l), Const::U16(r)) => *op = Op::Const(Const::U16(l | r)),
                                        (Const::U32(l), Const::U32(r)) => *op = Op::Const(Const::U32(l | r)),
                                        (Const::U64(l), Const::U64(r)) => *op = Op::Const(Const::U64(l | r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::BAnd { left, right } => {
                                    let left_value = match self.constants.get(&left.0) {
                                        Some(s) => s,
                                        None => continue
                                    };
                                    let right_value = match self.constants.get(&right.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match (left_value, right_value) {
                                        (Const::I8(l), Const::I8(r)) => *op = Op::Const(Const::I8(l & r)),
                                        (Const::I16(l), Const::I16(r)) => *op = Op::Const(Const::I16(l & r)),
                                        (Const::I32(l), Const::I32(r)) => *op = Op::Const(Const::I32(l & r)),
                                        (Const::I64(l), Const::I64(r)) => *op = Op::Const(Const::I64(l & r)),
                                        (Const::U8(l), Const::U8(r)) => *op = Op::Const(Const::U8(l & r)),
                                        (Const::U16(l), Const::U16(r)) => *op = Op::Const(Const::U16(l & r)),
                                        (Const::U32(l), Const::U32(r)) => *op = Op::Const(Const::U32(l & r)),
                                        (Const::U64(l), Const::U64(r)) => *op = Op::Const(Const::U64(l & r)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::INeg(v) => {
                                    let value = match self.constants.get(&v.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match value {
                                        Const::I8(l) => *op = Op::Const(Const::I8(-l)),
                                        Const::I16(l) => *op = Op::Const(Const::I16(-l)),
                                        Const::I32(l) => *op = Op::Const(Const::I32(-l)),
                                        Const::I64(l) => *op = Op::Const(Const::I64(-l)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                Op::FNeg(v) => {
                                    let value = match self.constants.get(&v.0) {
                                        Some(s) => s,
                                        None => continue
                                    };

                                    match value {
                                        Const::F32(l) => *op = Op::Const(Const::F32(-l)),
                                        Const::F64(l) => *op = Op::Const(Const::F64(-l)),
                                        _ => continue
                                    }
                                    if let Op::Const(c) = op {
                                        self.constants.insert(dest.0, c.clone());
                                    }
                                },
                                _ => continue
                            }
                        },
                        _ => continue
                    }
                }
                for &i in consts_to_remove.iter().rev() {
                    block.insts.remove(i);
                }
            }
        }
    }
}

impl OptimizationPass for ConstantFolder {
    fn name(&self) -> String {
        "ConstantFolder".to_string()
    }
    fn apply(&mut self, module: &mut Module) {
        self.run(module);
    }
}