use bitfield_struct::bitfield;

extern "C" {
    fn _page_tables_start();
    fn _page_tables_end();
}

pub fn page_tables_phys_start() -> usize {
    _page_tables_start as usize
}

pub fn page_tables_phys_end() -> usize {
    _page_tables_end as usize
}

#[allow(dead_code)]
pub fn page_tables_area() -> &'static mut [u8] {
    let s = page_tables_phys_start();
    let e = page_tables_phys_end();
    unsafe { core::slice::from_raw_parts_mut(s as *mut u8, e - s) }
}

#[bitfield(u64)]
pub struct VirtualAddress {
    #[bits(12)]
    pub offset: u64,
    #[bits(9)]
    pub lvl0: u64,
    #[bits(9)]
    pub lvl1: u64,
    #[bits(9)]
    pub lvl2: u64,
    #[bits(9)]
    pub lvl3: u64,
    #[bits(16)]
    pub asid: u64,
}

const PAGE_SHIFT: u64 = 12;
const LARGE_PAGE_SHIFT: u64 = 21;
const HUGE_PAGE_SHIFT: u64 = 30;

const PAGE_SIZE: u64 = 1 << PAGE_SHIFT;
const LARGE_PAGE_SIZE: u64 = 1 << LARGE_PAGE_SHIFT;
const HUGE_PAGE_SIZE: u64 = 1 << HUGE_PAGE_SHIFT;

#[derive(Debug, Clone, Copy)]
pub enum PageMapError {
    OutOfMemory,
    NonCanonicalVirtAddress,
    MisalignedVirtAddress,
    MisalignedPhysAddress,
    InvalidMappingSize,
    EmptyMapping,
}

#[derive(Debug)]
pub struct PageTableSpace<'a> {
    phys_start: usize,
    phys_end: usize,
    space: &'a mut [u8],
    brk: usize,
}

impl<'a> PageTableSpace<'a> {
    pub fn new(phys_start: usize, phys_end: usize, space: &'a mut [u8]) -> Self {
        debug_assert!(phys_end > phys_start);
        debug_assert!(phys_start & (PAGE_SIZE as usize - 1) == 0);
        debug_assert!(phys_end & (PAGE_SIZE as usize - 1) == 0);

        // Situate the root table at the beginning,
        // and initialize it with a value that makes pages appear as
        // non-present (at least on x64 and aarch64).
        space[..PAGE_SIZE as usize].fill(0xfe);

        Self {
            phys_start,
            phys_end,
            space,
            brk: phys_start - PAGE_SIZE as usize,
        }
    }

    pub fn start(&self) -> usize {
        self.phys_start
    }

    pub fn end(&self) -> usize {
        self.phys_end
    }

    pub fn brk(&self) -> usize {
        self.brk
    }

    pub fn available(&self) -> usize {
        if self.phys_end > self.brk {
            self.phys_end - self.brk
        } else {
            0
        }
    }

    fn map_huge(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        size: u64,
    ) -> Result<(), PageMapError> {
        debug_assert!(phys_addr & (HUGE_PAGE_SIZE - 1) == 0);
        debug_assert!(virt_addr.offset() == 0);
        debug_assert!(virt_addr.lvl0() == 0);
        debug_assert!(virt_addr.lvl1() == 0);
        debug_assert!(size & (HUGE_PAGE_SIZE - 1) == 0);
        debug_assert!(size != 0);

        Ok(())
    }

    fn map_large(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        map_size: u64,
    ) -> Result<(), PageMapError> {
        debug_assert!(phys_addr & (LARGE_PAGE_SIZE - 1) == 0);
        debug_assert!(virt_addr.offset() == 0);
        debug_assert!(virt_addr.lvl0() == 0);
        debug_assert!(map_size & (LARGE_PAGE_SIZE - 1) == 0);
        debug_assert!(map_size != 0);

        Ok(())
    }

    fn map(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        map_size: u64,
    ) -> Result<(), PageMapError> {
        debug_assert!(phys_addr & (PAGE_SIZE - 1) == 0);
        debug_assert!(virt_addr.offset() == 0);
        debug_assert!(map_size & (PAGE_SIZE - 1) == 0);
        debug_assert!(map_size != 0);

        Ok(())
    }

    pub fn map_range(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        map_size: u64,
    ) -> Result<(), PageMapError> {
        if phys_addr & (PAGE_SIZE - 1) != 0 {
            return Err(PageMapError::MisalignedPhysAddress);
        }
        if map_size & (PAGE_SIZE - 1) != 0 {
            return Err(PageMapError::InvalidMappingSize);
        }
        if map_size == 0 {
            return Err(PageMapError::EmptyMapping);
        }
        if virt_addr.offset() != 0 {
            return Err(PageMapError::MisalignedVirtAddress);
        }
        if virt_addr.asid() != 0 && virt_addr.asid() != 0xffff {
            return Err(PageMapError::NonCanonicalVirtAddress);
        }

        let mut mapped = 0;
        let mut non_mapped = map_size;
        let mut phys_addr = phys_addr;
        let mut virt_addr = virt_addr.into();

        while mapped < map_size {
            // Try larger pages first, then try to map up to the next large page.
            let map_size = if phys_addr & (HUGE_PAGE_SIZE - 1) == 0
                && virt_addr & (HUGE_PAGE_SIZE - 1) == 0
                && non_mapped >= HUGE_PAGE_SIZE
            {
                let map_size = non_mapped & !(HUGE_PAGE_SIZE - 1);
                self.map_huge(phys_addr, VirtualAddress(virt_addr), map_size)?;

                map_size
            } else if phys_addr & (LARGE_PAGE_SIZE - 1) == 0
                && virt_addr & (LARGE_PAGE_SIZE - 1) == 0
                && non_mapped >= LARGE_PAGE_SIZE
            {
                let map_size = non_mapped & (HUGE_PAGE_SIZE - 1) & !(LARGE_PAGE_SIZE - 1);
                self.map_large(phys_addr, VirtualAddress(virt_addr), map_size)?;

                map_size
            } else {
                let map_size = core::cmp::min(
                    non_mapped & (HUGE_PAGE_SIZE - 1),
                    non_mapped & (LARGE_PAGE_SIZE - 1),
                );
                self.map(phys_addr, VirtualAddress(virt_addr), map_size)?;

                map_size
            };

            mapped += map_size;
            non_mapped -= map_size;
            phys_addr += map_size;
            virt_addr += map_size;
        }

        debug_assert!(mapped == map_size);
        debug_assert!(non_mapped == 0);
        Ok(())
    }
}
