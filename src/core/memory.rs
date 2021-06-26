//! COMET2のメモリは1語16bitが65536語の1048576bitの128KiB。

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
    assert_eq!(u8u8_2_u16((0xff, 0xff)), 0x7fff);
}

#[derive(Debug, Clone)]
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
        stream.read_to_end(&mut buf)?;
        buf.into_iter()
            .to_pairs()
            .map(u8u8_2_u16)
            .enumerate()
            .for_each(|(index, value)| mem[index] = value);

        Ok(Memory(mem))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
    #[error("GetError: {0} is out of index of memory")]
    OutOfIndex(usize),
}

impl Memory {
    pub fn get(&self, idx: usize) -> Result<u16, GetError> {
        let Memory(raw) = *self;
        if idx < raw.len() {
            Ok(raw[idx])
        } else {
            Err(GetError::OutOfIndex(idx))
        }
    }
}
