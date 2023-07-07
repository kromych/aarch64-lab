use bitfield_struct::bitfield;

#[bitfield(u64)]
pub struct PageTableEntry {
    pub valid: bool,
    pub table: bool, // Use PageBlockEntry if `false`
    #[bits(8)]
    _mbz0: u64,
    pub accessed: bool,
    pub not_global: bool,
    #[bits(35)]
    pub next_table_pfn: u64,
    #[bits(12)]
    _mbz1: u64,
    pub priv_x_never: bool,
    pub user_x_never: bool,
    // NoEffect = 0b00,
    // PrivOnly = 0b01,
    // ReadOnly = 0b10,
    // PrivReadOnly = 0b11
    #[bits(2)]
    pub access_perm: u64,
    pub non_secure: bool,
}

#[bitfield(u64)]
pub struct PageBlockEntry {
    pub valid: bool,
    pub page: bool,
    #[bits(3)]
    pub mair_idx: usize,
    #[bits(1)]
    _mbz0: u64,
    // PrivOnly = 0b00,
    // ReadWrite = 0b01,
    // PrivReadOnly = 0b10,
    // ReadOnly = 0b11
    #[bits(2)]
    pub access_perm: u64,
    // NonShareable = 0b00,
    // OuterShareable = 0b10,
    // InnerShareable = 0b11
    #[bits(2)]
    pub share_perm: u64,
    pub accessed: bool,
    pub not_global: bool,
    #[bits(35)]
    pub address_pfn: u64,
    #[bits(4)]
    _mbz1: u64,
    pub dirty: bool,
    pub contig: bool,
    pub priv_x_never: bool,
    pub user_x_never: bool,
    #[bits(9)]
    _mbz2: u64,
}

#[bitfield(u64)]
pub struct VirtualAddress {
    #[bits(12)]
    pub offset: u64,
    #[bits(9)]
    pub lvl3: usize,
    #[bits(9)]
    pub lvl2: usize,
    #[bits(9)]
    pub lvl1: usize,
    #[bits(9)]
    pub lvl0: usize,
    #[bits(16)]
    pub asid: usize,
}

impl VirtualAddress {
    pub fn is_canonical(&self) -> bool {
        // The 16 most significant bits must be eqial to the 47th one.
        ((self.0 as i64) << 16 >> 16) == self.0 as i64
    }

    pub fn lvl_index(&self, index: usize) -> usize {
        match index {
            3 => self.lvl3(),
            2 => self.lvl2(),
            1 => self.lvl1(),
            0 => self.lvl0(),
            _ => panic!("invalid VA level index"),
        }
    }
}

const PAGE_SHIFT_4K: u64 = 12;
const PAGE_SHIFT_2M: u64 = 21;
const PAGE_SHIFT_1G: u64 = 30;

const PAGE_SIZE_4K: u64 = 1 << PAGE_SHIFT_4K;
const PAGE_SIZE_2M: u64 = 1 << PAGE_SHIFT_2M;
const PAGE_SIZE_1G: u64 = 1 << PAGE_SHIFT_1G;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageMapError {
    OutOfMemory,
    NonCanonicalVirtAddress,
    MisalignedVirtAddress,
    MisalignedPhysAddress,
    InvalidMappingSize,
    EmptyMapping,
    AlreadyMapped,
}

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum PageSize {
    Small = PAGE_SIZE_4K,
    Large = PAGE_SIZE_2M,
    Huge = PAGE_SIZE_1G,
}

#[derive(Debug)]
pub struct PageTableSpace<'a> {
    /// Physical address at which the page table area starts.
    /// The root page tables will be placed at this address.
    phys_page_table_root: usize,
    /// The memory occupied by page tables.
    space: &'a mut [u8],
    /// Physical address of the next free 4KiB-aligned block in the
    /// `space`. This is essentially a bump allocator for the memory
    /// used by the page tables.
    brk: usize,
}

impl<'a> PageTableSpace<'a> {
    pub fn new(phys_start: usize, space: &'a mut [u8]) -> Result<Self, PageMapError> {
        if phys_start & (PAGE_SIZE_4K as usize - 1) != 0 {
            return Err(PageMapError::MisalignedPhysAddress);
        }
        if space.len() & (PAGE_SIZE_4K as usize - 1) != 0 {
            return Err(PageMapError::InvalidMappingSize);
        }
        if space.is_empty() {
            return Err(PageMapError::EmptyMapping);
        }

        // Situate the root table at the beginning,
        // and initialize it with a value that makes pages appear as
        // non-present (at least on x64 and aarch64).
        space[..PAGE_SIZE_4K as usize].fill(0xfe);

        Ok(Self {
            phys_page_table_root: phys_start,
            space,
            brk: phys_start + PAGE_SIZE_4K as usize,
        })
    }

