#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
#[allow(dead_code)]
enum ElfDynTag {
    Null = 0,
    Needed = 1,
    PltRelSz = 2,
    PltGot = 3,
    Hash = 4,
    StrTab = 5,
    SymTab = 6,
    RelA = 7,
    RelASz = 8,
    RelAEnt = 9,
    StrSz = 10,
    SymEnt = 11,
    Init = 12,
    Fini = 13,
    SoName = 14,
    RPath = 15,
    Symbolic = 16,
    Rel = 17,
    RelSz = 18,
    RelEnt = 19,
    PltRel = 20,
    Debug = 21,
    TextRel = 22,
    JmpRel = 23,
    BindNow = 24,
    InitArray = 25,
    FiniArray = 26,
    InitArraySz = 27,
    FiniArraySz = 28,
    RunPath = 29,
    Flags = 30,
    PreinitArray = 32,
    PreinitArraySz = 33,
    GnuPrelinked = 0x6ffffdf5,
    GnuConflictSz = 0x6ffffdf6,
    GnuLiblistSz = 0x6ffffdf7,
    Checksum = 0x6ffffdf8,
    PltpadSz = 0x6ffffdf9,
    MoveEnt = 0x6ffffdfa,
    MoveSz = 0x6ffffdfb,
    Feature1 = 0x6ffffdfc,
    Posflag1 = 0x6ffffdfd,
    SyminSz = 0x6ffffdfe,
    SyminEnt = 0x6ffffdff,
    GnuHash = 0x6ffffef5,
    TlsdescPlt = 0x6ffffef6,
    TlsdescGot = 0x6ffffef7,
    GnuConflict = 0x6ffffef8,
    GnuLibList = 0x6ffffef9,
    Config = 0x6ffffefa,
    DepAudit = 0x6ffffefb,
    Audit = 0x6ffffefc,
    PltPad = 0x6ffffefd,
    MoveTab = 0x6ffffefe,
    SymInfo = 0x6ffffeff,
    VerSym = 0x6ffffff0,
    RelACount = 0x6ffffff9,
    RelCount = 0x6ffffffa,
    Flags1 = 0x6ffffffb,
    VerDef = 0x6ffffffc,
    VerDefNum = 0x6ffffffd,
    VerNeed = 0x6ffffffe,
    VerNeedNum = 0x6fffffff,
    Auxiliary = 0x7ffffffd,
    Filter = 0x7fffffff,
}

const R_AARCH64_RELATIVE: u32 = 0x403;

#[derive(Clone, Copy)]
#[repr(C)]
struct Elf64Dyn {
    tag: ElfDynTag,
    val: usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Elf64Rela {
    offset: u64,
    info: u64,
    addend: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Elf64Rel {
    offset: u64,
    info: u64,
}

fn rela_type(rela: &Elf64Rela) -> u32 {
    rela.info as u32
}

fn rel_type(rel: &Elf64Rel) -> u32 {
    rel.info as u32
}

#[no_mangle]
fn apply_rel(load_addr: u64, begin: usize, end: usize) {
    let rel = unsafe {
        core::slice::from_raw_parts_mut(
            begin as *mut Elf64Rel,
            (end - begin) / core::mem::size_of::<Elf64Rel>(),
        )
    };
    for rel in rel {
        if rel_type(rel) != R_AARCH64_RELATIVE {
            panic!("rel")
        }

        unsafe {
            let rel = (rel.offset + load_addr) as *mut u64;
            *rel = (*rel).wrapping_add(load_addr);
        }
    }
}

#[no_mangle]
fn apply_rela(load_addr: u64, begin: usize, end: usize) {
    let rela = unsafe {
        core::slice::from_raw_parts_mut(
            begin as *mut Elf64Rela,
            (end - begin) / core::mem::size_of::<Elf64Rela>(),
        )
    };
    for rel in rela {
        if rela_type(rel) != R_AARCH64_RELATIVE {
            panic!("rela")
        }

        unsafe {
            *((rel.offset + load_addr) as *mut u64) = rel.addend + load_addr;
        }
    }
}

#[no_mangle]
unsafe extern "C" fn relocate(load_addr: usize, begin: usize) {
    if load_addr == begin {
        // Empty dynamic section or wrong linker flags (no PIE?),
        // exit
        return;
    }

    let mut rela_offset = None;
    let mut rela_entry_size = 0;
    let mut rela_count = 0;

    let mut rel_offset = None;
    let mut rel_entry_size = 0;
    let mut rel_count = 0;

    let mut dynamic = begin as *mut Elf64Dyn;
    while dynamic.read_unaligned().tag != ElfDynTag::Null {
        match (*dynamic).tag {
            ElfDynTag::RelA => {
                rela_offset = Some((*dynamic).val);
            }
            ElfDynTag::RelAEnt => {
                rela_entry_size = (*dynamic).val;
            }
            ElfDynTag::Rel => {
                rel_offset = Some((*dynamic).val);
            }
            ElfDynTag::RelEnt => {
                rel_entry_size = (*dynamic).val;
            }
            ElfDynTag::RelACount => {
                rela_count = (*dynamic).val;
            }
            ElfDynTag::RelCount => {
                rel_count = (*dynamic).val;
            }
            _ => {}
        }

        dynamic = dynamic.wrapping_add(1);
    }

    if let Some(rela_offset) = rela_offset {
        const RELA_ENTRY_SIZE: usize = core::mem::size_of::<Elf64Rela>();
        if rela_entry_size != RELA_ENTRY_SIZE {
            panic!();
        }

        let begin = load_addr + rela_offset;
        let end = begin + rela_count * RELA_ENTRY_SIZE;
        apply_rela(load_addr as u64, begin, end);
    }

    if let Some(rel_offset) = rel_offset {
        const REL_ENTRY_SIZE: usize = core::mem::size_of::<Elf64Rel>();
        if rel_entry_size != REL_ENTRY_SIZE {
            panic!();
        }

        let begin = load_addr + rel_offset;
        let end = begin + rel_count * REL_ENTRY_SIZE;
        apply_rel(load_addr as u64, begin, end);
    }
}
