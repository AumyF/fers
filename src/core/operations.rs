//! 命令語の解釈とその処理の実行

#[derive(Debug, Clone, Copy)]
pub enum Operations {
    NoOperation,
    Load2(TwoRegisters),
    Store,
    LoadAddress,
    /// r1にr2をセットする
    Load1(TwoRegisters),

    AddArithmetic2,
    SubtractArithmetic2,
    AddLogical2,
    SubtractLogical2,

    AddArithmetic1(TwoRegisters),
    SubtractArithmetic1(TwoRegisters),
    AddLogical1(TwoRegisters),
    SubtractLogical1(TwoRegisters),

    And1(TwoRegisters),
    Or1(TwoRegisters),
    Xor1(TwoRegisters),

    Call(TwoRegisters),
    Return,
}

pub enum NumericLogical1 {
    Add(TwoRegisters),
    Subtract(TwoRegisters),
}

impl NumericLogical1 {
    fn eval(&self) -> Box<dyn FnOnce(u16, u16) -> Option<u16>> {
        use NumericLogical1::*;
        Box::new(match *self {
            Add(grs) => u16::checked_add,
            Subtract(_) => u16::checked_sub,
        })
    }
}

pub enum LogicalBitwise1 {
    And(TwoRegisters),
    Or(TwoRegisters),
    Xor(TwoRegisters),
}

impl LogicalBitwise1 {
    fn eval(&self) -> Box<dyn FnOnce(u16, u16) -> u16> {
        use std::ops;
        use LogicalBitwise1::*;
        Box::new(match *self {
            And(_) => ops::BitAnd::bitand,
            Or(_) => ops::BitOr::bitor,
            Xor(_) => ops::BitXor::bitxor,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NewError {
    #[error("Operation not defined for word {0:X}")]
    OperationNotDefined(u16),
    #[error("{0}")]
    RegisterOutOfIndex(#[from] RegisterOutOfIndex),
}

impl Operations {
    pub fn new(word: u16) -> Result<Operations, NewError> {
        use Operations::*;

        // TODO オペランドの形違いに対応する
        let two_registers = TwoRegisters::new(word);
        Ok(match word & 0xff00 {
            0 => NoOperation, // NOP
            0x1000 => Load2(two_registers?),
            0x1400 => Load1(two_registers?),          // LD
            0x2400 => AddArithmetic1(two_registers?), // ADDA
            0x2500 => SubtractArithmetic1(two_registers?),
            0x2600 => AddLogical1(two_registers?),
            0x2700 => SubtractLogical1(two_registers?),

            0x3400 => And1(two_registers?),
            0x3500 => Or1(two_registers?),
            0x3600 => Xor1(two_registers?),

            0x8000 => Call(two_registers?),
            0x8100 => Return,

            e => Err(NewError::OperationNotDefined(e))?,
        })
    }
}

/// GRのインデックスのペア。R1 <- f (R1, R2) みたいな演算で使う。
/// `new` で範囲内 (R <= 7) であることを保証しているので
/// これを使うときは範囲外アクセスを気にして `Result` を使う必要はない。
#[derive(Debug, Clone, Copy)]
pub struct TwoRegisters {
    r1: u16,
    r2: u16,
}

#[derive(Debug, thiserror::Error)]
#[error("{0:X}{1:X} is out of range for general register")]
pub struct RegisterOutOfIndex(u16, u16);

impl TwoRegisters {
    fn new(word: u16) -> Result<TwoRegisters, RegisterOutOfIndex> {
        let r1 = (word & 0x00f0) >> 1;
        let r2 = word & 0x000f;
        if r1 > 7 || r2 > 7 {
            Err(RegisterOutOfIndex(r1, r2))
        } else {
            Ok(TwoRegisters { r1, r2 })
        }
    }
    pub fn get_pair(&self) -> (&u16, &u16) {
        let Self { r1, r2 } = self;
        (r1, r2)
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
