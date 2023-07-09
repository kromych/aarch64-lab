#!/usr/bin/env python3

# Values found in the Aarch64 system registers in various environments

from collections import OrderedDict
import pprint

data = [
{
    "env": "Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max",
    "registers": [
        OrderedDict({ "name": "MIDR_EL1", "bits": "0x000000000f0510", "revision": "0", "part_num": "51", "architecture": "f", "variant": "0", "implementer": "0" }),
        OrderedDict({ "name": "ID_AA64PFR0_EL1", "bits": "0x01001100110022", "el0": "2", "el1": "2", "el2": "0", "el3": "0", "fp": "1", "adv_simd": "1", "gic": "0", "ras": "0", "sve": "1", "sel2": "1", "mpam": "0", "amu": "0", "dit": "1", "rme": "0", "csv2": "0", "csv3": "0" }),
        OrderedDict({ "name": "ID_AA64PFR1_EL1", "bits": "0x00000000000021", "bt": "1", "ssbs": "2", "mte": "0", "ras_frac": "0", "mpam_frac": "0", "res0": "0", "sme": "0", "rndr_trap": "0", "csv2_frac": "0", "nmi": "0", "mte_frac": "0", "gcs": "0", "the": "0", "mtex": "0", "df2": "0", "pfar": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR0_EL1", "bits": "0x00000000001125", "pa_range": "_48_bits_256TB", "asid_bits": "_16_bits_ASID", "big_end": "1", "sns_mem": "1", "big_end_el0": "0", "t_gran16": "No", "t_gran64": "Yes", "t_gran4": "Yes", "t_gran16_2": "AsStage1", "t_gran64_2": "AsStage1", "t_gran4_2": "AsStage1", "ex_s": "0", "fgt": "0", "ecv": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR1_EL1", "bits": "0x00000010211120", "hafdbs": "0", "vmid_bits": "2", "vh": "1", "hpds": "1", "lo": "1", "pan": "2", "spec_sei": "0", "twed": "1", "xnx": "0", "ets": "0", "hcx": "0", "afp": "0", "n_tlbpa": "0", "tidcp1": "0", "cmow": "0", "ecbhb": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR2_EL1", "bits": "0x00000010000011", "cn_p": "1", "uao": "1", "lsm": "0", "iesb": "0", "va_range": "0", "ccidx": "0", "nv": "0", "st": "1", "at": "0", "ids": "0", "fwb": "0", "res0": "0", "ttl": "0", "bbm": "0", "evt": "0", "e0pd": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR3_EL1", "bits": "0x00000000000000", "tcrx": "0", "sctlrx": "0", "s1pie": "0", "s2pie": "0", "s1poe": "0", "s2poe": "0", "aie": "0", "mec": "0", "d128": "0", "d128_2": "0", "snerr": "0", "anerr": "0", "res0": "0", "sderr": "0", "aderr": "0", "spec_fpacc": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR4_EL1", "bits": "0x00000000000000", "eiesb": "0" }),
    ]
},
{
    "env": "Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, before UEFI",
    "registers": [
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000000c50838", "m": "0", "a": "0", "c": "0", "sa": "1", "sa0": "1", "cp15ben": "1", "n_aa": "0", "itd": "0", "sed": "0", "uma": "0", "en_rctx": "0", "eos": "1", "i": "0", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "1", "n_twe": "1", "wxn": "0", "tscxt": "0", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "0", "lsmaoe": "0", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x00000000000000", "value": "[00, 00, 00, 00, 00, 00]" }),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000000000000", "t0sz": "0", "epd0": "0", "irgn0": "0", "orgn0": "0", "sh0": "0", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "0", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_Invalid", "ips": "_32_bits_4GB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000000000000", "mode": "EL0t", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "false" }),

    ]
},
{
    "env": "Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, after UEFI",
    "registers": [
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000030d0198d", "m": "1", "a": "0", "c": "1", "sa": "1", "sa0": "0", "cp15ben": "0", "n_aa": "0", "itd": "1", "sed": "1", "uma": "0", "en_rctx": "0", "eos": "1", "i": "1", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "0", "n_twe": "0", "wxn": "0", "tscxt": "1", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "1", "lsmaoe": "1", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x000000ffbb4400", "value": "[00, 44, bb, ff, 00, 00, 00, 00]"}),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000580803510", "t0sz": "10", "epd0": "0", "irgn0": "1", "orgn0": "1", "sh0": "3", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "1", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_4KB", "ips": "_48_bits_256TB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000080000a05", "mode": "EL1h", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "true" }),

    ]
},
{
    "env": "Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host",
    "registers": [
        OrderedDict({ "name": "MIDR_EL1", "bits": "0x000000410fd083", "revision": "3", "part_num": "d08", "architecture": "f", "variant": "0", "implementer": "41" }),
        OrderedDict({ "name": "ID_AA64PFR0_EL1", "bits": "0x1000000000002222", "el0": "2", "el1": "2", "el2": "2", "el3": "2", "fp": "0", "adv_simd": "0", "gic": "0", "ras": "0", "sve": "0", "sel2": "0", "mpam": "0", "amu": "0", "dit": "0", "rme": "0", "csv2": "0", "csv3": "1" }),
        OrderedDict({ "name": "ID_AA64PFR1_EL1", "bits": "0x00000000000000", "bt": "0", "ssbs": "0", "mte": "0", "ras_frac": "0", "mpam_frac": "0", "res0": "0", "sme": "0", "rndr_trap": "0", "csv2_frac": "0", "nmi": "0", "mte_frac": "0", "gcs": "0", "the": "0", "mtex": "0", "df2": "0", "pfar": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR0_EL1", "bits": "0x00000000001124", "pa_range": "_44_bits_16TB", "asid_bits": "_16_bits_ASID", "big_end": "1", "sns_mem": "1", "big_end_el0": "0", "t_gran16": "No", "t_gran64": "Yes", "t_gran4": "Yes", "t_gran16_2": "AsStage1", "t_gran64_2": "AsStage1", "t_gran4_2": "AsStage1", "ex_s": "0", "fgt": "0", "ecv": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR1_EL1", "bits": "0x00000000000000", "hafdbs": "0", "vmid_bits": "0", "vh": "0", "hpds": "0", "lo": "0", "pan": "0", "spec_sei": "0", "twed": "0", "xnx": "0", "ets": "0", "hcx": "0", "afp": "0", "n_tlbpa": "0", "tidcp1": "0", "cmow": "0", "ecbhb": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR2_EL1", "bits": "0x00000000000000", "cn_p": "0", "uao": "0", "lsm": "0", "iesb": "0", "va_range": "0", "ccidx": "0", "nv": "0", "st": "0", "at": "0", "ids": "0", "fwb": "0", "res0": "0", "ttl": "0", "bbm": "0", "evt": "0", "e0pd": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR3_EL1", "bits": "0x00000000000000", "tcrx": "0", "sctlrx": "0", "s1pie": "0", "s2pie": "0", "s1poe": "0", "s2poe": "0", "aie": "0", "mec": "0", "d128": "0", "d128_2": "0", "snerr": "0", "anerr": "0", "res0": "0", "sderr": "0", "aderr": "0", "spec_fpacc": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR4_EL1", "bits": "0x00000000000000", "eiesb": "0" }),

    ]
},
{
    "env": "Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host, before UEFI",
    "registers": [
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000000c50838", "m": "0", "a": "0", "c": "0", "sa": "1", "sa0": "1", "cp15ben": "1", "n_aa": "0", "itd": "0", "sed": "0", "uma": "0", "en_rctx": "0", "eos": "1", "i": "0", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "1", "n_twe": "1", "wxn": "0", "tscxt": "0", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "0", "lsmaoe": "0", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x1de7ec7edbadc0de", "value": "[de, c0, ad, db, 7e, ec, e7, 1d]"}),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000000000000", "t0sz": "0", "epd0": "0", "irgn0": "0", "orgn0": "0", "sh0": "0", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "0", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_Invalid", "ips": "_32_bits_4GB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000000000000", "mode": "EL0t", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "false" }),

    ]
},
{
    "env": "Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu host, after UEFI",
    "registers": [
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000030d0198d", "m": "1", "a": "0", "c": "1", "sa": "1", "sa0": "0", "cp15ben": "0", "n_aa": "0", "itd": "1", "sed": "1", "uma": "0", "en_rctx": "0", "eos": "1", "i": "1", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "0", "n_twe": "0", "wxn": "0", "tscxt": "1", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "1", "lsmaoe": "1", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x000000ffbb4400", "value": "[00, 44, bb, ff, 00, 00, 00, 00]"}),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000480803514", "t0sz": "14", "epd0": "0", "irgn0": "1", "orgn0": "1", "sh0": "3", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "1", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_4KB", "ips": "_44_bits_16TB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000060000205", "mode": "EL1h", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "true" }),

    ]
},
{
    "env": "Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max",
    "registers": [
        OrderedDict({ "name": "MIDR_EL1", "bits": "0x000000000f0510", "revision": "0", "part_num": "51", "architecture": "f", "variant": "0", "implementer": "0" }),
        OrderedDict({ "name": "ID_AA64PFR0_EL1", "bits": "0x1201001120110022", "el0": "2", "el1": "2", "el2": "0", "el3": "0", "fp": "1", "adv_simd": "1", "gic": "0", "ras": "2", "sve": "1", "sel2": "1", "mpam": "0", "amu": "0", "dit": "1", "rme": "0", "csv2": "2", "csv3": "1" }),
        OrderedDict({ "name": "ID_AA64PFR1_EL1", "bits": "0x00000001000021", "bt": "1", "ssbs": "2", "mte": "0", "ras_frac": "0", "mpam_frac": "0", "res0": "0", "sme": "1", "rndr_trap": "0", "csv2_frac": "0", "nmi": "0", "mte_frac": "0", "gcs": "0", "the": "0", "mtex": "0", "df2": "0", "pfar": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR0_EL1", "bits": "0x100032310201126", "pa_range": "_52_bits_4PB", "asid_bits": "_16_bits_ASID", "big_end": "1", "sns_mem": "1", "big_end_el0": "0", "t_gran16": "Yes_52bit", "t_gran64": "Yes", "t_gran4": "Yes_52bit", "t_gran16_2": "Yes_52bit", "t_gran64_2": "Yes", "t_gran4_2": "Yes_52bit", "ex_s": "0", "fgt": "1", "ecv": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR1_EL1", "bits": "0x00011010211122", "hafdbs": "2", "vmid_bits": "2", "vh": "1", "hpds": "1", "lo": "1", "pan": "2", "spec_sei": "0", "twed": "1", "xnx": "0", "ets": "1", "hcx": "1", "afp": "0", "n_tlbpa": "0", "tidcp1": "0", "cmow": "0", "ecbhb": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR2_EL1", "bits": "0x1221011010011011", "cn_p": "1", "uao": "1", "lsm": "0", "iesb": "1", "va_range": "1", "ccidx": "0", "nv": "0", "st": "1", "at": "0", "ids": "1", "fwb": "1", "res0": "0", "ttl": "1", "bbm": "2", "evt": "2", "e0pd": "1" }),
        OrderedDict({ "name": "ID_AA64MMFR3_EL1", "bits": "0x00000000000000", "tcrx": "0", "sctlrx": "0", "s1pie": "0", "s2pie": "0", "s1poe": "0", "s2poe": "0", "aie": "0", "mec": "0", "d128": "0", "d128_2": "0", "snerr": "0", "anerr": "0", "res0": "0", "sderr": "0", "aderr": "0", "spec_fpacc": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR4_EL1", "bits": "0x00000000000000", "eiesb": "0" }),

    ]
},
{
    "env": "Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, before UEFI",
    "registers": [
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000000c50838", "m": "0", "a": "0", "c": "0", "sa": "1", "sa0": "1", "cp15ben": "1", "n_aa": "0", "itd": "0", "sed": "0", "uma": "0", "en_rctx": "0", "eos": "1", "i": "0", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "1", "n_twe": "1", "wxn": "0", "tscxt": "0", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "0", "lsmaoe": "0", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0 " }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x00000000000000", "value": "[00, 00, 00, 00, 00, 00, 00, 00]" }),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000000000000", "t0sz": "0", "epd0": "0", "irgn0": "0", "orgn0": "0", "sh0": "0", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "0", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_Invalid", "ips": "_32_bits_4GB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0 " }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000000000000", "mode": "EL0t", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "false " }),

    ]
},
{
    "env": "Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, after UEFI",
    "registers": [
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000030d0198d", "m": "1", "a": "0", "c": "1", "sa": "1", "sa0": "0", "cp15ben": "0", "n_aa": "0", "itd": "1", "sed": "1", "uma": "0", "en_rctx": "0", "eos": "1", "i": "1", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "0", "n_twe": "0", "wxn": "0", "tscxt": "1", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "1", "lsmaoe": "1", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x000000ffbb4400", "value": "[00, 44, bb, ff, 00, 00, 00, 00]" }),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000580803510", "t0sz": "10", "epd0": "0", "irgn0": "1", "orgn0": "1", "sh0": "3", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "1", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_4KB", "ips": "_48_bits_256TB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000020000205", "mode": "EL1h", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "true" }),

    ]
},
{
    "env": "Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host",
    "registers": [
        OrderedDict({ "name": "MIDR_EL1", "bits": "0x00000000000000", "revision": "0", "part_num": "0", "architecture": "0", "variant": "0", "implementer": "0" }),
        OrderedDict({ "name": "ID_AA64PFR0_EL1", "bits": "0x1101000010110011", "el0": "1", "el1": "1", "el2": "0", "el3": "0", "fp": "1", "adv_simd": "1", "gic": "0", "ras": "1", "sve": "0", "sel2": "0", "mpam": "0", "amu": "0", "dit": "1", "rme": "0", "csv2": "1", "csv3": "1" }),
        OrderedDict({ "name": "ID_AA64PFR1_EL1", "bits": "0x00000000000000", "bt": "0", "ssbs": "0", "mte": "0", "ras_frac": "0", "mpam_frac": "0", "res0": "0", "sme": "0", "rndr_trap": "0", "csv2_frac": "0", "nmi": "0", "mte_frac": "0", "gcs": "0", "the": "0", "mtex": "0", "df2": "0", "pfar": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR0_EL1", "bits": "0x0010000f100003", "pa_range": "_42_bits_4TB", "asid_bits": "_8_bits_ASID", "big_end": "0", "sns_mem": "0", "big_end_el0": "0", "t_gran16": "Yes", "t_gran64": "No", "t_gran4": "Yes", "t_gran16_2": "AsStage1", "t_gran64_2": "AsStage1", "t_gran4_2": "AsStage1", "ex_s": "1", "fgt": "0", "ecv": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR1_EL1", "bits": "0x00000011212000", "hafdbs": "0", "vmid_bits": "0", "vh": "0", "hpds": "2", "lo": "1", "pan": "2", "spec_sei": "1", "twed": "1", "xnx": "0", "ets": "0", "hcx": "0", "afp": "0", "n_tlbpa": "0", "tidcp1": "0", "cmow": "0", "ecbhb": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR2_EL1", "bits": "0x1001001100001011", "cn_p": "1", "uao": "1", "lsm": "0", "iesb": "1", "va_range": "0", "ccidx": "0", "nv": "0", "st": "0", "at": "1", "ids": "1", "fwb": "0", "res0": "0", "ttl": "1", "bbm": "0", "evt": "0", "e0pd": "1" }),
        OrderedDict({ "name": "ID_AA64MMFR3_EL1", "bits": "0x00000000000000", "tcrx": "0", "sctlrx": "0", "s1pie": "0", "s2pie": "0", "s1poe": "0", "s2poe": "0", "aie": "0", "mec": "0", "d128": "0", "d128_2": "0", "snerr": "0", "anerr": "0", "res0": "0", "sderr": "0", "aderr": "0", "spec_fpacc": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR4_EL1", "bits": "0x00000000000000", "eiesb": "0" }),

    ]
},
{
    "env": "Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, before UEFI",
    "registers": [
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000030900180", "m": "0", "a": "0", "c": "0", "sa": "0", "sa0": "0", "cp15ben": "0", "n_aa": "0", "itd": "1", "sed": "1", "uma": "0", "en_rctx": "0", "eos": "0", "i": "0", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "0", "n_twe": "0", "wxn": "0", "tscxt": "1", "iesb": "0", "eis": "0", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "1", "lsmaoe": "1", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0 " }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x00000000000000", "value": "[00, 00, 00, 00, 00, 00, 00, 00]" }),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000000000000", "t0sz": "0", "epd0": "0", "irgn0": "0", "orgn0": "0", "sh0": "0", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "0", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_Invalid", "ips": "_32_bits_4GB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0 " }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000000000000", "mode": "EL0t", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "false " }),

    ]
},
{
    "env": "Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, after UEFI",
    "registers": [
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000030d0198d", "m": "1", "a": "0", "c": "1", "sa": "1", "sa0": "0", "cp15ben": "0", "n_aa": "0", "itd": "1", "sed": "1", "uma": "0", "en_rctx": "0", "eos": "1", "i": "1", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "0", "n_twe": "0", "wxn": "0", "tscxt": "1", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "1", "lsmaoe": "1", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x000000ffbb4400", "value": "[00, 44, bb, ff, 00, 00, 00, 00]" }),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000380803516", "t0sz": "16", "epd0": "0", "irgn0": "1", "orgn0": "1", "sh0": "3", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "1", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_4KB", "ips": "_42_bits_4TB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000080000205", "mode": "EL1h", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "true" }),
    ]
},
{
    "env": "ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host",
    "registers": [
        OrderedDict({"name": "MIDR_EL1", "bits": "0x000000410fd4c0", "revision": "0", "part_num": "d4c", "architecture": "f", "variant": "0", "implementer": "41" }),
        OrderedDict({"name": "ID_AA64PFR0_EL1", "bits": "0x1100000001111112", "el0": "2", "el1": "1", "el2": "1", "el3": "1", "fp": "1", "adv_simd": "1", "gic": "1", "ras": "0", "sve": "0", "sel2": "0", "mpam": "0", "amu": "0", "dit": "0", "rme": "0", "csv2": "1", "csv3": "1" }),
        OrderedDict({"name": "ID_AA64PFR1_EL1", "bits": "0x00000000000010", "bt": "0", "ssbs": "1", "mte": "0", "ras_frac": "0", "mpam_frac": "0", "res0": "0", "sme": "0", "rndr_trap": "0", "csv2_frac": "0", "nmi": "0", "mte_frac": "0", "gcs": "0", "the": "0", "mtex": "0", "df2": "0", "pfar": "0" }),
        OrderedDict({"name": "ID_AA64MMFR0_EL1", "bits": "0x00011100101021", "pa_range": "_36_bits_64GB", "asid_bits": "_16_bits_ASID", "big_end": "0", "sns_mem": "1", "big_end_el0": "0", "t_gran16": "Yes", "t_gran64": "Yes", "t_gran4": "Yes", "t_gran16_2": "No", "t_gran64_2": "No", "t_gran4_2": "No", "ex_s": "0", "fgt": "0", "ecv": "0" }),
        OrderedDict({"name": "ID_AA64MMFR1_EL1", "bits": "0x00000000200002", "hafdbs": "2", "vmid_bits": "0", "vh": "0", "hpds": "0", "lo": "0", "pan": "2", "spec_sei": "0", "twed": "0", "xnx": "0", "ets": "0", "hcx": "0", "afp": "0", "n_tlbpa": "0", "tidcp1": "0", "cmow": "0", "ecbhb": "0" }),
        OrderedDict({"name": "ID_AA64MMFR2_EL1", "bits": "0x00000100001010", "cn_p": "0", "uao": "1", "lsm": "0", "iesb": "1", "va_range": "0", "ccidx": "0", "nv": "0", "st": "0", "at": "1", "ids": "0", "fwb": "0", "res0": "0", "ttl": "0", "bbm": "0", "evt": "0", "e0pd": "0" }),
        OrderedDict({"name": "ID_AA64MMFR3_EL1", "bits": "0x00000000000000", "tcrx": "0", "sctlrx": "0", "s1pie": "0", "s2pie": "0", "s1poe": "0", "s2poe": "0", "aie": "0", "mec": "0", "d128": "0", "d128_2": "0", "snerr": "0", "anerr": "0", "res0": "0", "sderr": "0", "aderr": "0", "spec_fpacc": "0" }),
        OrderedDict({"name": "ID_AA64MMFR4_EL1", "bits": "0x00000000000000", "eiesb": "0" }),
    ]
},
{
    "env": "ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host, after UEFI",
    "registers": [
        OrderedDict({"name": "SCTLR_EL1", "bits": "0x00000030d01805", "m": "1", "a": "0", "c": "1", "sa": "0", "sa0": "0", "cp15ben": "0", "n_aa": "0", "itd": "0", "sed": "0", "uma": "0", "en_rctx": "0", "eos": "1", "i": "1", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "0", "n_twe": "0", "wxn": "0", "tscxt": "1", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "1", "lsmaoe": "1", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({"name": "MAIR_EL1", "bits": "0x000000ffbb4400", "value": "[0, 44, bb, ff, 0, 0, 0, 0]"}),
        OrderedDict({"name": "TCR_EL1", "bits": "0x0000018080351c", "t0sz": "1c", "epd0": "0", "irgn0": "1", "orgn0": "1", "sh0": "3", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "1", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_4KB", "ips": "_36_bits_64GB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({"name": "SPSR_EL1", "bits": "0x00000060000005",  "mode": "EL1h", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "false" }),
    ]
},
{
    "env": "MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host",
    "registers": [
        OrderedDict({ "name": "MIDR_EL1", "bits": "0x000000612f0250", "revision": "0", "part_num": "25", "architecture": "f", "variant": "2", "implementer": "61" }),
        OrderedDict({ "name": "ID_AA64PFR0_EL1", "bits": "0x1101000011110111", "el0": "1", "el1": "1", "el2": "1", "el3": "0", "fp": "1", "adv_simd": "1", "gic": "1", "ras": "1", "sve": "0", "sel2": "0", "mpam": "0", "amu": "0", "dit": "1", "rme": "0", "csv2": "1", "csv3": "1" }),
        OrderedDict({ "name": "ID_AA64PFR1_EL1", "bits": "0x00000000000020", "bt": "0", "ssbs": "2", "mte": "0", "ras_frac": "0", "mpam_frac": "0", "res0": "0", "sme": "0", "rndr_trap": "0", "csv2_frac": "0", "nmi": "0", "mte_frac": "0", "gcs": "0", "the": "0", "mtex": "0", "df2": "0", "pfar": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR0_EL1", "bits": "0x0012120f100003", "pa_range": "_42_bits_4TB", "asid_bits": "_8_bits_ASID", "big_end": "0", "sns_mem": "0", "big_end_el0": "0", "t_gran16": "Yes", "t_gran64": "No", "t_gran4": "Yes", "t_gran16_2": "Yes", "t_gran64_2": "No", "t_gran4_2": "Yes", "ex_s": "1", "fgt": "0", "ecv": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR1_EL1", "bits": "0x00000011212100", "hafdbs": "0", "vmid_bits": "0", "vh": "1", "hpds": "2", "lo": "1", "pan": "2", "spec_sei": "1", "twed": "1", "xnx": "0", "ets": "0", "hcx": "0", "afp": "0", "n_tlbpa": "0", "tidcp1": "0", "cmow": "0", "ecbhb": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR2_EL1", "bits": "0x1201011100001011", "cn_p": "1", "uao": "1", "lsm": "0", "iesb": "1", "va_range": "0", "ccidx": "0", "nv": "0", "st": "0", "at": "1", "ids": "1", "fwb": "1", "res0": "0", "ttl": "1", "bbm": "0", "evt": "2", "e0pd": "1" }),
        OrderedDict({ "name": "ID_AA64MMFR3_EL1", "bits": "0x00000000000000", "tcrx": "0", "sctlrx": "0", "s1pie": "0", "s2pie": "0", "s1poe": "0", "s2poe": "0", "aie": "0", "mec": "0", "d128": "0", "d128_2": "0", "snerr": "0", "anerr": "0", "res0": "0", "sderr": "0", "aderr": "0", "spec_fpacc": "0" }),
        OrderedDict({ "name": "ID_AA64MMFR4_EL1", "bits": "0x00000000000000", "eiesb": "0" }),
    ]
},
{
    "env": "MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, after UEFI",
    "registers": [ 
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000030d0198d", "m": "1", "a": "0", "c": "1", "sa": "1", "sa0": "0", "cp15ben": "0", "n_aa": "0", "itd": "1", "sed": "1", "uma": "0", "en_rctx": "0", "eos": "1", "i": "1", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "0", "n_twe": "0", "wxn": "0", "tscxt": "1", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "1", "lsmaoe": "1", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x000000ffbb4400", "value": "[0, 44, bb, ff, 0, 0, 0, 0]"}),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000380803516", "t0sz": "16", "epd0": "0", "irgn0": "1", "orgn0": "1", "sh0": "3", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "1", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_4KB", "ips": "_42_bits_4TB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000080000205", "mode": "EL1h", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "true" }),
    ]
},
{
    "env": "MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, before UEFI",
    "registers": [ 
        OrderedDict({ "name": "SCTLR_EL1", "bits": "0x00000030d501d8", "m": "0", "a": "0", "c": "0", "sa": "1", "sa0": "1", "cp15ben": "0", "n_aa": "1", "itd": "1", "sed": "1", "uma": "0", "en_rctx": "0", "eos": "0", "i": "0", "en_db": "0", "dze": "0", "uct": "0", "n_twi": "1", "n_twe": "1", "wxn": "0", "tscxt": "1", "iesb": "0", "eis": "1", "span": "1", "e0e": "0", "ee": "0", "uci": "0", "en_da": "0", "n_tlsmd": "1", "lsmaoe": "1", "en_ib": "0", "en_ia": "0", "cmow": "0", "msc_en": "0", "bt0": "0", "bt1": "0", "itfsb": "0", "tcf0": "0", "tcf": "0", "ata0": "0", "ata": "0", "dssbs": "0", "twed_en": "0", "twedel": "0", "tmt0": "0", "tmt": "0", "tme0": "0", "tme": "0", "en_asr": "0", "en_as0": "0", "en_als": "0", "epan": "0", "tcso0": "0", "tcso": "0", "en_tp2": "0", "nmi": "0", "spintmask": "0", "tidcp": "0" }),
        OrderedDict({ "name": "MAIR_EL1", "bits": "0x1de7ec7edbadc0de", "value": "[de, c0, ad, db, 7e, ec, e7, 1d]" }),
        OrderedDict({ "name": "TCR_EL1", "bits": "0x00000000000000", "t0sz": "0", "epd0": "0", "irgn0": "0", "orgn0": "0", "sh0": "0", "tg0": "_4KB", "t1sz": "0", "a1": "0", "epd1": "0", "irgn1": "0", "orgn1": "0", "sh1": "0", "tg1": "_Invalid", "ips": "_32_bits_4GB", "a_s": "0", "tbi0": "0", "tbi1": "0", "ha": "0", "hd": "0", "hpd0": "0", "hpd1": "0", "hwu059": "0", "hwu060": "0", "hwu061": "0", "hwu062": "0", "hwu159": "0", "hwu160": "0", "hwu161": "0", "hwu162": "0", "tbid0": "0", "tbid1": "0", "nfd0": "0", "nfd1": "0", "e0pd0": "0", "e0pd1": "0", "tcma0": "0", "tcma1": "0", "ds": "0", "mtx0": "0", "mtx1": "0" }),
        OrderedDict({ "name": "SPSR_EL1", "bits": "0x00000000000000", "mode": "EL0t", "aarch32": "false", "f": "false", "i": "false", "a": "false", "d": "false" }),
    ]
}
]

summary = dict()
reg_names = set()
envs = set()
reg_fields = dict()

for entry in data:
    env = entry["env"]
    registers = entry["registers"]
    for reg in registers:
        reg["env"] = env
        envs.add(env)
        name = reg["name"]
        reg_names.add(name)
        if name in summary:
            summary[name].append(reg)
        else:
            summary[name] = [reg]

reg_names = list(sorted(reg_names))
envs = list(sorted(envs))

for reg_name in summary.keys():
    reg_fields[reg_name] = list(summary[reg_name][0].keys())[1:-1]

print("# Some system registers\n")

for reg_idx, reg_name in enumerate(reg_names):
    print()
    print(f"## {reg_idx}. Register `{reg_name}`\n")
    header = ["Field\Env"]
    reg_envs = list()
    rows = list()

    for field in reg_fields[reg_name]:
        row = [f"`{field}`"]
        for env in envs:
            sample = None
            samples = summary[reg_name]
            for s in samples:
                if s["env"] == env:
                    if env not in reg_envs:
                        reg_envs.append(env)
                    row.append(s[field])
        rows.append(row)

    for idx, env in enumerate(reg_envs):
        print(f"{idx}. {env}")
        header.append(f"`{idx}`")
    print()
    print("|", " | ".join(header), "|")
    print("---".join(["|"]*len(header)), "---|")

    for row in rows:
        print("|", " | ".join(row), "|")
