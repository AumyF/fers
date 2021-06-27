use super::memory;
use super::operations::TwoWordOperations;
use super::register::GeneralRegister;
use super::{memory::Memory, operations, operations::Operations};
use std::ops::{BitAnd, BitOr, BitXor};
use std::rc::Rc;
use std::{io, vec};

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
    previous_word: Option<TwoWordOperations>,
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

impl Machine {
    fn exec(&self, word: u16) -> Result<Machine, ExecError> {
        use Operations::*;

        // 2語目
        if let Some(previous) = self.previous_word {
            use TwoWordOperations::*;
            return Ok(match previous {
                Load(tr) => {
                    // TODO 処理が分散してて汚い
                    let (_, &index_gr_num) = tr.get_pair();

                    let adr = word + self.gr.index(tr);
                    self.load2(tr, adr)
                }
                Push(tr) => {
                    let (_, offset) = self.gr.get(tr);
                    let (_, &index) = tr.get_pair();
                    let sp = self.sp - 1;
                    let mut mem = self.mem.0.clone();
                    mem[self.sp as usize] = word + offset;
                    let mem = Rc::new(Memory(mem));
                    Machine {
                        sp,
                        mem,
                        ..self.clone()
                    }
                }

                Call(tr) => {
                    let (_, xv) = self.gr.get(tr);
                    let adr = word + xv;
                    self.call(adr)
                }
                _ => unimplemented!(),
            });
        }

        // 1語目
        let operation = Operations::new(word)?;

        Ok(match operation {
            NoOperation => self.clone(),
            // AddArithmetic1(two_registers) => self.manipulate_gr(two_registers, |r1, r2| r1 + r2),
            AddLogical1(two_registers) => self.add_logical_1(two_registers),
            SubtractLogical1(tr) => self.subtract_logical_1(tr),
            And1(tr) => self.and_1(tr),
            Or1(two_registers) => self.or_1(two_registers),
            Xor1(tr) => self.xor_1(tr),
            Pop(tr) => {
                let r = self.mem.get(self.sp).unwrap();
                let sp = self.sp + 1;
                Machine {
                    sp,
                    ..self.mod_gr(tr, r)
                }
            }
            Return => self.return_(),

            _ => unimplemented!(),
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
    fn mod_gr(&self, two_registers: operations::TwoRegisters, r1_value: u16) -> Machine {
        let gr = self.gr.set(two_registers, r1_value);
        Machine { gr, ..self.clone() }
    }

    fn logical_1<F>(&self, two_registers: operations::TwoRegisters, f: F) -> Machine
    where
        F: FnOnce(u16, u16) -> Option<u16>,
    {
        // TODO フラグレジスタ
        let (r1, r2) = self.gr.get(two_registers);
        let machine = f(r1, r2).map_or(self.clone(), |r1| self.mod_gr(two_registers, r1));

        machine
    }

    pub fn add_logical_1(&self, two_registers: operations::TwoRegisters) -> Machine {
        self.logical_1(two_registers, u16::checked_add)
    }

    pub fn subtract_logical_1(&self, two_registers: operations::TwoRegisters) -> Machine {
        self.logical_1(two_registers, u16::checked_sub)
    }

    pub fn and_1(&self, two_registers: operations::TwoRegisters) -> Machine {
        self.logical_1(two_registers, |r1, r2| Some(r1 & r2))
    }
    pub fn or_1(&self, two_registers: operations::TwoRegisters) -> Machine {
        self.logical_1(two_registers, |r1, r2| Some(r1 | r2))
    }
    pub fn xor_1(&self, two_registers: operations::TwoRegisters) -> Machine {
        self.logical_1(two_registers, |r1, r2| Some(r1 ^ r2))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("MemoryAccessError")]
pub struct MemoryAccessError(#[from] memory::GetError);

impl Machine {
    fn load2(&self, two_registers: operations::TwoRegisters, adr: u16) -> Machine {
        self.mod_gr(two_registers, self.mem.get(adr).unwrap())
    }
    fn call(&self, adr: u16) -> Machine {
        let sp = self.sp - 1;
        // TODO メモリ操作が荒い
        let mut mem = self.mem.0.clone();
        mem[sp as usize] = self.pr;
        let mem = Rc::new(Memory(mem));
        let pr = adr;

        Machine {
            sp,
            mem,
            pr,
            ..self.clone()
        }
    }
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
