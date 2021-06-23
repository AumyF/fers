//! COMET2のメモリは1語16bitが65536語の1048576bitの128KiB。

use super::errors::MachineStepError;
use super::errors::MemoryInitError;
use crate::utils::to_pairs::ToPairBlanket;
use std::io;

fn u8u8_2_u16((x, y): (u8, u8)) -> i16 {
    let x = x as i16;
    let y = y as i16;
    (x << 8) + y
}

#[test]
fn test_u8u8_2_u16() {
    assert_eq!(u8u8_2_u16((0x7e, 0x80)), 0x7e80);
    assert_eq!(u8u8_2_u16((0xff, 0xff)), 0x7fff);
}

#[derive(Debug, Clone)]
pub struct Memory(pub [i16; 65536]);

impl Memory {
    pub fn load_program(stream: &mut impl io::Read) -> Result<Memory, MemoryInitError> {
        let mut mem = [0; 65536];

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).map_err(MemoryInitError)?;
        buf.into_iter()
            .to_pairs()
            .map(u8u8_2_u16)
            .enumerate()
            .for_each(|(index, value)| mem[index] = value);

        Ok(Memory(mem))
    }

    pub fn get(&self, idx: usize) -> Result<i16, MachineStepError> {
        let Memory(raw) = *self;
        if idx < raw.len() {
            Ok(raw[idx])
        } else {
            Err(MachineStepError::MemoryOutOfIndex(idx))
        }
    }
}
