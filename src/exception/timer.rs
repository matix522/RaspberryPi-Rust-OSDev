use core::ops;
use crate::memory::physical::MMIO_BASE;
use register::{mmio::*, register_bitfields};

pub struct SystemTimer;

register_bitfields! {
    u32,
    /// TIMER CONTROLE STATUS
    CTRL_STATUS [
        LINE_0 OFFSET(0) NUMBITS(1) [OK = 1],
        LINE_1 OFFSET(1) NUMBITS(1) [OK = 1],
        LINE_2 OFFSET(2) NUMBITS(1) [OK = 1],
        LINE_3 OFFSET(3) NUMBITS(1) [OK = 1]
    ]
}
pub struct RegisterStatus {
    control_status: WriteOnly<u32, CTRL_STATUS::Register>,  // 0x00
    counter_low: ReadOnly<u32>,                             // 0x04
    counter_high: ReadOnly<u32>,                            // 0x08
    compare_0: WriteOnly<u32>,                              // 0x0C - GPU RESEREVED
    compare_1: WriteOnly<u32>,                              // 0x10
    compare_2: WriteOnly<u32>,                              // 0x14 - GPU RESEREVED
    compare_3: WriteOnly<u32>,                              // 0x18
}
const TIMER_BASE : usize = MMIO_BASE + 0x00003000;
impl ops::Deref for SystemTimer {
    type Target = RegisterStatus;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(TIMER_BASE as *const RegisterStatus) }
    }
}
impl SystemTimer {
    #[inline(never)]
    pub fn get_time() -> u64 {
        let mut high = SystemTimer::get_time_high();
        let mut low = SystemTimer::get_time_low(); 
        //chrek for change in high
        if high != SystemTimer::get_time_high() {
            high = SystemTimer::get_time_high();
            low = SystemTimer::get_time_low();
        }

        u64::from(high) << 32 | u64::from(low)
    }
    #[inline(never)]
    fn get_time_low() -> u32 {
        SystemTimer.counter_low.get()
    }
    #[inline(never)]
    fn get_time_high() -> u32 {
        SystemTimer.counter_high.get()
    }
    #[inline(never)]
    pub fn set_interupt_interval(interval :u32) {
        SystemTimer.compare_1.set(SystemTimer::get_time_low() + interval);
    }
    #[inline(never)]
    pub fn reset_interupt_interval(interval :u32) {
        SystemTimer::set_interupt_interval(interval);
        SystemTimer.control_status.write(CTRL_STATUS::LINE_1::OK);
    }
}
//CNTCLKEN
//CNTVALUEB[63:0]
pub struct RegisterBlocArm {
    route_clock: WriteOnly<u32>,
}
const ARM_CLOCK_BASE : usize = 0x40000040; 
pub struct ArmQemuTimer;
impl ops::Deref for ArmQemuTimer {
    type Target = RegisterBlocArm;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(ARM_CLOCK_BASE as *const RegisterBlocArm) }
    }
}
impl ArmQemuTimer {
    pub fn enable(){
        ArmQemuTimer.route_clock.set(0x8);
        let val : u32 = 1;
        unsafe {
            asm!("msr cntv_ctl_el0, $0" : : "r"(val) : : "volatile");
        }
    }
    pub fn disable(){
        ArmQemuTimer.route_clock.set(0x0);
        let val : u32 = 0;
        unsafe {
            asm!("msr cntv_ctl_el0, $0" : : "r"(val) : : "volatile");
        }
    }
    pub fn get_frequency() -> u32 {
        let frequency;
        unsafe {
            asm!("mrs $0, cntfrq_el0" : "=r"(frequency) : : : "volatile");
        }
        frequency
    }
    pub fn interupt_after(ticks : u32) {
        unsafe {
        	asm!("msr cntv_tval_el0, $0" : : "r"(ticks) : : "volatile");
        }
    }
    pub fn ticks_to_interupt() -> u32 {
        let ticks;
        unsafe {
            asm!("mrs $0, cntfrq_el0" : "=r"(ticks) : : : "volatile");
        }
        ticks
    }
    pub fn get_time() -> u64 {
        let ticks;
        unsafe {
	        asm!("mrs $0, cntvct_el0" : "=r" (ticks));
        }
        ticks
    }
}