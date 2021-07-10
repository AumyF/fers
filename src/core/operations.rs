//! 命令語の解釈とその処理の実行

use itertools::Either;

#[derive(Debug, Clone, Copy)]
pub enum Operation2 {
    /// r <- (実効アドレス)
    Load,
    /// - 実効アドレスの番地 <- rの値
    /// - フラグ維持
    Store,
    /// - rを実効アドレスに
    /// - フラグ維持
    LoadAddress,

    AddArithmetic,
    SubtractArithmetic,
    AddLogical,
    SubtractLogical,

    And,
    Or,
    Xor,

    /// - OFは0
    CompareArithmetic,
    /// - OFは0
    CompareLogical,

    ShiftLeftArithmetic,
    ShiftLeftLogical,
    ShiftRightArithmetic,
    ShiftRightLogical,

    /// - フラグ維持
    JumpOnMinus,
    /// - フラグ維持
    JumpOnNonZero,
    /// - フラグ維持
    JumpOnZero,
    /// - フラグ維持
    UnconditionalJump,
    /// - フラグ維持
    JumpOnPlus,
    /// - フラグ維持
    JumpOnOverflow,

    /// - フラグ維持
    Push,

    /// - フラグ維持
    Call,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation1 {
    /// 何もしない
    NoOperation,

    /// r1にr2をセットする
    Load1,

    AddArithmetic1,
    SubtractArithmetic1,
    AddLogical1,
    SubtractLogical1,

    And1,
    Or1,
    Xor1,

    /// r1 > r2 のときsf=0, zf=0\
    /// r1 = r2 のときsf=0, zf=1\
    /// r1 < r2 のときsf=1, zf=0
    CompareArithmetic,
    /// r1 > r2 のときsf=0, zf=0\
    /// r1 = r2 のときsf=0, zf=1\
    /// r1 < r2 のときsf=1, zf=0
    CompareLogical,

    Pop,

    Return,
}

pub fn ope(word: u16) -> Result<Either<Word1, Word2>, NewError> {
    let (r1_r, r2_x) = RegisterNumber::new_pair(word);
    Ok(Operation1::new(word).map_or(
        Either::Right(Word2 {
            operation: Operation2::new(word)?,
            r: r1_r,
            x: r2_x,
        }),
        |operation| {
            Either::Left(Word1 {
                operation,
                r1: r1_r,
                r2: r2_x,
            })
        },
    ))
}

#[derive(Debug, thiserror::Error)]
pub enum NewError {
    #[error("Operation not defined for word {0:X}")]
    OperationNotDefined(u16),
    #[error("{0}")]
    RegisterOutOfIndex(#[from] RegisterOutOfIndex),
}

pub struct Word1 {
    pub operation: Operation1,
    pub r1: RegisterNumber,
    pub r2: RegisterNumber,
}

#[derive(Debug, Clone, Copy)]
pub struct Word2 {
    pub operation: Operation2,
    pub r: RegisterNumber,
    pub x: RegisterNumber,
}

impl Operation1 {
    pub fn new(word: u16) -> Result<Operation1, NewError> {
        use Operation1::*;

        Ok(match word & 0xff00 {
            0 => NoOperation,         // NOP
            0x1400 => Load1,          // LD
            0x2400 => AddArithmetic1, // ADDA
            0x2500 => SubtractArithmetic1,
            0x2600 => AddLogical1,
            0x2700 => SubtractLogical1,

            0x3400 => And1,
            0x3500 => Or1,
            0x3600 => Xor1,

            0x4400 => CompareArithmetic,
            0x4500 => CompareLogical,

            0x7100 => Pop,

            0x8100 => Return,

            e => Err(NewError::OperationNotDefined(e))?,
        })
    }
}

impl Operation2 {
    pub fn new(word: u16) -> Result<Operation2, NewError> {
        use Operation2::*;

        Ok(match word & 0xff00 {
            0x1000 => Load,
            0x1100 => Store,
            0x1200 => LoadAddress,

            0x2000 => AddArithmetic,
            0x2100 => SubtractArithmetic,
            0x2200 => AddLogical,
            0x2300 => SubtractLogical,

            0x3000 => And,
            0x3100 => Or,
            0x3200 => Xor,

            0x4000 => CompareArithmetic,
            0x4100 => CompareLogical,

            0x5000 => ShiftLeftArithmetic,
            0x5100 => ShiftLeftLogical,
            0x5200 => ShiftRightArithmetic,
            0x5300 => ShiftRightLogical,

            0x6100 => JumpOnMinus,
            0x6200 => JumpOnNonZero,
            0x6300 => JumpOnZero,
            0x6400 => UnconditionalJump,
            0x6500 => JumpOnPlus,
            0x6600 => JumpOnOverflow,

            0x7000 => Push,
            0x8000 => Call,

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

/// GRの番号．0~7の保証付き．
#[derive(Debug, Clone, Copy)]
pub struct RegisterNumber(pub u8);

impl RegisterNumber {
    pub fn new(n: u16) -> RegisterNumber {
        RegisterNumber(n as u8)
    }
    pub fn new_pair(word: u16) -> (RegisterNumber, RegisterNumber) {
        let r1_r = (word & 0x00f0) >> 1;
        let r2_x = word & 0x000f;
        (RegisterNumber::new(r1_r), RegisterNumber::new(r2_x))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0:X} is out of range for general register")]
pub struct RegisterOutOfIndex(u16);

impl TwoRegisters {
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