    fn allocate_page_table(&mut self) -> Result<u64, PageMapError> {
        if self.brk >= self.phys_page_table_root + self.space.len() {
            return Err(PageMapError::OutOfMemory);
        }
        let page_table_phys_addr = self.brk;
        self.brk += PAGE_SIZE_4K as usize;

        Ok(page_table_phys_addr as u64)
    }

    fn read_entry(&self, phys_table_start: u64, index: usize) -> u64 {
        debug_assert!(
            (phys_table_start as usize) < self.phys_page_table_root + self.space.len()
                && (phys_table_start as usize) >= self.phys_page_table_root
        );
        debug_assert!(phys_table_start & (PAGE_SIZE_4K - 1) == 0);
        debug_assert!(index < PAGE_SIZE_4K as usize / core::mem::size_of::<PageTableEntry>());

        let pos = phys_table_start as usize - self.phys_page_table_root
            + index * core::mem::size_of::<PageTableEntry>();
        u64::from_le_bytes([
            self.space[pos],
            self.space[pos + 1],
            self.space[pos + 2],
            self.space[pos + 3],
            self.space[pos + 4],
            self.space[pos + 5],
            self.space[pos + 6],
            self.space[pos + 7],
        ])
    }

    fn write_entry(&mut self, phys_table_start: u64, index: usize, entry: u64) {
        debug_assert!(
            (phys_table_start as usize) < self.phys_page_table_root + self.space.len()
                && (phys_table_start as usize) >= self.phys_page_table_root
        );
        debug_assert!(phys_table_start & (PAGE_SIZE_4K - 1) == 0);
        debug_assert!(index < PAGE_SIZE_4K as usize / core::mem::size_of::<PageTableEntry>());

        let pos = phys_table_start as usize - self.phys_page_table_root
            + index * core::mem::size_of::<PageTableEntry>();
        self.space[pos..pos + 8].copy_from_slice(&entry.to_le_bytes());
    }

    pub fn map_huge_page(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        _memory_attribute_index: usize,
    ) -> Result<(), PageMapError> {
        if phys_addr & (PAGE_SIZE_1G - 1) != 0 {
            return Err(PageMapError::MisalignedPhysAddress);
        }
        if virt_addr.offset() != 0 || virt_addr.lvl3() != 0 || virt_addr.lvl2() != 0 {
            return Err(PageMapError::MisalignedVirtAddress);
        }
        if !virt_addr.is_canonical() {
            return Err(PageMapError::NonCanonicalVirtAddress);
        }

        Ok(())
    }

    pub fn map_large_page(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        _memory_attribute_index: usize,
    ) -> Result<(), PageMapError> {
        if phys_addr & (PAGE_SIZE_2M - 1) != 0 {
            return Err(PageMapError::MisalignedPhysAddress);
        }
        if virt_addr.offset() != 0 || virt_addr.lvl3() != 0 {
            return Err(PageMapError::MisalignedVirtAddress);
        }
        if !virt_addr.is_canonical() {
            return Err(PageMapError::NonCanonicalVirtAddress);
        }

        Ok(())
    }

    pub fn map_small_page(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        memory_attribute_index: usize,
    ) -> Result<(), PageMapError> {
        if phys_addr & (PAGE_SIZE_4K - 1) != 0 {
            return Err(PageMapError::MisalignedPhysAddress);
        }
        if virt_addr.offset() != 0 {
            return Err(PageMapError::MisalignedVirtAddress);
        }
        if !virt_addr.is_canonical() {
            return Err(PageMapError::NonCanonicalVirtAddress);
        }

        let mut table_phys_addr = self.phys_page_table_root as u64;
        let mut level = 0;
        while level < 3 {
            let mut table_entry =
                PageTableEntry::from(self.read_entry(table_phys_addr, virt_addr.lvl_index(level)));
            if !table_entry.valid() {
                let next_table_phys_addr = self.allocate_page_table()?;

                // Without setting the `accessed` flag, qemu fails translation
                // if the HA flag is not enabled in the TCR register. Support for
                // HA in indicated in the MMU features register #1.

                table_entry = PageTableEntry::new()
                    .with_valid(true)
                    .with_table(false)
                    .with_accessed(true)
                    .with_next_table_pfn(next_table_phys_addr >> PAGE_SHIFT_4K);

                self.write_entry(
                    table_phys_addr,
                    virt_addr.lvl_index(level),
                    table_entry.into(),
                );
            }
            table_phys_addr = table_entry.next_table_pfn() << PAGE_SHIFT_4K;

            level += 1;
        }

        let mut page_entry =
            PageBlockEntry::from(self.read_entry(table_phys_addr, virt_addr.lvl_index(level)));
        if page_entry.valid() {
            return Err(PageMapError::AlreadyMapped);
        }

        page_entry = PageBlockEntry::new()
            .with_valid(true)
            .with_page(true)
            .with_accessed(true)
            .with_access_perm(1)
            .with_share_perm(3)
            .with_mair_idx(memory_attribute_index)
            .with_address_pfn(phys_addr >> PAGE_SHIFT_4K);

        self.write_entry(
            table_phys_addr,
            virt_addr.lvl_index(level),
            page_entry.into(),
        );

        Ok(())
    }

