#![no_std]
#![no_main]
#![feature(asm)]
//extern crate alloc;

mod gpio;
mod io;
mod kernel;
mod memory;
mod random;
mod utils;

use io::{uart, Read, Write};
use kernel::KernelBuilder;
use utils::asm;

#[no_mangle]
fn kernel_setup() -> ! {
    let uart = uart::MiniUart::new();
    uart.init();
    uart.put_string("UART: Cannot use println!\n");
    let kernel = KernelBuilder::new()
        .with_stdio(uart)
        .of_name("BROS - Battle Royale Operating System")
        .version(0, 0, 1)
        .build();
    let k = kernel::get_kernel_ref();
    uart.put_string("UART: Can use println!\n");
    println!("println: {}", " Hello from println!");
    panic!();
    //k.get_stdio().put_string(k.get_name());
    let version = k.get_version();
    println!(
        "{} - version {}.{}.{}",
        k.get_name(),
        version.0,
        version.1,
        version.2
    );
    loop {
        println!("Enter a number: ");
        match scanln!(f64).0 {
            Some(f) => println!("You have entered: {}", f),
            _ => println!("that was not a number :("),
        }
    }
    /*
    utils::delay(1000);
    uart.puts("\n first line\n");
        utils::delay(1000);
    uart.puts("\n second line\n");

    loop {
        let c = uart.getc();
        let c1 = (c as u8 + 2) as char;
        uart.send(c);
        utils::delay(100);
        uart.send(c);
        utils::delay(100);
        uart.send(c1);
        utils::delay(100);
        uart.send(c1);
    }*/
    /*
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
    }*/
}

raspi3_boot::entry!(kernel_setup);
