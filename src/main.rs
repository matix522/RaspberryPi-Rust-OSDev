#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![allow(dead_code)]
#![feature(naked_functions)]
#![feature(const_fn)]

//extern crate alloc;

mod gpio;
mod io;
mod kernel;
mod memory;
mod random;
mod utils;
#[macro_use]
mod exception;

use io::{uart, Read, Write};
use kernel::KernelBuilder;
use utils::asm;
//use interupt::interuptHandler;

#[no_mangle]
fn kernel_setup() -> ! {

    extern "C" {
        static __exception_vectors_start: u64;
    }

    let uart = uart::MiniUart::new();
    uart.init();
    let kernel = KernelBuilder::new()
        .with_stdio(uart)
        .of_name("\x1B[36mBROS - Battle Royale Operating System\x1B[0m Fortnite Edition")
        .version(0, 0, 1)
        .build();
    let k = kernel::get_kernel_ref();
    let version = k.get_version();
    println!("\x1B[2J\x1B[1;1H");

    println!(
        "{} - version {}.{}.{}",
        k.get_name(),
        version.0,
        version.1,
        version.2
    );
    if unsafe {
        let exception_vectors_start: u64 = &__exception_vectors_start as *const _ as u64;

        exception::set_vbar_el1_checked(exception_vectors_start)
    } {
        //println!("[5] Exception vectors are set up.");
    } else {
        println!("[!][Error] Error setting exception vectors.");

    }
    println!("Working at exception level: {:?}", utils::get_excception_level());

    let rand = random::RandomNumberGenerator::new();
    rand.init();
    //println!("Currently randomizing nummbers!");
    //println!("Before Exception");
   /* let big_addr: u64 = 3 * 1024 * 1024 * 1024;
    unsafe { core::ptr::read_volatile(big_addr as *mut u64) };*/
    //println!("After Exception");
    //println!("{}", "Ten tekst tylko ze bez bialych znakow " );
    exception::timer::ArmQemuTimer::enable();
    exception::timer::ArmQemuTimer::interupt_after(exception::timer::ArmQemuTimer::get_frequency());

    exception::timer::SystemTimer::enable();

    loop {
        println!("Enter parameters for randomisation <min> <max> <count>: ");
        match scanln!(i32, i32, u32) {
            (Some(min),Some(max), Some(count)) if min <= max => {
                for i in 0..count {
                    println!("{}", rand.rand(min, max));
                }
            },
            (Some(_), Some(_), Some(_))  => println!("Value of max must be at least as large as min."),
            _ => println!("Incorect input format.")
        }
    }

}

raspi3_boot::entry!(kernel_setup);