use super::operations::{RegisterNumber, TwoRegisters};

#[derive(Debug, Clone, Copy)]
pub struct GeneralRegister([u16; 8]);

impl GeneralRegister {
    pub fn new(gr: [u16; 8]) -> GeneralRegister {
        GeneralRegister(gr)
    }
    /// インデックスレジスタ
    pub fn index(&self, two: TwoRegisters) -> IndexRegister {
        let (_, &num_of_index_register) = two.get_pair();
        IndexRegister(self.0[num_of_index_register as usize])
    }

    pub fn get(&self, r1: RegisterNumber, r2: RegisterNumber) -> (u16, u16) {
        (self.0[r1.0 as usize], self.0[r2.0 as usize])
    }

    /// 値をi16 (符号つき) として扱う．arithmeticな演算で使う
    pub fn get_arithmetic(&self, r1: RegisterNumber, r2: RegisterNumber) -> (i16, i16) {
        (self.0[r1.0 as usize] as i16, self.0[r2.0 as usize] as i16)
    }

    pub fn set(&self, r1: RegisterNumber, r1_value: u16) -> GeneralRegister {
        let mut gr = self.0;
        gr[r1.0 as usize] = r1_value;
        GeneralRegister(gr)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IndexRegister(u16);

impl IndexRegister {
    pub fn new(gr: u16) -> IndexRegister {
        IndexRegister(gr)
    }
    pub fn value(&self) -> u16 {
        self.0
    }
}
