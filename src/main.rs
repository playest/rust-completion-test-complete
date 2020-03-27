extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::prelude::*;
use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::Findable;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};

fn main() -> Ev3Result<()> {
    // the type of m is {unknown} with rust analyzer
    let m = LargeMotor::get(MotorPort::OutA)?; // .get(...) comes from trait Findable, along side .list()
    
    //LargeMotor::list(); // works

    // LargeMotor::li<ctrl + space> doesn't show "list" even if it exists

    //m.run_direct()?; // this methods exists, uncommting this line is ok
    // m.run<ctrl + space> doesn't show .run_direct(...) even if it exists

    Ok(())
}

