use super::memory;
use super::{memory::Memory, operations, operations::Operations};
use std::io;

#[derive(Debug, Clone)]
pub struct Machine {
    pub mem: Memory,
    gr: [u16; 8],
    sp: u16,
    pr: u16,
    of: bool,
    sf: bool,
    zf: bool,
    second_word: Option<Operations>,
}

#[derive(Debug, thiserror::Error)]
pub enum MachineInitError {
    #[error("{0}")]
    MemoryInitFailed(#[from] memory::LoadProgramError),
}

impl Machine {
    pub fn init(stream: &mut impl io::Read) -> Result<Machine, MachineInitError> {
        let mem = Memory::load_program(stream)?;
        Ok(Machine {
            mem,
            gr: [0; 8],
            sp: 0,
            pr: 0,
            of: false,
            sf: false,
            zf: false,

            second_word: None,
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

        let operation = Operations::new(word)?;
        Ok(match operation {
            NoOperation => self.clone(),
            AddArithmetic1(two_registers) => self.manipulate_gr(two_registers, |r1, r2| r1 + r2),

            _ => unimplemented!(),
        })
    }
}

impl Machine {
    /// ワードを読んで命令を実行する

    pub fn mem_info(&self) -> String {
        let Memory(mem) = self.mem;

        format!("bytes: {:X?}, length: {}", mem, mem.len())
    }

    /// PRが現在指示しているメモリの番地
    pub fn pr_at(&self) -> String {
        format!("{:X}", self.mem.get(self.pr as usize).unwrap() as u16)
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
    pub fn clock(self) -> Result<Machine, StepError> {
        let word = self.mem.get(self.pr as usize)?;
        let machine = self.exec(word as u16)?;

        Ok(Machine {
            pr: machine.pr + 16,
            ..machine
        })
    }
}

impl Machine {
    pub fn get_gr(&self, index: u16) -> Result<&u16, u16> {
        self.gr.get(index as usize).ok_or(index)
    }

    /// `TwoRegisters` を使ってGRにアクセスする。
    fn get_grs(&self, two_registers: operations::TwoRegisters) -> (&u16, &u16) {
        let (&r1, &r2) = two_registers.get_pair();
        (self.get_gr(r1).unwrap(), self.get_gr(r2).unwrap())
    }

    pub fn manipulate_gr<F>(&self, two_registers: operations::TwoRegisters, f: F) -> Machine
    where
        F: FnOnce(u16, u16) -> u16,
    {
        let (&r1_index, _) = two_registers.get_pair();

        let (r1_value, r2_value) = self.get_grs(two_registers);

        let mut gr = self.gr.clone();

        gr[r1_index as usize] = f(*r1_value, *r2_value);

        Machine {
            gr,
            mem: self.mem.clone(),
            second_word: self.second_word.clone(),
            ..*self
        }
    }
}
