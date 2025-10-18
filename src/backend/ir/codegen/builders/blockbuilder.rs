use super::*;
use super::super::super::{entities::*, inst::*};
use std::{rc::Rc, cell::RefCell};

pub struct BlockBuilder {
    pub(crate) block: Block,
    pub(crate) insts: Rc<RefCell<Vec<Inst>>>,
    pub(crate) next_id: Rc<RefCell<usize>>
}

impl BlockBuilder {
    pub fn ins(&self) -> InstBuilder {
        InstBuilder { block: self.insts.clone(), next_id: self.next_id.clone() }
    }

    pub fn id(&self) -> BlockID {
        self.block.id.clone()
    }
    pub fn with_param(mut self, param: Type) -> Self {
        self.block.params.push(ParamID(*self.next_id.borrow(), param));
        *self.next_id.borrow_mut() += 1;
        self
    }
    pub fn call(&self, args: &[ValueID]) -> BlockCall {
        BlockCall { block: self.block.id.clone(), args: args.to_vec() }
    }
    pub fn get_param(&self, index: usize) -> ParamID {
        self.block.params[index]
    }
}