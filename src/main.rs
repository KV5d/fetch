// auth: propagate
// 5/24/22

extern crate systemstat;
extern crate colored;

use crate::systemstat::System;
use crate::systemstat::Platform;
use crate::systemstat::saturating_sub_bytes;

use colored::Colorize;

fn main() {
    if cfg!(windows){
        
    } else if cfg!(unix) {
        println!("please use windows");
        std::process::exit(0x0100);
    }
    
    let sys = System::new();

    print!("{} {}", "Operating System:".white(), "⊞".blue());

    match sys.memory() {
        Ok(mem) => print!("\nMemory: {} used / {}", saturating_sub_bytes(mem.total, mem.free), mem.total),
        Err(x) => println!("\nMemory: error: {}", x)
    }

}

