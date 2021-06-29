use std::{error::Error, thread::sleep, time::Duration};
mod core;
mod lib;
mod utils;

use crate::core::machine::Machine;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut code = std::fs::File::open("sample")?;
    let mut code = std::io::Cursor::new(vec![0x20, 0x01]);
    let machine = Machine::init(&mut code)?;
    println!("{}", machine.r_info());
    println!("{}", machine.mem.0[0]);
    machine.clock()?;

    println!("{}", machine.r_info());
    println!("{}", machine.mem.0[0]);
    println!("{}", machine.mem.info());
    Ok(())
}

#[test]
fn t() {
    assert_eq!(-1i8 as u8, 255);
    assert_eq!(255u8 as i8, -1);
}
