#![cfg(test)]

use crate::mmu::PageSize;
use crate::mmu::PageTableSpace;
use crate::mmu::VirtualAddress;

#[test]
fn test_mmu() {
    let mut space = vec![0xaa; 0x100000];
    let mut page_tables =
        PageTableSpace::new(0x00000040248000, &mut space).expect("Can initialize page tables");

    let res = page_tables.map_small_page(0x4000, VirtualAddress::from(0x4000));
    assert_eq!(res, Ok(()));

    let res = page_tables.map_small_page(0x5000, VirtualAddress::from(0x5000));
    assert_eq!(res, Ok(()));

    let res = page_tables.map_small_page(0x4000, VirtualAddress::from(0xffff_8000_0000_4000));
    assert_eq!(res, Ok(()));

    let res = page_tables.map_small_page(0x5000, VirtualAddress::from(0xffff_8000_0000_5000));
    assert_eq!(res, Ok(()));

    let res = page_tables.map_pages(
        0x4000_0000,
        VirtualAddress::from(0x4000_0000),
        0x200_0000,
        PageSize::Small,
    );
    assert_eq!(res, Ok(()));

    std::fs::write("page_tables.bin", space).expect("can dump the page tables");
}
