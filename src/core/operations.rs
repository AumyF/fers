//! 命令語の解釈とその処理の実行

use super::machine::{Machine, RegisterOutOfIndexError};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Operations {
    NoOperation,
    Load2,
    Store,
    LoadAddress,
    Load1,
    AddArithmetic1(TwoRegisters),
    SubtractArithmetic,
    AddLogical,
    SubtratLogical,
}

impl Operations {
    // TODO 返り値はResultにする
    pub fn new(word: u16) -> Operations {
        use Operations::*;
        match word & 0xff00 {
            0 => NoOperation, // NOP
            0x2400 => AddArithmetic1(TwoRegisters {
                r1: (word & 0x00f0) >> 1,
                r2: (word & 0x000f),
            }),
            e => unimplemented!(),
        }
    }

    // TODO &Machineにする
    pub fn exec(&self, machine: Machine) -> Result<Machine, OperationExecutionError> {
        Ok(match *self {
            Operations::NoOperation => machine,
            Operations::AddArithmetic1(TwoRegisters { r1, r2 }) => machine
                .manipulate_gr(r1, r2, |r1, r2| r1 + r2)
                .or_else(|e| Err(OperationExecutionError::RegisterOutOfIndex(e)))?,
            _ => unimplemented!(),
        })
    }
}

struct OperationNotDefined {
    word: u16,
}

#[derive(Debug, Clone, Copy)]
struct TwoRegisters {
    r1: u16,
    r2: u16,
}

#[derive(Debug)]
pub enum OperationExecutionError {
    RegisterOutOfIndex(RegisterOutOfIndexError),
}

impl fmt::Display for OperationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OperationExecutionError::RegisterOutOfIndex(r) => r,
            }
        )
    }
}

mod test {
    use std::{error, io};

    use crate::core::machine::Machine;

    // TODO テスト書く
    // #[test]
    // fn add_arithmetic_1() -> Result<(), Box<dyn error::Error>> {
    //     let vec = vec![0x00, 0x24, 0x12];
    //     let mut bytes = io::Cursor::new(vec);
    //     let machine = Machine::init(&mut bytes);
    //     // assert_eq!()

    //     Ok(())
    // }
}
