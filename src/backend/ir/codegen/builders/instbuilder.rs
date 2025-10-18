use super::super::super::{entities::*, inst::*};
use std::{rc::Rc, cell::RefCell};

pub struct InstBuilder {
    pub(crate) block: Rc<RefCell<Vec<Inst>>>,
    pub(crate) next_id: Rc<RefCell<usize>>
}

impl InstBuilder {
    pub fn i8const(&mut self, n: i8) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::I8(n);
        let value = ValueID(id, Type::I8);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn i16const(&mut self, n: i16) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::I16(n);
        let value = ValueID(id, Type::I16);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn i32const(&mut self, n: i32) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::I32(n);
        let value = ValueID(id, Type::I32);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn i64const(&mut self, n: i64) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::I64(n);
        let value = ValueID(id, Type::I64);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn u8const(&mut self, n: u8) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::U8(n);
        let value = ValueID(id, Type::U8);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn u16const(&mut self, n: u16) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::U16(n);
        let value = ValueID(id, Type::U16);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn u32const(&mut self, n: u32) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::U32(n);
        let value = ValueID(id, Type::U32);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn u64const(&mut self, n: u64) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::U64(n);
        let value = ValueID(id, Type::U64);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn f32const(&mut self, n: f32) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::F32(n);
        let value = ValueID(id, Type::F32);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn f64const(&mut self, n: f64) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::F64(n);
        let value = ValueID(id, Type::F64);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn bool_(&mut self, n: bool) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::Bool(n as u8);
        let value = ValueID(id, Type::Bool);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }
    pub fn void(&mut self) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let c = Const::Void;
        let value = ValueID(id, Type::Void);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Const(c) });
        value
    }

    pub fn iadd(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::IAdd { left: l, right: r }});
        value
    }
    pub fn isub(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::ISub { left: l, right: r }});
        value
    }
    pub fn imul(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::IMul { left: l, right: r }});
        value
    }
    pub fn sdiv(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::SDiv { left: l, right: r }});
        value
    }
    pub fn udiv(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::UDiv { left: l, right: r }});
        value
    }
    pub fn srem(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::SRem { left: l, right: r }});
        value
    }
    pub fn urem(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::URem { left: l, right: r }});
        value
    }
    pub fn fadd(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::FAdd { left: l, right: r }});
        value
    }
    pub fn fsub(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::FSub { left: l, right: r }});
        value
    }
    pub fn fmul(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::FMul { left: l, right: r }});
        value
    }
    pub fn fdiv(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::FDiv { left: l, right: r }});
        value
    }
    pub fn frem(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::FRem { left: l, right: r }});
        value
    }
    pub fn lsh(&mut self, n: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, n.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::Lsh(n)});
        value
    }
    pub fn lrsh(&mut self, n: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, n.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::LRsh(n)});
        value
    }
    pub fn arsh(&mut self, n: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, n.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::ARsh(n)});
        value
    }
    pub fn bnot(&mut self, n: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, n.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::BNot(n)});
        value
    }
    pub fn bor(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::BOr { left: l, right: r }});
        value
    }
    pub fn band(&mut self, l: ValueID, r: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, l.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::BAnd { left: l, right: r }});
        value
    }
    pub fn ineg(&mut self, n: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, n.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::INeg(n)});
        value
    }
    pub fn fneg(&mut self, n: ValueID) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, n.1);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::FNeg(n)});
        value
    }
    pub fn icmp(&mut self, l: ValueID, r: ValueID, predicate: CmpPred) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, Type::Bool);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::ICmp { predicate, left: l, right: r }});
        value
    }
    pub fn fcmp(&mut self, l: ValueID, r: ValueID, predicate: CmpPred) -> ValueID {
        let id = {
            let mut next_id = self.next_id.borrow_mut();
            let id = *next_id;
            *next_id += 1;
            id
        };
        let value = ValueID(id, Type::Bool);
        self.block.borrow_mut().push(Inst::Assign { dest: value, op: Op::FCmp { predicate, left: l, right: r }});
        value
    }
    pub fn ret<V: Into<ValueID>>(&mut self, value: V) {
        let value_id: ValueID = value.into();
        self.block.borrow_mut().push(Inst::Ret(value_id));
    }
    pub fn jmp(&mut self, id: BlockCall) {
        self.block.borrow_mut().push(Inst::Jmp(id));
    }
    pub fn br(&mut self, condition: ValueID, true_path: BlockCall, false_path: BlockCall) {
        self.block.borrow_mut().push(Inst::Branch { condition, true_path: true_path, false_path: false_path });
    }
}