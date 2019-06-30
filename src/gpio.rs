use crate::memory::physical::GPIO_BASE;
use register::{mmio::ReadWrite, register_bitfields};
// 0x 7E20 0000 GPFSEL0 GPIO Function Select 0 32 R/W
// 0x 7E20 0000 GPFSEL0 GPIO Function Select 0 32 R/W
// 0x 7E20 0004 GPFSEL1 GPIO Function Select 1 32 R/W
// 0x 7E20 0008 GPFSEL2 GPIO Function Select 2 32 R/W
// 0x 7E20 000C GPFSEL3 GPIO Function Select 3 32 R/W
// 0x 7E20 0010 GPFSEL4 GPIO Function Select 4 32 R/W
// 0x 7E20 0014 GPFSEL5 GPIO Function Select 5 32 R/W
// 0x 7E20 0018 - Reserved - -
// 0x 7E20 001C GPSET0 GPIO Pin Output Set 0 32 W
// 0x 7E20 0020 GPSET1 GPIO Pin Output Set 1 32 W
// 0x 7E20 0024 - Reserved - -
// 0x 7E20 0028 GPCLR0 GPIO Pin Output Clear 0 32 W
// 0x 7E20 002C GPCLR1 GPIO Pin Output Clear 1 32 W
// 0x 7E20 0030 - Reserved - -
// 0x 7E20 0034 GPLEV0 GPIO Pin Level 0 32 R
// 0x 7E20 0038 GPLEV1 GPIO Pin Level 1 32 R
// 0x 7E20 003C - Reserved - -
// 0x 7E20 0040 GPEDS0 GPIO Pin Event Detect Status 0 32 R/W
// 0x 7E20 0044 GPEDS1 GPIO Pin Event Detect Status 1 32 R/W
// 0x 7E20 0048 - Reserved - -
// 0x 7E20 004C GPREN0 GPIO Pin Rising Edge Detect Enable 0 32 R/W
// 0x 7E20 0050 GPREN1 GPIO Pin Rising Edge Detect Enable 1 32 R/W
// 0x 7E20 0054 - Reserved - -
// 0x 7E20 0058 GPFEN0 GPIO Pin Falling Edge Detect Enable 0 32 R/W
// 0x 7E20 005C GPFEN1 GPIO Pin Falling Edge Detect Enable 1 32 R/W
// 0x 7E20 0060 - Reserved - -
// 0x 7E20 0064 GPHEN0 GPIO Pin High Detect Enable 0 32 R/W
// 0x 7E20 0068 GPHEN1 GPIO Pin High Detect Enable 1 32 R/W
// 0x 7E20 006C - Reserved - -
// 0x 7E20 0070 GPLEN0 GPIO Pin Low Detect Enable 0 32 R/W
// 0x 7E20 0074 GPLEN1 GPIO Pin Low Detect Enable 1 32 R/W
// 0x 7E20 0078 - Reserved - -
// 0x 7E20 007C GPAREN0 GPIO Pin Async. Rising Edge Detect 0 32 R/W
// 0x 7E20 0080 GPAREN1 GPIO Pin Async. Rising Edge Detect 1 32 R/W
// 0x 7E20 0084 - Reserved - -
// 0x 7E20 0088 GPAFEN0 GPIO Pin Async. Falling Edge Detect 0 32 R/W
// 0x 7E20 008C GPAFEN1 GPIO Pin Async. Falling Edge Detect 1 32 R/W
// 0x 7E20 0090 - Reserved - -
// 0x 7E20 0094 GPPUD GPIO Pin Pull-up/down Enable 32 R/W
// 0x 7E20 0098 GPPUDCLK0 GPIO Pin Pull-up/down Enable Clock 0 32 R/W
// 0x 7E20 009C GPPUDCLK1 GPIO Pin Pull-up/down Enable Clock 1 32 R/W
// 0x 7E20 00A0 - Reserved - -
// 0x 7E20 00B0 - Test 4 R/W

register_bitfields! {
    u32,
    /// GPIO Fuction Select 1

    GPIO_FUCNTION_SELECT_1 [
        PIN_19 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],
        PIN_18 OFFSET(24) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],
        PIN_17 OFFSET(21) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],
        PIN_16 OFFSET(18) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],
        PIN_15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            RXD0 = 0b100, // UART0     - Alternate function 0
            RXD1 = 0b010  // Mini UART - Alternate function 5

        ],
        PIN_14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            TXD0 = 0b100, // UART0     - Alternate function 0
            TXD1 = 0b010  // Mini UART - Alternate function 5
        ],
        PIN_13 OFFSET(9) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],
        PIN_12 OFFSET(6) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],
        PIN_11 OFFSET(3) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],
        PIN_10 OFFSET(0) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ]
    ],
    /// GPIO Pull-up/down Clock Register 0
    GPIO_PULL_UP_DOWN_CLOCK_0 [
        PIN_15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],
        PIN_14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ]
}

pub struct Pin(u8);

impl Pin {
    fn new() -> () {

    }
}

pub const GPIO_FUCNTION_SELECT_1: *const ReadWrite<u32, GPIO_FUCNTION_SELECT_1::Register> =
    (GPIO_BASE + 0x0000_0004) as *const ReadWrite<u32, GPIO_FUCNTION_SELECT_1::Register>;

pub const GPIO_PULL_UP_DOWN: *const ReadWrite<u32> =
    (GPIO_BASE + 0x0000_0094) as *const ReadWrite<u32>;

pub const GPIO_PULL_UP_DOWN_CLOCK_0: *const ReadWrite<u32, GPIO_PULL_UP_DOWN_CLOCK_0::Register> =
    (GPIO_BASE + 0x0000_0098) as *const ReadWrite<u32, GPIO_PULL_UP_DOWN_CLOCK_0::Register>;
