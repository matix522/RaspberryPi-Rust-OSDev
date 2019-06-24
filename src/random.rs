use super::MMIO_BASE;
use register::{mmio::*,register_bitfields};
use core::ops::Deref;
use crate::utils::asm;
register_bitfields! {
    u32,
    CONTROL [
        ENABLE OFFSET(0) NUMBITS(1) [
            True = 1,
            False = 0
        ]
    ],
    INT_MASK [
        INT_OFF  OFFSET(0) NUMBITS(1) [
            True = 1,
            False = 0
        ]
    ]
}

const RANDOM_NUMER_GENERATOR_BASE: u32 = MMIO_BASE + 0x104_000;
const RANDOM_NUMER_GENERATOR_WARMUP_COUNT: u32 = 0x40_000;

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    CONTROL: ReadWrite<u32, CONTROL::Register>,         // 0x00
    STATUS: ReadWrite<u32>,                       // 0x04
    DATA: ReadOnly<u32>,                          // 0x08
    __reserved_0: u32,                            // 0x0c
    INT_MASK: ReadWrite<u32, INT_MASK::Register>, // 0x10
}

pub struct RandomNumberGenerator;

impl Deref for RandomNumberGenerator {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

impl RandomNumberGenerator {
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator
    }

    /// Returns a pointer to the register block
    fn ptr() -> *const RegisterBlock {
        RANDOM_NUMER_GENERATOR_BASE as *const _
    }

    /// Initialize the RNG
    pub fn init(&self) {
        // Disable interrupts
        self.INT_MASK.modify(INT_MASK::INT_OFF::True);

        // Set warm-up count and enable
        self.STATUS.set(RANDOM_NUMER_GENERATOR_WARMUP_COUNT);
        self.CONTROL.modify(CONTROL::ENABLE::True);
    }

    /// Return a random number between [min..max]
    pub fn rand(&self, min: i32, max: i32) -> i32 {
        // wait for gaining some entropy
        loop {
            if (self.STATUS.get() >> 24) != 0 {
                break;
            }
            asm::nop();
        }

        let mut r = self.DATA.get() as i32;
        if r < 0 {
            r = -r;
        }
        r % (max - min) + min
    }
}
