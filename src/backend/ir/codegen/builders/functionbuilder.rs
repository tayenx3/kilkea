use super::*;
use super::super::super::entities::*;
use std::{rc::Rc, cell::RefCell};

pub struct FunctionBuilder {
    pub(crate) function: Function,
    pub(crate) next_id: Rc<RefCell<usize>>,
    pub(crate) next_block_id: usize,
}

impl FunctionBuilder {
    pub fn create_block(&mut self) -> BlockBuilder {
        let block_id = self.next_block_id;
        self.next_block_id += 1;
        
        BlockBuilder {
            block: Block {
                id: BlockID(block_id),
                insts: Vec::new(),
                params: Vec::new()
            },
            insts: Rc::new(RefCell::new(Vec::new())),
            next_id: self.next_id.clone(),
        }
    }
    
    pub fn eat_block(&mut self, block_builder: BlockBuilder) {
        let mut block = block_builder.block;
        block.insts = Rc::try_unwrap(block_builder.insts)
            .expect("Multiple references to instructions")
            .into_inner();
            
        self.function.blocks.push(block);
    }

    pub fn build(self) -> Function { self.function }
}