    pub fn map_pages(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        map_size: u64,
        page_size: PageSize,
        memory_attribute_index: usize,
    ) -> Result<(), PageMapError> {
        let pages_to_map = map_size / (page_size as u64);
        let mut pages_mapped = 0;
        let mut phys_addr = phys_addr;
        let mut virt_addr = virt_addr.0;
        while pages_mapped < pages_to_map {
            match page_size {
                PageSize::Small => self.map_small_page(
                    phys_addr,
                    VirtualAddress(virt_addr),
                    memory_attribute_index,
                )?,
                PageSize::Large => self.map_large_page(
                    phys_addr,
                    VirtualAddress(virt_addr),
                    memory_attribute_index,
                )?,
                PageSize::Huge => self.map_huge_page(
                    phys_addr,
                    VirtualAddress(virt_addr),
                    memory_attribute_index,
                )?,
            }

            pages_mapped += 1;
            phys_addr += page_size as u64;
            virt_addr += page_size as u64;
        }

        Ok(())
    }

    pub fn map_range(
        &mut self,
        phys_addr: u64,
        virt_addr: VirtualAddress,
        map_size: u64,
        memory_attribute_index: usize,
    ) -> Result<(), PageMapError> {
        if phys_addr & (PAGE_SIZE_4K - 1) != 0 {
            return Err(PageMapError::MisalignedPhysAddress);
        }
        if map_size & (PAGE_SIZE_4K - 1) != 0 {
            return Err(PageMapError::InvalidMappingSize);
        }
        if map_size == 0 {
            return Err(PageMapError::EmptyMapping);
        }
        if virt_addr.offset() != 0 {
            return Err(PageMapError::MisalignedVirtAddress);
        }
        if !virt_addr.is_canonical() {
            return Err(PageMapError::NonCanonicalVirtAddress);
        }

        let mut mapped = 0;
        let mut non_mapped = map_size;
        let mut phys_addr = phys_addr;
        let mut virt_addr = virt_addr.into();

        while mapped < map_size {
            // Try larger pages first, then try to map up to the next large page.
            let map_size = if phys_addr & (PAGE_SIZE_1G - 1) == 0
                && virt_addr & (PAGE_SIZE_1G - 1) == 0
                && non_mapped >= PAGE_SIZE_1G
            {
                let map_size = non_mapped & !(PAGE_SIZE_1G - 1);
                self.map_pages(
                    phys_addr,
                    VirtualAddress(virt_addr),
                    map_size,
                    PageSize::Huge,
                    memory_attribute_index,
                )?;

                map_size
            } else if phys_addr & (PAGE_SIZE_2M - 1) == 0
                && virt_addr & (PAGE_SIZE_2M - 1) == 0
                && non_mapped >= PAGE_SIZE_2M
            {
                let map_size = non_mapped & (PAGE_SIZE_1G - 1) & !(PAGE_SIZE_2M - 1);
                self.map_pages(
                    phys_addr,
                    VirtualAddress(virt_addr),
                    map_size,
                    PageSize::Large,
                    memory_attribute_index,
                )?;

                map_size
            } else {
                let map_size = core::cmp::min(
                    non_mapped & (PAGE_SIZE_1G - 1),
                    non_mapped & (PAGE_SIZE_2M - 1),
                );
                self.map_pages(
                    phys_addr,
                    VirtualAddress(virt_addr),
                    map_size,
                    PageSize::Small,
                    memory_attribute_index,
                )?;

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
