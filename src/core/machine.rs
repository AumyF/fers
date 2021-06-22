use super::{
    memory::{Memory, MemoryInitError},
    operations,
    operations::Operations,
};
use std::{error, fmt, io};

#[derive(Debug)]
pub enum MachineStepError {
    UnknownOperation(operations::OperationExecutionError),
    MemoryOutOfIndex(usize),
}

impl fmt::Display for MachineStepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MachineStepError::*;
        match self {
            MemoryOutOfIndex(n) => write!(f, "Memory out of index: {}", n),
            UnknownOperation(n) => write!(f, "Unknown operation: {}", n),
        }
    }
}
impl error::Error for MachineStepError {}

#[derive(Debug)]
pub enum MachineInitError {
    IOError(io::Error),
    OutOfMemory(usize),
}

impl fmt::Display for MachineInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MachineInitError::*;
        match self {
            IOError(e) => e.fmt(f),
            OutOfMemory(size) => write!(f, "Out of memory: {}", size),
        }
    }
}

impl error::Error for MachineInitError {}

pub struct Machine {
    pub mem: Memory,
    gr: [i16; 8],
    sp: i16,
    pr: i16,
    of: bool,
    sf: bool,
    zf: bool,
    second_word: Option<Operations>,
}

impl Machine {
    pub fn init(stream: &mut impl io::Read) -> Result<Machine, MachineInitError> {
        let mem = Memory::load_program(stream)
            .map_err(|MemoryInitError(e)| MachineInitError::IOError(e))?;
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
    /// ワードを読んで命令を実行する
    fn exec(self, word: u16) -> Result<Machine, operations::OperationExecutionError> {
        Operations::new(word).exec(self)
    }

    fn is_register_valid(num: u16) {}

    pub fn mem_info(&self) -> String {
        let Memory(mem) = self.mem;

        format!("bytes: {:X?}, length: {}", mem, mem.len())
    }

    /// PRが現在指示しているメモリの番地
    pub fn pr_at(&self) -> String {
        format!("{:X}", self.mem.get(self.pr as usize).unwrap() as u16)
    }

    pub fn clock(self) -> Result<Machine, MachineStepError> {
        use MachineStepError::*;
        let word = self.mem.get(self.pr as usize)?;
        let machine = self.exec(word as u16).map_err(|e| UnknownOperation(e))?;

        Ok(Machine {
            pr: machine.pr + 16,
            ..machine
        })
    }
}

#[derive(Debug)]
pub struct RegisterOutOfIndexError {
    index: u16,
}

impl fmt::Display for RegisterOutOfIndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Range of general registers is 0-7 but got: {}",
            self.index
        )
    }
}

impl Machine {
    pub fn get_gr(&self, index: u16) -> Result<&i16, RegisterOutOfIndexError> {
        self.gr
            .get(index as usize)
            .ok_or(RegisterOutOfIndexError { index })
    }

    pub fn manipulate_gr<F>(
        &self,
        r1_index: u16,
        r2_index: u16,
        f: F,
    ) -> Result<Machine, RegisterOutOfIndexError>
    where
        F: FnOnce(i16, i16) -> i16,
    {
        let r1_value = self.get_gr(r1_index)?;
        let r2_value = self.get_gr(r2_index)?;

        let mut gr = self.gr.clone();

        gr[r1_index as usize] = f(*r1_value, *r2_value);

        Ok(Machine {
            gr,
            mem: self.mem.clone(),
            second_word: self.second_word.clone(),
            ..*self
        })
    }
}
