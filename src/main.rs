#![no_std]
#![no_main]
#![feature(asm)]

//extern crate alloc;
#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate spin;


const MMIO_BASE: u32 = 0x3F00_0000;

mod gpio;
mod uart;
mod utils;
mod random;

fn kernel_entry() -> ! {

    println!("BROS - Battle Royale Operating System"); 

    println!("Working at exception level: {:?}", utils::get_excception_level());

    let rand = random::RandomNumberGenerator::new();
    rand.init();

    println!("Currently randomizing nummbers!");

    loop {
        println!("Enter parameters for randomisation <min> <max> <count>: ");
        let (min, max, count) = scanln!(i32, i32, u32);

        if min.is_some() && max.is_some() && count.is_some(){

            let min = min.unwrap();
            let max = max.unwrap();
            let count = count.unwrap();

            if min > max {
                println!("Min cannot be more than max.");
                continue;
            }
            for i in 0..count {
                println!("{}", rand.rand(min, max));
            }
        }
        else {
            println!("Incorect input format.");
        }
    }
}

raspi3_boot::entry!(kernel_entry);
