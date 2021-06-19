use super::machine::Machine;

pub enum Ops {
    NoOperation,
    Load2,
    Store,
    LoadAddress,
    Load1,
    AddArithmetic1(AddArithmetic1),
    SubtractArithmetic,
    AddLogical,
    SubtratLogical,
}

trait Operation {
    fn code() -> u16;
    fn exec(self, machine: Machine) -> Result<Machine, OperationExecutionExeption>;
}

struct OperationNotDefined {
    word: u16,
}

// fn word_to_operation(word: u16) -> Result<impl Operation, OperationNotDefined> {
//     match word & 0x1100 {
//         0 => Ok(NoOperation {}), // NOP
//         0x1000 => Ok(Machine { ..self }),
//         _ => Err(ExecError {}),
//     }
// }

trait OneLength {}

pub trait TwoLengthOperation {}

struct NoOperation {}

impl Operation for NoOperation {
    fn code() -> u16 {
        0x0000
    }
    fn exec(self, machine: Machine) -> Result<Machine, OperationExecutionExeption> {
        Ok(machine)
    }
}

struct AddArithmetic1 {
    r1: (u8, u16),
    r2: (u8, u16),
}

enum OperationExecutionExeption {
    RegisterOutOfIndex(u8),
}

// impl Operation for AddArithmetic1 {
//     fn code() -> u16 {
//         0x2400
//     }
//     fn exec(self, machine: Machine) -> Result<Machine, OperationExecutionExeption> {
//         // let gr_a = machine
//         //     .gr
//         //     .get(self.r1.0 as usize)
//         //     .ok_or(OperationExecutionExeption::RegisterOutOfIndex(self.r1.0))?;
//         // let gr_b = machine
//         //     .gr
//         //     .get(self.r2.0 as usize)
//         //     .ok_or(OperationExecutionExeption::RegisterOutOfIndex(self.r2.0))?;

//         // for elem in machine.gr.iter_mut() {
//         //     elem.
//         // }

//         Ok(Machine {
//             gr: [..machine.gr, 3],
//             ..machine
//         })
//     }
// }
