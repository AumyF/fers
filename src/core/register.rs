use super::operations::TwoRegisters;

#[derive(Debug, Clone, Copy)]
pub struct GeneralRegister([u16; 8]);

impl GeneralRegister {
    pub fn new(gr: [u16; 8]) -> GeneralRegister {
        GeneralRegister(gr)
    }
    /// インデックスレジスタ
    pub fn index(&self, two: TwoRegisters) -> u16 {
        let (_, &num_of_index_register) = two.get_pair();
        self.0[num_of_index_register as usize]
    }

    pub fn get(&self, tw: TwoRegisters) -> (u16, u16) {
        let (&r1_r, &r2_x) = tw.get_pair();
        (self.0[r1_r as usize], self.0[r2_x as usize])
    }

    pub fn set(&self, tw: TwoRegisters, r1_value: u16) -> GeneralRegister {
        let mut gr = self.0;
        let (&r1_i, _) = tw.get_pair();
        gr[r1_i as usize] = r1_value;
        GeneralRegister(gr)
    }
}
