use super::operations;
use std::{error, fmt, io};

#[derive(Debug)]
pub struct RegisterOutOfIndexError {
    pub index: u16,
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

#[derive(Debug)]
pub enum MachineStepError {
    UnknownOperation(OperationExecutionError),
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

pub struct MemoryInitError(pub io::Error);

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
