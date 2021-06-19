trait Register {
    fn isIndexRegister(&self) -> bool;
}

struct GeneralRegister([i16; 8]);

struct StackPointer(u16);

struct ProgramRegister(u16);

struct IndexRegister {
    value: i16,
}

struct FlagRegister(i16, i16, i16);

impl Register for IndexRegister {
    fn isIndexRegister(&self) -> bool {
        true
    }
}
