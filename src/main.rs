use std::error::Error;
mod core;
mod lib;
mod utils;

use crate::core::machine::Machine;

fn main() -> Result<(), Box<dyn Error>> {
    let mut code = std::fs::File::open("sample")?;
    let machine = Machine::init(&mut code)?;

    let machine = machine
        .clock()?
        .clock()?
        .clock()?
        .clock()?
        .clock()?
        .clock()?;

    println!("{}", machine.pr_at());
    Ok(())
}

#[test]
fn t() {
    assert_eq!(-1i8 as u8, 255)
}
