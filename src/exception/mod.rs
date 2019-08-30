use crate::println;

use crate::memory::physical::MMIO_BASE;
use register::{mmio::*,register_bitfields};
use core::ops;

pub mod timer;

use timer::SystemTimer;

global_asm!(include_str!("handler.S"));

pub unsafe fn set_vbar_el1_checked(vec_base_addr: u64) -> bool {
    if vec_base_addr.trailing_zeros() < 11 {
        false
    } else {
        asm!("msr VBAR_EL1, $0" :  : "r"(vec_base_addr) : : "volatile");

        // Force VBAR update to complete before next instruction.
        asm!("ISB SY" : : : "memory" : "volatile");

        true
    }
}

#[repr(C)]
pub struct GPR {
    x: [u64; 31],
}

#[repr(C)]
pub struct ExceptionContext {
    // General Purpose Registers
    gpr: GPR,
    spsr_el1: u64,
    elr_el1: u64,
    esr_el1: u64
}

#[no_mangle]
pub unsafe extern "C" fn default_exception_handler() {
    let esr : usize;
    let address: usize; 
    asm!("mrs $0, esr_el1" : "=r"(esr) : : : "volatile");
    asm!("mrs $0, elr_el1" : "=r"(address) : : : "volatile");
    crate::println!("Invalid exception ({}) at address: {} with esr: {}", stringify!(ident), address, esr);
    loop {
        asm!("wfe": : : : "volatile");
    }
}

// To implement an exception handler, overwrite it by defining the respective
// function below.
// Don't forget the #[no_mangle] attribute.
//
// unsafe extern "C" fn current_el0_synchronous(e: &mut ExceptionContext);
// unsafe extern "C" fn current_el0_irq(e: &mut ExceptionContext);
// unsafe extern "C" fn current_el0_serror(e: &mut ExceptionContext);

// unsafe extern "C" fn current_elx_synchronous(e: &mut ExceptionContext);
// unsafe extern "C" fn current_elx_irq(e: &mut ExceptionContext);
// unsafe extern "C" fn current_elx_serror(e: &mut ExceptionContext);

// unsafe extern "C" fn lower_aarch64_synchronous(e: &mut ExceptionContext);
// unsafe extern "C" fn lower_aarch64_irq(e: &mut ExceptionContext);
// unsafe extern "C" fn lower_aarch64_serror(e: &mut ExceptionContext);

// unsafe extern "C" fn lower_aarch32_synchronous(e: &mut ExceptionContext);
// unsafe extern "C" fn lower_aarch32_irq(e: &mut ExceptionContext);
// unsafe extern "C" fn lower_aarch32_serror(e: &mut ExceptionContext);

#[no_mangle]
unsafe extern "C" fn current_elx_synchronous(e: &mut ExceptionContext) {
    println!("[!] A synchronous exception happened.");
    e.elr_el1 += 4;
    println!("[!] Returning from exception...");
}
/////////////////////////
//         IRQ         //
///////////////////////// 
register_bitfields! {
    u32,
    IRQ_PENDING_1 [
        TIMER_LINE_0 OFFSET(0) NUMBITS(1) [Yes = 1, No = 0],
        TIMER_LINE_1 OFFSET(1) NUMBITS(1) [Yes = 1, No = 0],
        TIMER_LINE_2 OFFSET(2) NUMBITS(1) [Yes = 1, No = 0],
        TIMER_LINE_3 OFFSET(3) NUMBITS(1) [Yes = 1, No = 0],
        UNUSED OFFSET(4) NUMBITS(28)[]
    ],
    IRQ_ENABLE_1 [
        TIMER_LINE_0 OFFSET(0) NUMBITS(1) [Set = 1],
        TIMER_LINE_1 OFFSET(1) NUMBITS(1) [Set = 1],
        TIMER_LINE_2 OFFSET(2) NUMBITS(1) [Set = 1],
        TIMER_LINE_3 OFFSET(3) NUMBITS(1) [Set = 1],
        UNUSED OFFSET(4) NUMBITS(28)[]
    ],
    IRQ_DISABLE_1 [
        TIMER_LINE_0 OFFSET(0) NUMBITS(1) [Set = 1],
        TIMER_LINE_1 OFFSET(1) NUMBITS(1) [Set = 1],
        TIMER_LINE_2 OFFSET(2) NUMBITS(1) [Set = 1],
        TIMER_LINE_3 OFFSET(3) NUMBITS(1) [Set = 1],
        UNUSED OFFSET(4) NUMBITS(28)[]
    ]
}
struct Irq;

const IRQ_BASE : usize = MMIO_BASE + 0x0000B200;

struct RegisterBlock {
    IRQ_BASIC_PENDING : ReadOnly<u32>,                      //0x00
    IRQ_PENDING_1 : ReadOnly<u32, IRQ_PENDING_1::Register>, //0x04
    IRQ_PENDING_2 : ReadOnly<u32>,                          //0x08
    __unused_val : u32,                                     //0x0C
    IRQ_ENABLE_1 : WriteOnly<u32, IRQ_ENABLE_1::Register>,  //0x10
    IRQ_ENABLE_2 : WriteOnly<u32>,                          //0x14
    IRQ_ENABLE_BASIC : WriteOnly<u32>,                      //0x18
    IRQ_DISABLE_1 : WriteOnly<u32, IRQ_DISABLE_1::Register>,//0x1C
    IRQ_DISABLE_2 : WriteOnly<u32>,                         //0x20
    IRQ_DISABLE_BASIC : WriteOnly<u32>,                     //0x24
}
impl ops::Deref for Irq {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(IRQ_BASE as *const RegisterBlock)}
    }
}
impl Irq {
    #[inline]
    #[naked]
    pub fn enable() {
        unsafe { asm!("msr daifclr, #2" : : : : "volatile"); }
    }
    #[inline]
    #[naked]
    pub fn disable() {
        unsafe { asm!("msr daifset, #2" : : : : "volatile"); }
    }
}

const TIMER_INTERVAL : u32 = 20_000;

#[no_mangle]
unsafe extern "C" fn current_elx_irq(e: &mut ExceptionContext) {
    println!("[!] IRQ Interupt");

    if Irq.IRQ_PENDING_1.read(IRQ_PENDING_1::TIMER_LINE_1) != 0 {

        SystemTimer::reset_interupt_interval(TIMER_INTERVAL);
        let time = SystemTimer::get_time();
        println!("[{}] Timer generated an interupt",time);
    }
    unsafe {
        use timer::ArmQemuTimer;
        ArmQemuTimer::disable();
        ArmQemuTimer::interupt_after(ArmQemuTimer::get_frequency());
        ArmQemuTimer::enable();
        println!("Time since boot: {} seconds", ArmQemuTimer::get_time() / (ArmQemuTimer::get_frequency() + 1) as u64);
    }
}
impl SystemTimer {
    #[inline(never)]
    pub fn enable() {
        Irq::enable();
        println!("Value {:b}",IRQ_ENABLE_1::TIMER_LINE_1::Set.value);
        Irq.IRQ_ENABLE_1.write(IRQ_ENABLE_1::TIMER_LINE_1::Set);
        SystemTimer::set_interupt_interval(TIMER_INTERVAL);
    }
    #[inline(never)]
    pub fn disable() {
        Irq.IRQ_DISABLE_1.write(IRQ_DISABLE_1::TIMER_LINE_1::Set);
        Irq::disable();
    }
}
