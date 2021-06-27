//! COMET2のメモリは1語16bitが65536語の1048576bitの128KiB。

use super::machine;
use crate::utils::to_pairs::ToPairBlanket;
use std::io;

fn u8u8_2_u16((x, y): (u8, u8)) -> u16 {
    let x = x as u16;
    let y = y as u16;
    (x << 8) + y
}

#[test]
fn test_u8u8_2_u16() {
    assert_eq!(u8u8_2_u16((0x7e, 0x80)), 0x7e80);
    assert_eq!(u8u8_2_u16((0xff, 0xff)), 0xffff);
}

#[derive(Debug, Clone, Copy)]
pub struct Memory(pub [u16; 65536]);

#[derive(Debug, thiserror::Error)]
pub enum LoadProgramError {
    #[error("{0}")]
    IOError(#[from] io::Error),
}

impl Memory {
    pub fn load_program(stream: &mut impl io::Read) -> Result<Memory, LoadProgramError> {
        let mut mem = [0; 65536];

        let mut buf = Vec::new();
        let bytes = stream.read_to_end(&mut buf)?;
        buf.into_iter()
            .to_pairs()
            .map(u8u8_2_u16)
            .enumerate()
            .for_each(|(index, value)| {
                mem[index + machine::STACK_SIZE] = value;
            });

        Ok(Memory(mem))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
    #[error("GetError: {0} is out of index of memory")]
    OutOfIndex(usize),
}

impl Memory {
    pub fn get(&self, index: u16) -> Result<u16, GetError> {
        let index = index as usize;
        let Memory(raw) = *self;
        if index < raw.len() {
            Ok(raw[index])
        } else {
            Err(GetError::OutOfIndex(index))
        }
    }
    pub fn info(&self) -> String {
        let mem = self.0;
        let stack = &mem[..machine::STACK_SIZE];
        let mem = &mem[machine::STACK_SIZE..machine::STACK_SIZE + 256];

        format!(
            "Stack: {:X?}\nMem: {:X?}\nlength: {}",
            stack,
            mem,
            mem.len()
        )
    }
}
