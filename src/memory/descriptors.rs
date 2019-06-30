use register::{mmio::*, register_bitfields};
use super::kernel_mem_range::*;

/// A descriptor pointing to the next page table.
pub(super) struct TableDescriptor(register::FieldValue<u64, DESCRIPTOR::Register>);

/// A Level2 block descriptor with 2 MiB aperture.
///
/// The output points to physical memory.
pub(super) struct Level2BlockDescriptor(register::FieldValue<u64, DESCRIPTOR::Register>);

/// A page descriptor with 4 KiB aperture.
///
/// The output points to physical memory.
pub(super) struct PageDescriptor(register::FieldValue<u64, DESCRIPTOR::Register>);

#[rustfmt::skip]
register_bitfields!{
    u64,
    DESCRIPTOR [
        IS_VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True  = 1
        ],
        TYPE OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],
        MEMORY_ATTRIBUTE_INDEX OFFSET(2) NUMBITS(3) [
            
        ],

        ACCESS_PERMISSION OFFSET(6) NUMBITS(2) [
            ReadWrite_EL1      = 0b00,
            ReadWrite_EL1_EL0  = 0b01,
            ReadOnly_EL1       = 0b10,
            ReadOnly_EL1_EL0   = 0b11
        ],
        SHAREABLE OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],
        ACCESSED OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ], 
        TABLE_ADDR_4KiB OFFSET(12) NUMBITS(36) [], // [47:12]

        LEVEL2_OUTPUT_ADDR_4KiB OFFSET(21) NUMBITS(27) [], // [47:21]
        
        PRIVILEGED_EXECUTE_NEVER OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}
pub const FOUR_KIB : usize = 4 * 1024;
pub const FOUR_KIB_SHIFT: usize = 12; // log2(FOUR_KIB)

pub const TWO_MIB : usize = 2 * 1024 * 1024;
pub const TWO_MIB_SHIFT: usize = 21; // log2(FOUR_KIB)


impl TableDescriptor {
     pub(super) fn new(next_lvl_table_addr: usize) -> Result<TableDescriptor, &'static str> {
        if next_lvl_table_addr % FOUR_KIB != 0 {
            return Err("TableDescriptor: Address is not 4 KiB aligned.");
        }

        let shifted = next_lvl_table_addr >> FOUR_KIB_SHIFT;

        Ok(TableDescriptor(
            DESCRIPTOR::IS_VALID::True
                + DESCRIPTOR::TYPE::Table
                + DESCRIPTOR::TABLE_ADDR_4KiB.val(shifted as u64),
        ))
    }

     pub(super) fn value(&self) -> u64 {
        self.0.value
    }
}

impl Level2BlockDescriptor {
     pub(super) fn new(
        output_addr: usize,
        attribute_fields: AttributeFields,
    ) -> Result<Level2BlockDescriptor, &'static str> {
        if output_addr % TWO_MIB != 0 {
            return Err("BlockDescriptor: Address is not 2 MiB aligned.");
        }

        let shifted = output_addr >> TWO_MIB_SHIFT;

        Ok(Level2BlockDescriptor(
            DESCRIPTOR::IS_VALID::True
                + DESCRIPTOR::ACCESSED::True
                + into_mmu_attributes(attribute_fields)
                + DESCRIPTOR::TYPE::Block
                + DESCRIPTOR::LEVEL2_OUTPUT_ADDR_4KiB.val(shifted as u64),
        ))
    }

     pub(super) fn value(&self) -> u64 {
        self.0.value
    }
}


impl PageDescriptor {
    pub(super) fn new(output_addr: usize,attribute_fields: AttributeFields) 
        -> Result<PageDescriptor, &'static str> {
        if output_addr % FOUR_KIB != 0 {
            return Err("PageDescriptor: Address is not 4 KiB aligned.");
        }

        let shifted = output_addr >> FOUR_KIB_SHIFT;

        Ok(PageDescriptor(
            DESCRIPTOR::IS_VALID::True
                + DESCRIPTOR::ACCESSED::True
                + into_mmu_attributes(attribute_fields)
                + DESCRIPTOR::TYPE::Table
                + DESCRIPTOR::TABLE_ADDR_4KiB.val(shifted as u64),
        ))
    }

     pub(super) fn value(&self) -> u64 {
        self.0.value
    }
}

fn into_mmu_attributes(
    attribute_fields: AttributeFields,
) -> register::FieldValue<u64, DESCRIPTOR::Register> {
    //use crate::memory::{AccessPermissions, MemAttributes};
    use super::mmu::mair;
    // Memory attributes
    let mut desc = match attribute_fields.mem_attributes {
        MemAttributes::CacheableDRAM => {
            DESCRIPTOR::SHAREABLE::InnerShareable + 
            DESCRIPTOR::MEMORY_ATTRIBUTE_INDEX.val(mair::NORMAL)
        }
        MemAttributes::Device => {
            DESCRIPTOR::SHAREABLE::OuterShareable + 
            DESCRIPTOR::MEMORY_ATTRIBUTE_INDEX.val(mair::DEVICE)
        }
    };

    // Access Permissions
    desc += match attribute_fields.acc_perms {
        AccessPermissions::ReadOnly => DESCRIPTOR::ACCESS_PERMISSION::ReadOnly_EL1,
        AccessPermissions::ReadWrite => DESCRIPTOR::ACCESS_PERMISSION::ReadWrite_EL1,
    };

    // Execute Never
    desc += if attribute_fields.execute_never {
        DESCRIPTOR::PRIVILEGED_EXECUTE_NEVER::True
    } else {
        DESCRIPTOR::PRIVILEGED_EXECUTE_NEVER::False
    };

    desc
}