//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;

#[no_mangle]
pub fn main() -> i32 {
    256
}

#[no_mangle]
pub fn update(system: &mut System) -> i32 {
    // system.logger.log(format_args!("App updated"));
    system.logger.log_str("App updated");
    666
}

#[no_mangle]
pub fn input(system: &mut System, input: InputType) -> i32 {
    system.logger.log_fmt(format_args!("Input recived: {:?}", input));
    // system.logger.log(format_args!("Recieved Input : {:?} ", input));
    666
}