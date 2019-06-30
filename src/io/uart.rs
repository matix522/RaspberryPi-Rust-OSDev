
use crate::memory::physical::MINI_UART_BASE;
use crate::gpio;
use core::ops;
use register::{mmio::*, register_bitfields};
use core::fmt;

use crate::utils::*;
use super::*;
// Auxilary mini UART registers

register_bitfields! {
    u32,

    /// Auxiliary enables
    AUX_ENABLES [
        /// If set the mini UART is enabled. The UART will immediately
        /// start receiving data, especially if the UART1_RX line is
        /// low.
        /// If clear the mini UART is disabled. That also disables any
        /// mini UART register access
        MINI_UART_ENABLE OFFSET(0) NUMBITS(1) []
    ],

    /// Mini Uart Interrupt Identify
    AUX_MU_IIR [
        /// Writing with bit 1 set will clear the receive FIFO
        /// Writing with bit 2 set will clear the transmit FIFO
        FIFO_CLEAR OFFSET(1) NUMBITS(2) [
            Rx = 0b01,
            Tx = 0b10,
            All = 0b11
        ]
    ],

    /// Mini Uart Line Control
    AUX_MU_LCR [
        /// Mode the UART works in
        DATA_SIZE OFFSET(0) NUMBITS(2) [
            SevenBit = 0b00,
            EightBit = 0b11
        ]
    ],

    /// Mini Uart Line Status
    AUX_MU_LSR [
        /// This bit is set if the transmit FIFO can accept at least
        /// one byte.
        TX_EMPTY   OFFSET(5) NUMBITS(1) [],

        /// This bit is set if the receive FIFO holds at least 1
        /// symbol.
        DATA_READY OFFSET(0) NUMBITS(1) []
    ],

    /// Mini Uart Extra Control
    AUX_MU_CNTL [
        /// If this bit is set the mini UART transmitter is enabled.
        /// If this bit is clear the mini UART transmitter is disabled.
        TX_EN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// If this bit is set the mini UART receiver is enabled.
        /// If this bit is clear the mini UART receiver is disabled.
        RX_EN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Mini Uart Baudrate
    AUX_MU_BAUD [
        /// Mini UART baudrate counter
        RATE OFFSET(0) NUMBITS(16) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    __reserved_0: u32,                                  // 0x00
    AUX_ENABLES: ReadWrite<u32, AUX_ENABLES::Register>, // 0x04
    __reserved_1: [u32; 14],                            // 0x08
    AUX_MU_IO: ReadWrite<u32>,                          // 0x40 - Mini Uart I/O Data
    AUX_MU_IER: WriteOnly<u32>,                         // 0x44 - Mini Uart Interrupt Enable
    AUX_MU_IIR: WriteOnly<u32, AUX_MU_IIR::Register>,   // 0x48
    AUX_MU_LCR: WriteOnly<u32, AUX_MU_LCR::Register>,   // 0x4C
    AUX_MU_MCR: WriteOnly<u32>,                         // 0x50
    AUX_MU_LSR: ReadOnly<u32, AUX_MU_LSR::Register>,    // 0x54
    __reserved_2: [u32; 2],                             // 0x58
    AUX_MU_CNTL: WriteOnly<u32, AUX_MU_CNTL::Register>, // 0x60
    __reserved_3: u32,                                  // 0x64
    AUX_MU_BAUD: WriteOnly<u32, AUX_MU_BAUD::Register>, // 0x68
}
#[derive(Copy,Clone)]
pub struct MiniUart;

impl ops::Deref for MiniUart {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl MiniUart {
    pub fn new() -> MiniUart {
        MiniUart
    }

    /// Returns a pointer to the register block
    fn ptr() -> *const RegisterBlock {
        MINI_UART_BASE as *const _
    }

    ///Set baud rate and characteristics (115200 8N1) and map to GPIO
    pub fn init(&self) {
        // initialize UART
        self.AUX_ENABLES.modify(AUX_ENABLES::MINI_UART_ENABLE::SET);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_CNTL.set(0);
        self.AUX_MU_LCR.write(AUX_MU_LCR::DATA_SIZE::EightBit);
        self.AUX_MU_MCR.set(0);
        self.AUX_MU_IER.set(0);
        self.AUX_MU_IIR.write(AUX_MU_IIR::FIFO_CLEAR::All);
        self.AUX_MU_BAUD.write(AUX_MU_BAUD::RATE.val(270)); // 115200 baud

        // map UART1 to GPIO pins
        unsafe {
            (*gpio::GPIO_FUCNTION_SELECT_1)
                .modify(
                    gpio::GPIO_FUCNTION_SELECT_1::PIN_14::TXD1
                  + gpio::GPIO_FUCNTION_SELECT_1::PIN_15::RXD1);

            (*gpio::GPIO_PULL_UP_DOWN).set(0); // enable pins 14 and 15
            
            delay(150);

            (*gpio::GPIO_PULL_UP_DOWN_CLOCK_0)
                .write(
                    gpio::GPIO_PULL_UP_DOWN_CLOCK_0::PIN_14::AssertClock
                  + gpio::GPIO_PULL_UP_DOWN_CLOCK_0::PIN_15::AssertClock,
            );

            delay(150);

            (*gpio::GPIO_PULL_UP_DOWN_CLOCK_0).set(0);
        }

        self.AUX_MU_CNTL
            .write(AUX_MU_CNTL::RX_EN::Enabled + AUX_MU_CNTL::TX_EN::Enabled);
    }
}


impl Read for MiniUart {

    /// Receive a character
    fn get_char(&self) -> Option<char> {
        // wait until something is in the buffer
        loop {
            if self.AUX_MU_LSR.is_set(AUX_MU_LSR::DATA_READY) {
                break;
            }
            asm::nop();
        }

        // read it and return
        let mut ret = self.AUX_MU_IO.get() as u8 as char;

        // convert carrige return to newline
        if ret == '\r' {
            ret = '\n'
        }

        Some(ret)
    }
    fn get_line(&self) -> (usize, [u8;128]) {
        let mut s : [u8;128]= [10; 128];
        for i in 0..127 {
            s[i] = self.get_char().unwrap() as u8;
            self.put_char(s[i] as char).unwrap();
            if s[i] == 10 {
                return (i,s)
            }
        }
        self.put_char('\n').unwrap();
        return (127,s)
    }


}
impl Write for MiniUart {
     /// Send a character
    fn put_char(&self, c: char) -> Result<(),WriteError> {
        // wait until we can send
        loop {
            if self.AUX_MU_LSR.is_set(AUX_MU_LSR::TX_EMPTY) {
                break;
            }
            asm::nop();
        }

        // write the character to the buffer
        self.AUX_MU_IO.set(c as u32);
        Ok(())
    }
        /// Display a string
    fn put_string(&self, string: &str)-> Result<(),WriteError> {
        for c in string.chars() {
            crate::utils::delay(100);
            // convert newline to carrige return + newline
            if c == '\n' {
                self.put_char('\r')?;
            }

            self.put_char(c)?;
        }
        Ok(())
    }

}
