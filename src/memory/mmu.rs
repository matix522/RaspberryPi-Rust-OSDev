use super::descriptors::{PageDescriptor,Level2BlockDescriptor, TableDescriptor};
use super::descriptors;
use super::BaseAddr;

const SIZE_FOR_4KIB: usize = 512;

#[repr(C)]
#[repr(align(4096))]
struct PageTable {
    entries: [u64; SIZE_FOR_4KIB],
}
impl super::BaseAddr for PageTable {
    fn base_addr_u64(&self) -> u64 {
        &self.entries as *const u64 as u64
    }

    fn base_addr_usize(&self) -> usize {
        &self.entries as *const u64 as usize
    }
}

/// The LEVEL2 page table containng the 2 MiB entries.
static mut LEVEL2_TABLE: PageTable = PageTable {
    entries: [0; SIZE_FOR_4KIB],
};

/// The LEVEL3 page table containing the 4 KiB entries.
///
/// The first entry of the LEVEL2_TABLE will forward to this table.
static mut LEVEL3_TABLE: PageTable = PageTable {
    entries: [0; SIZE_FOR_4KIB],
};

pub unsafe fn init() -> Result<(), &'static str> {
    set_mair_registry();
    LEVEL2_TABLE.entries[0] = match TableDescriptor::new(LEVEL3_TABLE.base_addr_usize()) {
        Err(s) => return Err(s),
        Ok(d) => d.value(),
    };

    for (block_descriptor_nr, entry) in LEVEL2_TABLE.entries.iter_mut().enumerate().skip(1) {
        let virt_addr = block_descriptor_nr << descriptors::TWO_MIB_SHIFT;

        let (output_addr, attribute_fields) = match super::get_virt_addr_properties(virt_addr) {
            Err(s) => return Err(s),
            Ok((a, b)) => (a, b),
        };

        let block_desc = match Level2BlockDescriptor::new(output_addr, attribute_fields) {
            Err(s) => return Err(s),
            Ok(desc) => desc,
        };

        *entry = block_desc.value();
    }



    
    Ok(())
}
/// Constants for indexing the MAIR_EL1.
//#[allow(dead_code)]
pub(super) mod mair {
    pub const DEVICE: u64 = 0;
    pub const NORMAL: u64 = 1;
}

/// Setup function for the MAIR_EL1 register.
unsafe fn set_mair_registry() {
    const MAIR_HIGH_DEVICE: u64 = 0b00000000;
    const MAIR_HIGH_OUTER_NON_CACHEABLE: u64 = 0b01000000;
    const MAIR_HIGH_OUTER_WRITE_BACK_NON_TRANSIENT_READ_ALLOC_WRITE_ALLOC: u64 = 0b11110000;

    const MAIR_LOW_DEVICE_nGnRE: u64 = 0b00000100;

    const MAIR_LOW_INNER_NON_CACHEABLE: u64 = 0b00000100;
    const MAIR_LOW_INNER_WRITE_BACK_NON_TRANSIENT_READ_ALLOC_WRITE_ALLOC: u64 = 0b00001111;

    let mut attributes: [u64; 8] = [0; 8];

    attributes[0] = MAIR_HIGH_DEVICE + MAIR_LOW_DEVICE_nGnRE;
    attributes[1] = MAIR_HIGH_OUTER_WRITE_BACK_NON_TRANSIENT_READ_ALLOC_WRITE_ALLOC
        + MAIR_LOW_INNER_WRITE_BACK_NON_TRANSIENT_READ_ALLOC_WRITE_ALLOC;
    let mut mair_el1 = 0;
    for i in 0..8 {
        mair_el1 = mair_el1 * 0x100 + attributes[7 - i];
    }
    asm!("msr mair_el1 ,$0" : : "r"(mair_el1): : "volatile");
}
