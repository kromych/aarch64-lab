#![cfg(test)]

use crate::mmu::PageSize;
use crate::mmu::PageTableSpace;
use crate::mmu::VirtualAddress;
use crate::regs::MemoryAttributeEl1;
use crate::regs::MemoryAttributeIndirectionEl1;

#[test]
fn test_mmu() {
    let mut space = vec![0xaa; 0x100000];
    let mut page_tables =
        PageTableSpace::new(0x00000040248000, &mut space).expect("Can initialize page tables");

    let mair_el1 = MemoryAttributeIndirectionEl1::default();
    let wb_index = mair_el1
        .get_index(MemoryAttributeEl1::Normal_WriteBack)
        .expect("must be some WB memory available");

    let res = page_tables.map_pages(
        0x4000,
        VirtualAddress::from(0x4000),
        1,
        PageSize::Small,
        wb_index,
    );
    assert_eq!(res, Ok(()));

    let res = page_tables.map_pages(
        0x5000,
        VirtualAddress::from(0x5000),
        1,
        PageSize::Small,
        wb_index,
    );
    assert_eq!(res, Ok(()));

    let res = page_tables.map_pages(
        0x4000,
        VirtualAddress::from(0xffff_8000_0000_4000),
        1,
        PageSize::Small,
        wb_index,
    );
    assert_eq!(res, Ok(()));

    let res = page_tables.map_pages(
        0x5000,
        VirtualAddress::from(0xffff_8000_0000_5000),
        1,
        PageSize::Small,
        wb_index,
    );
    assert_eq!(res, Ok(()));

    let res = page_tables.map_pages(
        0x4000_0000,
        VirtualAddress::from(0x4000_0000),
        0x200,
        PageSize::Small,
        wb_index,
    );
    assert_eq!(res, Ok(()));

    std::fs::write("page_tables.bin", space).expect("can dump the page tables");
}
