use crate::utils::to_pairs::ToPairBlanket;
use std::io;

use super::machine::MachineStepError;
#[derive(Debug)]
pub struct Memory(pub [i16; 4096]);

pub struct MemoryInitError(pub io::Error);

fn u8u8_2_u16((x, y): (u8, u8)) -> i16 {
    let x = x as i16;
    let y = y as i16;
    (x << 8) + y
}

impl Memory {
    pub fn load_program(stream: &mut impl io::Read) -> Result<Memory, MemoryInitError> {
        let mut mem = [0; 4096];

        let mut buf = Vec::new();
        let rea = stream.read_to_end(&mut buf).map_err(MemoryInitError)?;
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
