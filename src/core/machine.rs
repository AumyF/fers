use itertools::Either;

use crate::core::operations::Word1;

use super::memory;
use super::operations::{Operation2, RegisterNumber, Word2};
use super::register::GeneralRegister;
use super::utils::is_negative;
use super::{memory::Memory, operations, operations::Operation1};
use std::ops;
use std::rc::Rc;
use std::{cmp, io};

/// プログラム開始時に確保されるスタックの大きさ (ワード数)。
pub const STACK_SIZE: usize = 256;

#[derive(Debug)]
pub struct Machine {
    /// メモリ。`Rc` で共有したほうがメモリの節約になるのかもしれないという思いがある
    pub mem: Rc<Memory>,
    gr: GeneralRegister,
    sp: u16,
    pr: u16,
    of: bool,
    sf: bool,
    zf: bool,
    /// 前の命令。アドレス関係で2ワード読む場合に前の命令が何だったか保持するのに使う
    previous_word: Option<Word2>,
}

impl Clone for Machine {
    fn clone(&self) -> Self {
        Machine {
            mem: Rc::clone(&self.mem),
            ..*self
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MachineInitError {
    #[error("{0}")]
    MemoryInitFailed(#[from] memory::LoadProgramError),
}

impl Machine {
    pub fn init(stream: &mut impl io::Read) -> Result<Machine, MachineInitError> {
        println!("v:{:X}", 3);
        let mem = Rc::new(Memory::load_program(stream)?);

        println!("v:{:X}", mem.0[STACK_SIZE]);
        Ok(Machine {
            mem,
            gr: GeneralRegister::new([0; 8]),
            sp: STACK_SIZE as u16,
            pr: STACK_SIZE as u16,
            of: false,
            sf: false,
            zf: false,

            previous_word: None,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("{0}")]
    OperationNotDefined(#[from] operations::NewError),
}

pub struct Values2 {
    r: u16,
    effective_addr: u16,
    mem_value: u16,
}

impl Machine {
    fn get_effective_value(&self, x: RegisterNumber, addr: u16) -> u16 {
        let (_, x) = self.gr.get(x, x);
        addr + x
    }
    fn access2(
        &self,
        r: RegisterNumber,
        x: RegisterNumber,
        addr: u16,
    ) -> Result<Values2, memory::GetError> {
        let (r, x) = self.gr.get(r, x);
        let effective_addr = addr + x;
        let mem_value = self.mem.get(effective_addr)?;

        Ok(Values2 {
            r,
            effective_addr,
            mem_value,
        })
    }
    fn exec(&self, word: u16) -> Result<Machine, ExecError> {
        use Operation1::*;

        // 2語目
        if let Some(Word2 { operation, r, x }) = self.previous_word {
            use Operation2::*;
            // TODO access2のエラー処理 たぶんエラー出る

            return Ok(match operation {
                Load => {
                    let Values2 { mem_value, .. } = self.access2(r, x, word).unwrap();
                    self.mod_gr(r, mem_value).set_sf_zf(mem_value)
                }
                Store => {
                    let Values2 {
                        effective_addr, r, ..
                    } = self.access2(r, x, word).unwrap();

                    let mut mem = self.mem.0.clone();
                    mem[effective_addr as usize] = r;
                    let mem = Rc::new(Memory(mem));
                    Machine {
                        mem,
                        ..self.clone()
                    }
                }

                LoadAddress => {
                    let Values2 { effective_addr, .. } = self.access2(r, x, word).unwrap();
                    self.mod_gr(r, effective_addr)
                }

                AddLogical => {
                    let Values2 {
                        r: r_value,
                        mem_value,
                        ..
                    } = self.access2(r, x, word).unwrap();

                    let (r_value, of) = r_value.overflowing_add(mem_value);

                    Machine {
                        of,
                        ..self.mod_gr(r, r_value).set_sf_zf(r_value)
                    }
                }

                SubtractLogical => {
                    let Values2 {
                        r: r_value,
                        mem_value,
                        ..
                    } = self.access2(r, x, word).unwrap();

                    let (r_value, of) = r_value.overflowing_sub(mem_value);

                    Machine {
                        of,
                        ..self.mod_gr(r, r_value).set_sf_zf(r_value)
                    }
                }

                Or => {
                    let Values2 {
                        r: r_value,
                        mem_value,
                        ..
                    } = self.access2(r, x, word).unwrap();

                    let r_value = r_value | mem_value;

                    self.mod_gr(r, r_value).set_sf_zf(r_value)
                }
                And => {
                    let Values2 {
                        r: r_value,
                        mem_value,
                        ..
                    } = self.access2(r, x, word).unwrap();

                    let r_value = r_value & mem_value;

                    self.mod_gr(r, r_value).set_sf_zf(r_value)
                }
                Xor => {
                    let Values2 {
                        r: r_value,
                        mem_value,
                        ..
                    } = self.access2(r, x, word).unwrap();

                    let r_value = r_value ^ mem_value;

                    self.mod_gr(r, r_value).set_sf_zf(r_value)
                }

                JumpOnPlus => self.jump_to(x, word, self.sf == false && self.zf == false),
                JumpOnMinus => self.jump_to(x, word, self.sf == true),
                JumpOnNonZero => self.jump_to(x, word, self.zf == false),
                JumpOnZero => self.jump_to(x, word, self.zf == true),
                JumpOnOverflow => self.jump_to(x, word, self.of == true),
                UnconditionalJump => self.jump_to(x, word, true),

                Push => {
                    let Values2 { effective_addr, .. } = self.access2(r, x, word).unwrap();

                    let mut mem = self.mem.0.clone();

                    let sp = self.sp - 1;
                    mem[self.sp as usize] = effective_addr;

                    let mem = Rc::new(Memory(mem));
                    Machine {
                        sp,
                        mem,
                        ..self.clone()
                    }
                }

                Call => {
                    let Values2 { effective_addr, .. } = self.access2(r, x, word).unwrap();

                    let mut mem = self.mem.0.clone();

                    let sp = self.sp - 1;
                    mem[sp as usize] = self.pr;

                    let mem = Rc::new(Memory(mem));
                    let pr = effective_addr;
                    Machine {
                        sp,
                        mem,
                        pr,
                        ..self.clone()
                    }
                }
                _ => unimplemented!(),
            });
        }

        // 1語目
        Ok(match operations::ope(word)? {
            Either::Left(Word1 { operation, r1, r2 }) => match operation {
                NoOperation => self.clone(),
                // AddArithmetic1(two_registers) => self.manipulate_gr(two_registers, |r1, r2| r1 + r2),
                AddLogical1 => self.add_logical_1(r1, r2),
                SubtractLogical1 => self.subtract_logical_1(r1, r2),
                And1 => self.and_1(r1, r2),
                Or1 => self.or_1(r1, r2),
                Xor1 => self.xor_1(r1, r2),

                CompareArithmetic => {
                    let (r1, r2) = self.gr.get_arithmetic(r1, r2);

                    self.compare(r1, r2)
                }
                CompareLogical => {
                    let (r1, r2) = self.gr.get(r1, r2);

                    self.compare(r1, r2)
                }

                Pop => {
                    let r = self.mem.get(self.sp).unwrap();
                    let sp = self.sp + 1;
                    Machine {
                        sp,
                        ..self.mod_gr(r1, r)
                    }
                }
                Return => self.return_(),

                _ => unimplemented!(),
            },
            Either::Right(t) => Machine {
                previous_word: Some(t),
                ..self.clone()
            },
        })
    }
}

impl Machine {
    pub fn mem_info(&self) -> String {
        let mem = self.mem.0;
        let mem = &mem[..256];

        format!("bytes: {:X?}, length: {}", mem, mem.len())
    }

    /// PRが現在指示しているメモリの番地
    pub fn pr_at(&self) -> String {
        format!("{:X}", self.mem.get(self.pr).unwrap() as u16)
    }

    pub fn r_info(&self) -> String {
        format!("GR: {:X?}\nPR: {:X}SP: {:X}", self.gr, self.pr, self.sp,)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StepError {
    #[error("{0}")]
    ExecError(#[from] ExecError),
    #[error("{0}")]
    MemoryGetError(#[from] memory::GetError),
}

impl Machine {
    pub fn clock(&self) -> Result<Machine, StepError> {
        let word = self.mem.get(self.pr)?;
        let machine = self.exec(word as u16)?;

        Ok(Machine {
            pr: machine.pr + 16,
            ..machine
        })
    }
}

impl Machine {
    fn jump_to(&self, x: RegisterNumber, addr: u16, cond: bool) -> Machine {
        let effective_addr = self.get_effective_value(x, addr);

        if cond {
            Machine {
                pr: effective_addr,
                ..self.clone()
            }
        } else {
            self.clone()
        }
    }
    fn set_sf_zf(&self, value: u16) -> Machine {
        let sf = is_negative(value);
        let zf = value == 0;
        Machine {
            sf,
            zf,
            ..self.clone()
        }
    }
    fn mod_gr(&self, r1: RegisterNumber, r1_value: u16) -> Machine {
        let gr = self.gr.set(r1, r1_value);
        Machine { gr, ..self.clone() }
    }

    fn logical_1<F>(&self, r1: RegisterNumber, r2: RegisterNumber, f: F) -> Machine
    where
        F: FnOnce(u16, u16) -> (u16, bool),
    {
        // TODO フラグレジスタ
        let (r1_v, r2_v) = self.gr.get(r1, r2);
        let (r1_v, of) = f(r1_v, r2_v);

        Machine {
            of,
            ..self.mod_gr(r1, r1_v).set_sf_zf(r1_v)
        }
    }

    pub fn add_logical_1(&self, r1: RegisterNumber, r2: RegisterNumber) -> Machine {
        self.logical_1(r1, r2, u16::overflowing_add)
    }

    pub fn subtract_logical_1(&self, r1: RegisterNumber, r2: RegisterNumber) -> Machine {
        self.logical_1(r1, r2, u16::overflowing_sub)
    }

    fn bit_1<F>(&self, r1: RegisterNumber, r2: RegisterNumber, f: F) -> Machine
    where
        F: FnOnce(u16, u16) -> u16,
    {
        let (r1_v, r2_v) = self.gr.get(r1, r2);
        self.mod_gr(r1, f(r1_v, r2_v))
    }

    pub fn and_1(&self, r1: RegisterNumber, r2: RegisterNumber) -> Machine {
        self.bit_1(r1, r2, ops::BitAnd::bitand)
    }
    pub fn or_1(&self, r1: RegisterNumber, r2: RegisterNumber) -> Machine {
        self.bit_1(r1, r2, ops::BitOr::bitor)
    }
    pub fn xor_1(&self, r1: RegisterNumber, r2: RegisterNumber) -> Machine {
        self.bit_1(r1, r2, ops::BitXor::bitxor)
    }
    pub fn compare<T: cmp::Ord>(&self, a: T, b: T) -> Machine {
        let (sf, zf) = match a.cmp(&b) {
            cmp::Ordering::Greater => (false, false),
            cmp::Ordering::Equal => (false, true),
            cmp::Ordering::Less => (true, false),
        };
        Machine {
            sf,
            zf,
            ..self.clone()
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("MemoryAccessError")]
pub struct MemoryAccessError(#[from] memory::GetError);

impl Machine {
    fn return_(&self) -> Machine {
        // TODO resultにする
        let pr = self.mem.get(self.sp).unwrap();
        let sp = self.sp + 1;

        Machine {
            sp,
            pr,
            ..self.clone()
        }
    }
}

// #[test]
// fn test() -> Result<(), Box<dyn std::error::Error>> {
//     let machine = Machine::init(&mut io::Cursor::new(vec![0x24]));

//     Ok(())
// }
