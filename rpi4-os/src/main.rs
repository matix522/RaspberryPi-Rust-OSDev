#![no_std]
#![no_main]
#![feature(asm)]
mod gpio;
mod mbox;
mod uart;
mod io;
const MMIO_BASE: u32 = 0xFE00_0000;


use core::sync::atomic::{compiler_fence, Ordering, fence};

use io::UART as uart;

fn kernel_entry() -> ! {
    let mut mbox = mbox::Mbox::new();

    // set up serial console
    match uart.init(&mut mbox) {
        Ok(_) => uart.puts("\n[0] UART is live!\n"),
        Err(_) => loop {
            unsafe { asm!("wfe" :::: "volatile") }; // If UART fails, abort early
        },
    }

    uart.puts("[1] Press a key to continue booting... ");
    uart.getc();
    uart.puts("Greetings fellow Rustacean!\n");

    // get the board's unique serial number with a mailbox call
    mbox.buffer[0] = 8 * 4; // length of the message
    mbox.buffer[1] = mbox::REQUEST; // this is a request message
    mbox.buffer[2] = mbox::tag::GETSERIAL; // get serial number command
    mbox.buffer[3] = 8; // buffer size
    mbox.buffer[4] = 8;
    mbox.buffer[5] = 0; // clear output buffer
    mbox.buffer[6] = 0;
    mbox.buffer[7] = mbox::tag::LAST;

    // Insert a compiler fence that ensures that all stores to the
    // mbox buffer are finished before the GPU is signaled (which is
    // done by a store operation as well).
    fence(Ordering::Release);

    // send the message to the GPU and receive answer
    let serial_avail = match mbox.call(mbox::channel::PROP) {
        Err(_) => false,
        Ok(()) => true,
    };

    if serial_avail {
        uart.puts("[i] My serial number is: 0x");
        uart.hex(mbox.buffer[6]);
        uart.hex(mbox.buffer[5]);
        uart.puts("\n");
    } else {
        uart.puts("[i] Unable to query serial!\n");
    }

    // echo everything back
    loop {
        uart.send(uart.getc());
    }
}
// fn kernel_entry() -> ! {
//     gpio::setup();
//     gpio::blink();
// }

boot::entry!(kernel_entry);