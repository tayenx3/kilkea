use std::fmt;

use super::entities::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Const(Const),
    IAdd {
        left: ValueID, right: ValueID
    },
    ISub {
        left: ValueID, right: ValueID
    },
    IMul {
        left: ValueID, right: ValueID
    },
    SDiv {
        left: ValueID, right: ValueID
    },
    UDiv {
        left: ValueID, right: ValueID
    },
    SRem {
        left: ValueID, right: ValueID
    },
    URem {
        left: ValueID, right: ValueID
    },
    FAdd {
        left: ValueID, right: ValueID
    },
    FSub {
        left: ValueID, right: ValueID
    },
    FMul {
        left: ValueID, right: ValueID
    },
    FDiv {
        left: ValueID, right: ValueID
    },
    FRem {
        left: ValueID, right: ValueID
    },
    Lsh(ValueID), LRsh(ValueID), ARsh(ValueID),
    BNot(ValueID), 
    BOr {
        left: ValueID, right: ValueID
    },
    BAnd {
        left: ValueID, right: ValueID
    },
    INeg(ValueID),
    FNeg(ValueID),
    ICmp { predicate: CmpPred, left: ValueID, right: ValueID },
    FCmp { predicate: CmpPred, left: ValueID, right: ValueID },
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(c) => write!(f, "const {}", c),
            Self::IAdd { left, right } => write!(f, "iadd {} {}", left, right),
            Self::ISub { left, right } => write!(f, "isub {} {}", left, right),
            Self::IMul { left, right } => write!(f, "imul {} {}", left, right),
            Self::SDiv { left, right } => write!(f, "sdiv {} {}", left, right),
            Self::UDiv { left, right } => write!(f, "udiv {} {}", left, right),
            Self::SRem { left, right } => write!(f, "srem {} {}", left, right),
            Self::URem { left, right } => write!(f, "urem {} {}", left, right),

            Self::FAdd { left, right } => write!(f, "fadd {} {}", left, right),
            Self::FSub { left, right } => write!(f, "fsub {} {}", left, right),
            Self::FMul { left, right } => write!(f, "fmul {} {}", left, right),
            Self::FDiv { left, right } => write!(f, "fdiv {} {}", left, right),
            Self::FRem { left, right } => write!(f, "frem {} {}", left, right),

            Self::Lsh(i) => write!(f, "lsh {}", i),
            Self::LRsh(i) => write!(f, "lrsh {}", i),
            Self::ARsh(i) => write!(f, "arsh {}", i),
            Self::BNot(i) => write!(f, "bnot {}", i),
            Self::BOr { left, right } => write!(f, "bor {} {}", left, right),
            Self::BAnd { left, right } => write!(f, "band {} {}", left, right),
            Self::INeg(i) => write!(f, "ineg {}", i),
            Self::FNeg(i) => write!(f, "fneg {}", i),

            Self::ICmp { predicate, left, right } => write!(f, "icmp {} {} {}", predicate, left, right),
            Self::FCmp { predicate, left, right } => write!(f, "fcmp {} {} {}", predicate, left, right),
            _ => todo!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmpPred {
    Eq, Ne, SGt, SLt, SGe, SLe, UGt, ULt, UGe, ULe
}

impl CmpPred {
    pub fn eq() -> Self { Self::Eq }
    pub fn ne() -> Self { Self::Ne }
    pub fn sgt() -> Self { Self::SGt }
    pub fn slt() -> Self { Self::SLt }
    pub fn sge() -> Self { Self::SGe }
    pub fn sle() -> Self { Self::SLe }
    pub fn ugt() -> Self { Self::UGt }
    pub fn ult() -> Self { Self::ULt }
    pub fn uge() -> Self { Self::UGe }
    pub fn ule() -> Self { Self::ULe }
}

impl fmt::Display for CmpPred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::Ne => write!(f, "ne"),
            Self::SGt => write!(f, "sgt"),
            Self::SLt => write!(f, "slt"),
            Self::SGe => write!(f, "sge"),
            Self::SLe => write!(f, "sle"),
            Self::UGt => write!(f, "ugt"),
            Self::ULt => write!(f, "ult"),
            Self::UGe => write!(f, "uge"),
            Self::ULe => write!(f, "ule"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockCall {
    pub(crate) block: BlockID,
    pub(crate) args: Vec<ValueID>
}

impl fmt::Display for BlockCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.block, self.args.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Inst {
    Assign { dest: ValueID, op: Op },
    Ret(ValueID),
    Jmp(BlockCall),
    Branch {
        condition: ValueID,
        true_path: BlockCall,
        false_path: BlockCall
    }
}

impl fmt::Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign { dest, op } => write!(f, "{} = {}", dest, op),
            Self::Ret(r) => write!(f, "ret {}", r),
            Self::Jmp(b) => write!(f, "jmp {}", b),
            Self::Branch { 
                condition, true_path, false_path 
            } => write!(f, "br {} {} {}", condition, true_path, false_path),
        }
    }
}