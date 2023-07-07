#![cfg(test)]

use crate::mmu::PageMapError;
use crate::mmu::PageSize;
use crate::mmu::PageTableSpace;
use crate::mmu::VirtualAddress;
use crate::regs::MemoryAttributeEl1;
use crate::regs::MemoryAttributeIndirectionEl1;

#[test]
fn test_mmu_small_pages() {
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

#[test]
fn test_mmu_large_pages() {
    let mut space = vec![0xaa; 0x100000];
    let mut page_tables =
        PageTableSpace::new(0x00000040248000, &mut space).expect("Can initialize page tables");

    let mair_el1 = MemoryAttributeIndirectionEl1::default();
    let wb_index = mair_el1
        .get_index(MemoryAttributeEl1::Normal_WriteBack)
        .expect("must be some WB memory available");

    let res = page_tables.map_pages(
        0,
        VirtualAddress::from(0),
        0x2000,
        PageSize::Large,
        wb_index,
    );
    assert_eq!(res, Ok(()));

    let res = page_tables.map_pages(
        0x4000,
        VirtualAddress::from(0x4000),
        4,
        PageSize::Small,
        wb_index,
    );
    assert_eq!(res, Err(PageMapError::AlreadyMapped));

    std::fs::write("page_tables_large.bin", space).expect("can dump the page tables");
}

#[test]
fn test_mmu_huge_pages() {
    let mut space = vec![0xaa; 0x100000];
    let mut page_tables =
        PageTableSpace::new(0x00000040248000, &mut space).expect("Can initialize page tables");

    let mair_el1 = MemoryAttributeIndirectionEl1::default();
    let wb_index = mair_el1
        .get_index(MemoryAttributeEl1::Normal_WriteBack)
        .expect("must be some WB memory available");

    let res = page_tables.map_pages(0, VirtualAddress::from(0), 4, PageSize::Huge, wb_index);
    assert_eq!(res, Ok(()));

    let res = page_tables.map_pages(
        1 << 30,
        VirtualAddress::from(0x4000_0000),
        4,
        PageSize::Small,
        wb_index,
    );
    assert_eq!(res, Err(PageMapError::AlreadyMapped));

    std::fs::write("page_tables_huge.bin", space).expect("can dump the page tables");
}
