pub mod asm {
    #[inline]
    pub fn nop(){
        unsafe { asm!("nop" : : : : "volatile");}
    }
    #[inline]
    pub fn wfe(){
        unsafe { asm!("wfe" : : : : "volatile");}
    }
    #[inline]
    pub fn eret(){
        unsafe { asm!("eret" : : : : "volatile");}
    }
}
/// Function waits at least ticks instructions
/// # Arguments
/// ticks : u32 - Number of ticks to wait
pub fn delay(ticks : u32) {
    for _ in 0..ticks {
        asm::nop();
    }
}
#[derive(Debug)]
/// Enum representing value of Exception Level of ARM proccessor.
pub enum ExceptionLevel {
    ///Exception Level 0
    User,
    ///Exception Level 1
    Kernel,
    ///Exception Level 2
    Hypervisor,
    ///Exception Level 3
    Safe,
    ///Not correct value
    Error
}
/// Function returns enum ExceptionLevel that is used for execution of this function
pub fn get_excception_level() -> ExceptionLevel {
    use ExceptionLevel::*;
    let mut level : u64;
    unsafe { 
        asm!("mrs $0, CurrentEL" : "=r"(level) : : : "volatile"); 
    }
    match level >> 2 {
        0 => User,
        1 => Kernel,
        2 => Hypervisor,
        3 => Safe,
        _ => Error
    }
}