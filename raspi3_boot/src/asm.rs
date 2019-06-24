#![deny(missing_docs)]
#![deny(warnings)]

//! Asm utiils used in raspi3_boot.
//! Module containing definitions of inline functions for
//! abstarction of assembly instructuions used in kernel.
#[inline]
///Assembly nop (No operation) instruction
pub fn nop() {
    unsafe {
        asm!("nop" : : : : "volatile");
    }
}
#[inline]
///Assembly wfe (Wait for event) instruction
pub fn wfe() {
    unsafe {
        asm!("wfe" : : : : "volatile");
    }
}
#[inline]
///Assembly eret (Exception return) instruction
pub fn eret() {
    unsafe {
        asm!("eret" : : : : "volatile");
    }
}
#[inline]
///Set Stack Pointer of Kernel Mode
pub fn set_stack_pointer_kernel(sp: u64) {
    unsafe {
        asm!("msr sp_el1, $0" :  : "r"(sp) : : "volatile");
    }
}
#[inline]
///Set System Control Register for Kernel Mode
pub fn set_system_control_register_kernel(sctrl: u64) {
    unsafe {
        asm!("msr sctlr_el1, $0" :  :  "r"(sctrl) : : "volatile");
    }
}
#[inline]
///Set Hypervisor Configuration Register
pub fn set_hypervisor_configuration_register(hcr: u64) {
    unsafe {
        asm!("msr hcr_el2, $0" :  : "r"(hcr) : : "volatile");
    }
}

#[inline]
///Set Saved Program Status Register for Hypervisor
pub fn set_hypervisor_saved_program_status_register(spsr: u64) {
    unsafe {
        asm!("msr spsr_el2, $0" :  : "r"(spsr) : : "volatile");
    }
}
#[inline]
///Set Saved Program Status Register for Hypervisor
pub fn set_hypervisor_exception_return_adrress(spsr: u64) {
    unsafe {
        asm!("msr elr_el2, $0" :  : "r"(spsr) : : "volatile");
    }
}

#[inline]
///Set Secure Configuration Register
pub fn set_secure_configuration_register_safe(scr: u64) {
    unsafe {
        asm!("msr scr_el3, $0" :  : "r"(scr) : : "volatile");
    }
}
#[inline]
///Set Saved Program Status Register for Secure Mode
pub fn set_secure_saved_program_status_register(spsr: u64) {
    unsafe {
        asm!("msr spsr_el3, $0" :  : "r"(spsr) : : "volatile");
    }
}
#[inline]
///Set Saved Program Status Register for Secure Mode
pub fn set_secure_exception_return_adrress(spsr: u64) {
    unsafe {
        asm!("msr elr_el3, $0" :  : "r"(spsr) : : "volatile");
    }
}
#[inline]
///Returns id of current CPU ID
pub fn get_core_id() -> u64 {
    let id;
    unsafe {
        asm!("mrs $0, mpidr_el1" : "=r"(id) : : : "volatile");
    }
    id
}
#[inline]
///Returns exception level of this path of execution
pub fn get_exception_level() -> u64 {
    let el;
    unsafe {
        asm!("mrs $0, CurrentEL" : "=r"(el) : : : "volatile");
    }
    el
}
