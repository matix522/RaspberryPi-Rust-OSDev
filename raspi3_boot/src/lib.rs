#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![feature(asm)]
//! Low-level boot of the Raspberry's processor

extern crate panic_abort;
pub mod asm;

/// Type check the user-supplied entry function.
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    };
}

/// Reset function.
///
/// Initializes the bss section before calling into the user's `main()`.
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        // Boundaries of the .bss section, provided by the linker script
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    // Zeroes the .bss section
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    extern "Rust" {
        fn main() -> !;
    }

    main();
}
mod exception_level {

    use super::asm;

    const SCTLR_RESERVED: u64 = (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11);
    const SCTLR_EE_LITTLE_ENDIAN: u64 = (0 << 25);
    //const  SCTLR_EOE_LITTLE_ENDIAN  :u64 = (0 << 24);
    const SCTLR_I_CACHE_DISABLED: u64 = (0 << 12);
    const SCTLR_D_CACHE_DISABLED: u64 = (0 << 2);
    const SCTLR_MMU_DISABLED: u64 = (0 << 0);
    //const  SCTLR_MMU_ENABLED :u64 =   (1 << 0);

    const SCTLR_VALUE_MMU_DISABLED: u64 = (SCTLR_RESERVED
        | SCTLR_EE_LITTLE_ENDIAN
        | SCTLR_I_CACHE_DISABLED
        | SCTLR_D_CACHE_DISABLED
        | SCTLR_MMU_DISABLED);

    const HCR_RW: u64 = (1 << 31);
    const HCR_VALUE: u64 = HCR_RW;

    const SCR_RESERVED: u64 = (3 << 4);
    const SCR_RW: u64 = (1 << 10);
    const SCR_NS: u64 = (1 << 0);
    const SCR_VALUE: u64 = (SCR_RESERVED | SCR_RW | SCR_NS);

    const SPSR_MASK_ALL: u64 = (7 << 6);
    const SPSR_EL1H: u64 = (5 << 0);
    const SPSR_VALUE: u64 = (SPSR_MASK_ALL | SPSR_EL1H);

    #[inline]
    /// Switch Exception Level from 2 or 3 to 1
    /// For any other value enter infinite loop
    pub unsafe fn to1() -> ! {
        const STACK_START: u64 = 0x80_000;

        let current_level: u64 = asm::get_exception_level();

        match current_level >> 2 {
            2 => {
                asm::set_stack_pointer_kernel(STACK_START);
                asm::set_system_control_register_kernel(SCTLR_VALUE_MMU_DISABLED);
                asm::set_hypervisor_configuration_register(HCR_VALUE);
                asm::set_hypervisor_saved_program_status_register(SPSR_VALUE);
                asm::set_hypervisor_exception_return_adrress(super::reset as *const () as u64);
            }
            3 => {
                asm::set_stack_pointer_kernel(STACK_START);
                asm::set_system_control_register_kernel(SCTLR_VALUE_MMU_DISABLED);
                asm::set_hypervisor_configuration_register(HCR_VALUE);
                asm::set_secure_configuration_register_safe(SCR_VALUE);
                asm::set_secure_saved_program_status_register(SPSR_VALUE);
                asm::set_secure_exception_return_adrress(super::reset as *const () as u64);
            }
            _ => loop {
                asm::wfe();
            },
        }

        asm::eret();
        loop {
            asm::wfe();
        }
    }
}
#[link_section = ".text.boot"]
#[no_mangle]
/// Entry Point for all cores after GPU transferes
pub unsafe extern "C" fn _boot_cores() -> ! {
    const CORE_0: u64 = 0;
    const CORE_MASK: u64 = 0x3;

    let core_number = asm::get_core_id();

    if CORE_0 == (core_number & CORE_MASK) {
        //asm!( "mov sp, $0" : : "r"(STACK_START) : : "volatile");
        exception_level::to1();
    }
    // if not core0, infinitely wait for events
    loop {
        asm::wfe();
    }
}
