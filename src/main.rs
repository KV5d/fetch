// auth: propagate
// 5/24/22

#[allow(unused_imports)]
#[allow(unused_variables)]
use colored::Colorize;
use sysinfo::{System, SystemExt, CpuExt, ComponentExt};
use std::option::Option;

fn main() {
    if cfg!(windows){
        
    } else if cfg!(unix) {
        println!("please use windows");
        std::process::exit(0x0100);
    }
    
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{} {}", "Operating System:".white(), "⊞".blue());

    let used_mem = sys.used_memory() / 1000000000;
    let total_mem = sys.total_memory() / 1000000000;

    println!("{} {:?}GB / {:?}GB", "RAM:", used_mem, total_mem);

    for cpu in sys.cpus() {
        print!("{:?} {:?} | {:?}%", cpu.brand(), cpu.name(), cpu.cpu_usage()); 
    }

    for component in sys.components() {
        print!(" TEMP: {}°C", component.temperature());
    }

    

}

