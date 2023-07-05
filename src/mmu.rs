extern "C" {
    fn _page_tables_start();
    fn _page_tables_end();
}

pub fn page_tables_area_start_addr() -> usize {
    _page_tables_start as usize
}

pub fn page_tables_area_end_addr() -> usize {
    _page_tables_end as usize
}

#[allow(dead_code)]
pub fn reset_tables_area() -> &'static mut [u8] {
    let s = page_tables_area_start_addr();
    let e = page_tables_area_end_addr();
    let b = unsafe { core::slice::from_raw_parts_mut(s as *mut u8, e - s) };
    b.fill(0xfe);

    b
}
