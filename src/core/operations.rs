//! 命令語の解釈とその処理の実行

use itertools::Either;

#[derive(Debug, Clone, Copy)]
pub enum TwoWordOperations {
    Load(TwoRegisters),
    Store(TwoRegisters),
    LoadAddress(TwoRegisters),

    AddArithmetic(TwoRegisters),
    SubtractArithmetic(TwoRegisters),
    AddLogical(TwoRegisters),
    SubtractLogical(TwoRegisters),

    And(TwoRegisters),
    Or(TwoRegisters),
    Xor(TwoRegisters),

    CompareArithmetic(TwoRegisters),
    CompareLogical(TwoRegisters),

    ShiftLeftArithmetic(TwoRegisters),
    ShiftLeftLogical(TwoRegisters),
    ShiftRightArithmetic(TwoRegisters),
    ShiftRightLogical(TwoRegisters),

    JumpOnMinus(TwoRegisters),
    JumpOnNonZero(TwoRegisters),
    JumpOnZero(TwoRegisters),
    UnconditionalJump(TwoRegisters),
    JumpOnPlus(TwoRegisters),
    JumpOnOverflow(TwoRegisters),

    Push(TwoRegisters),

    Call(TwoRegisters),
}

#[derive(Debug, Clone, Copy)]
pub enum Operations {
    NoOperation,

    /// r1にr2をセットする
    Load1(TwoRegisters),

    AddArithmetic1(TwoRegisters),
    SubtractArithmetic1(TwoRegisters),
    AddLogical1(TwoRegisters),
    SubtractLogical1(TwoRegisters),

    And1(TwoRegisters),
    Or1(TwoRegisters),
    Xor1(TwoRegisters),

    CompareArithmetic1(TwoRegisters),
    CompareLogical1(TwoRegisters),

    Pop(TwoRegisters),

    Return,
}

pub fn ope(word: u16) -> Result<Either<Operations, TwoWordOperations>, NewError> {
    Ok(Operations::new(word).map_or(Either::Right(TwoWordOperations::new(word)?), Either::Left))
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
            0 => NoOperation,                         // NOP
            0x1400 => Load1(two_registers?),          // LD
            0x2400 => AddArithmetic1(two_registers?), // ADDA
            0x2500 => SubtractArithmetic1(two_registers?),
            0x2600 => AddLogical1(two_registers?),
            0x2700 => SubtractLogical1(two_registers?),

            0x3400 => And1(two_registers?),
            0x3500 => Or1(two_registers?),
            0x3600 => Xor1(two_registers?),

            0x7100 => Pop(two_registers?),

            0x8100 => Return,

            e => Err(NewError::OperationNotDefined(e))?,
        })
    }
}

impl TwoWordOperations {
    pub fn new(word: u16) -> Result<TwoWordOperations, NewError> {
        use TwoWordOperations::*;

        let two_registers = TwoRegisters::new(word)?;
        Ok(match word & 0xff00 {
            0x1000 => Load(two_registers),
            0x1100 => Store(two_registers),
            0x1200 => LoadAddress(two_registers),

            0x2000 => AddArithmetic(two_registers),
            0x2100 => SubtractArithmetic(two_registers),
            0x2200 => AddLogical(two_registers),
            0x2300 => SubtractLogical(two_registers),

            0x3000 => And(two_registers),
            0x3100 => Or(two_registers),
            0x3200 => Xor(two_registers),

            0x4000 => CompareArithmetic(two_registers),
            0x4100 => CompareLogical(two_registers),

            0x5000 => ShiftLeftArithmetic(two_registers),
            0x5100 => ShiftLeftLogical(two_registers),
            0x5200 => ShiftRightArithmetic(two_registers),
            0x5300 => ShiftRightLogical(two_registers),

            0x6100 => JumpOnMinus(two_registers),
            0x6200 => JumpOnNonZero(two_registers),
            0x6300 => JumpOnZero(two_registers),
            0x6400 => UnconditionalJump(two_registers),
            0x6500 => JumpOnPlus(two_registers),
            0x6600 => JumpOnOverflow(two_registers),

            0x7000 => Push(two_registers),
            0x8000 => Call(two_registers),

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
