# Aarch64 low-level lab

Here you can find some low-level Aarch64 code that can run on the bare processor
inside a virtual machine, i.e. prior to any firmware or an OS.

The repo produces a position-independent executable able to relocate itself, log the
register values to the PL011 port, and can enable the MMU. The 4KiB translation granule size
is used, and the optimal mix of page sizes is chosen when mapping. There are tests present,
and, in particlular, one test that generates code on the fly.

The code showcases using semihosting on Aarch64, too.

## 0. Register `ID_AA64MMFR0_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00011100101021 | 0x0010000f100003 | 0x100032310201126 | 0x0012120f100003 | 0x10010000f100003 | 0x00000000001124 | 0x00000000001125 |
| `pa_range` | _36_bits_64GB | _42_bits_4TB | _52_bits_4PB | _42_bits_4TB | _42_bits_4TB | _44_bits_16TB | _48_bits_256TB |
| `asid_bits` | _16_bits_ASID | _8_bits_ASID | _16_bits_ASID | _8_bits_ASID | _8_bits_ASID | _16_bits_ASID | _16_bits_ASID |
| `big_end` | 0 | 0 | 1 | 0 | 0 | 1 | 1 |
| `sns_mem` | 1 | 0 | 1 | 0 | 0 | 1 | 1 |
| `big_end_el0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `t_gran16` | Yes | Yes | Yes_52bit | Yes | Yes | No | No |
| `t_gran64` | Yes | No | Yes | No | No | Yes | Yes |
| `t_gran4` | Yes | Yes | Yes_52bit | Yes | Yes | Yes | Yes |
| `t_gran16_2` | No | AsStage1 | Yes_52bit | Yes | AsStage1 | AsStage1 | AsStage1 |
| `t_gran64_2` | No | AsStage1 | Yes | No | AsStage1 | AsStage1 | AsStage1 |
| `t_gran4_2` | No | AsStage1 | Yes_52bit | Yes | AsStage1 | AsStage1 | AsStage1 |
| `ex_s` | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| `fgt` | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| `ecv` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 1. Register `ID_AA64MMFR1_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000000200002 | 0x00000011212000 | 0x00011010211122 | 0x00000011212100 | 0x00000011312000 | 0x00000000000000 | 0x00000010211120 |
| `hafdbs` | 2 | 0 | 2 | 0 | 0 | 0 | 0 |
| `vmid_bits` | 0 | 0 | 2 | 0 | 0 | 0 | 2 |
| `vh` | 0 | 0 | 1 | 1 | 0 | 0 | 1 |
| `hpds` | 0 | 2 | 1 | 2 | 2 | 0 | 1 |
| `lo` | 0 | 1 | 1 | 1 | 1 | 0 | 1 |
| `pan` | 2 | 2 | 2 | 2 | 3 | 0 | 2 |
| `spec_sei` | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| `twed` | 0 | 1 | 1 | 1 | 1 | 0 | 1 |
| `xnx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ets` | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| `hcx` | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| `afp` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `n_tlbpa` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tidcp1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `cmow` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ecbhb` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 2. Register `ID_AA64MMFR2_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000100001010 | 0x1001001100001011 | 0x1221011010011011 | 0x1201011100001011 | 0x1001001100001011 | 0x00000000000000 | 0x00000010000011 |
| `cn_p` | 0 | 1 | 1 | 1 | 1 | 0 | 1 |
| `uao` | 1 | 1 | 1 | 1 | 1 | 0 | 1 |
| `lsm` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `iesb` | 1 | 1 | 1 | 1 | 1 | 0 | 0 |
| `va_range` | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| `ccidx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `nv` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `st` | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| `at` | 1 | 1 | 0 | 1 | 1 | 0 | 0 |
| `ids` | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| `fwb` | 0 | 0 | 1 | 1 | 0 | 0 | 0 |
| `res0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ttl` | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| `bbm` | 0 | 0 | 2 | 0 | 0 | 0 | 0 |
| `evt` | 0 | 0 | 2 | 2 | 0 | 0 | 0 |
| `e0pd` | 0 | 1 | 1 | 1 | 1 | 0 | 0 |

## 3. Register `ID_AA64MMFR3_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 |
| `tcrx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `sctlrx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `s1pie` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `s2pie` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `s1poe` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `s2poe` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `aie` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `mec` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `d128` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `d128_2` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `snerr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `anerr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `res0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `sderr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `aderr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `spec_fpacc` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 4. Register `ID_AA64MMFR4_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 |
| `eiesb` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 5. Register `ID_AA64PFR0_EL1`

Traceback (most recent call last):
  File "/Volumes/src/virt/aarch64-lab/tools/registers-aarch64.py", line 256, in <module>
    row.append(s[field])
               ~^^^^^^^
KeyError: 'csv3'
krom@krom3m aarch64-lab % tools/registers-aarch64.py
# Some system registers


## 0. Register `ID_AA64MMFR0_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00011100101021 | 0x0010000f100003 | 0x100032310201126 | 0x0012120f100003 | 0x10010000f100003 | 0x00000000001124 | 0x00000000001125 |
| `pa_range` | _36_bits_64GB | _42_bits_4TB | _52_bits_4PB | _42_bits_4TB | _42_bits_4TB | _44_bits_16TB | _48_bits_256TB |
| `asid_bits` | _16_bits_ASID | _8_bits_ASID | _16_bits_ASID | _8_bits_ASID | _8_bits_ASID | _16_bits_ASID | _16_bits_ASID |
| `big_end` | 0 | 0 | 1 | 0 | 0 | 1 | 1 |
| `sns_mem` | 1 | 0 | 1 | 0 | 0 | 1 | 1 |
| `big_end_el0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `t_gran16` | Yes | Yes | Yes_52bit | Yes | Yes | No | No |
| `t_gran64` | Yes | No | Yes | No | No | Yes | Yes |
| `t_gran4` | Yes | Yes | Yes_52bit | Yes | Yes | Yes | Yes |
| `t_gran16_2` | No | AsStage1 | Yes_52bit | Yes | AsStage1 | AsStage1 | AsStage1 |
| `t_gran64_2` | No | AsStage1 | Yes | No | AsStage1 | AsStage1 | AsStage1 |
| `t_gran4_2` | No | AsStage1 | Yes_52bit | Yes | AsStage1 | AsStage1 | AsStage1 |
| `ex_s` | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| `fgt` | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| `ecv` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 1. Register `ID_AA64MMFR1_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000000200002 | 0x00000011212000 | 0x00011010211122 | 0x00000011212100 | 0x00000011312000 | 0x00000000000000 | 0x00000010211120 |
| `hafdbs` | 2 | 0 | 2 | 0 | 0 | 0 | 0 |
| `vmid_bits` | 0 | 0 | 2 | 0 | 0 | 0 | 2 |
| `vh` | 0 | 0 | 1 | 1 | 0 | 0 | 1 |
| `hpds` | 0 | 2 | 1 | 2 | 2 | 0 | 1 |
| `lo` | 0 | 1 | 1 | 1 | 1 | 0 | 1 |
| `pan` | 2 | 2 | 2 | 2 | 3 | 0 | 2 |
| `spec_sei` | 0 | 1 | 0 | 1 | 1 | 0 | 0 |
| `twed` | 0 | 1 | 1 | 1 | 1 | 0 | 1 |
| `xnx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ets` | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| `hcx` | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| `afp` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `n_tlbpa` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tidcp1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `cmow` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ecbhb` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 2. Register `ID_AA64MMFR2_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000100001010 | 0x1001001100001011 | 0x1221011010011011 | 0x1201011100001011 | 0x1001001100001011 | 0x00000000000000 | 0x00000010000011 |
| `cn_p` | 0 | 1 | 1 | 1 | 1 | 0 | 1 |
| `uao` | 1 | 1 | 1 | 1 | 1 | 0 | 1 |
| `lsm` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `iesb` | 1 | 1 | 1 | 1 | 1 | 0 | 0 |
| `va_range` | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| `ccidx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `nv` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `st` | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| `at` | 1 | 1 | 0 | 1 | 1 | 0 | 0 |
| `ids` | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| `fwb` | 0 | 0 | 1 | 1 | 0 | 0 | 0 |
| `res0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ttl` | 0 | 1 | 1 | 1 | 1 | 0 | 0 |
| `bbm` | 0 | 0 | 2 | 0 | 0 | 0 | 0 |
| `evt` | 0 | 0 | 2 | 2 | 0 | 0 | 0 |
| `e0pd` | 0 | 1 | 1 | 1 | 1 | 0 | 0 |

## 3. Register `ID_AA64MMFR3_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 |
| `tcrx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `sctlrx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `s1pie` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `s2pie` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `s1poe` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `s2poe` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `aie` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `mec` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `d128` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `d128_2` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `snerr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `anerr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `res0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `sderr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `aderr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `spec_fpacc` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 4. Register `ID_AA64MMFR4_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 |
| `eiesb` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 5. Register `ID_AA64PFR0_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x1100000001111112 | 0x1101000010110011 | 0x1201001120110022 | 0x1101000011110111 | 0x1101000010110011 | 0x1000000000002222 | 0x01001100110022 |
| `el0` | 2 | 1 | 2 | 1 | 1 | 2 | 2 |
| `el1` | 1 | 1 | 2 | 1 | 1 | 2 | 2 |
| `el2` | 1 | 0 | 0 | 1 | 0 | 2 | 0 |
| `el3` | 1 | 0 | 0 | 0 | 0 | 2 | 0 |
| `fp` | 1 | 1 | 1 | 1 | 1 | 0 | 1 |
| `adv_simd` | 1 | 1 | 1 | 1 | 1 | 0 | 1 |
| `gic` | 1 | 0 | 0 | 1 | 0 | 0 | 0 |
| `ras` | 0 | 1 | 2 | 1 | 1 | 0 | 0 |
| `sve` | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| `sel2` | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| `mpam` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `amu` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `dit` | 0 | 1 | 1 | 1 | 1 | 0 | 1 |
| `rme` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `csv2` | 1 | 1 | 2 | 1 | 1 | 0 | 0 |
| `csv3` | 1 | 1 | 1 | 1 | 1 | 1 | 0 |

## 6. Register `ID_AA64PFR1_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
6. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` |
|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000000000010 | 0x00000000000000 | 0x00000001000021 | 0x00000000000020 | 0x00000000000000 | 0x00000000000000 | 0x00000000000021 |
| `bt` | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| `ssbs` | 1 | 0 | 2 | 2 | 0 | 0 | 2 |
| `mte` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ras_frac` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `mpam_frac` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `res0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `sme` | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| `rndr_trap` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `csv2_frac` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `nmi` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `mte_frac` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `gcs` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `the` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `mtex` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `df2` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `pfar` | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 7. Register `MAIR_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host, after UEFI
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, after UEFI
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, before UEFI
3. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, after UEFI
4. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, before UEFI
5. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, after UEFI
6. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, before UEFI
7. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
8. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host, before UEFI
9. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu host, after UEFI
10. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, after UEFI
11. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, before UEFI

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` | `10` | `11` |
|---|---|---|---|---|---|---|---|---|---|---|---| ---|
| `bits` | 0x000000ffbb4400 | 0x000000ffbb4400 | 0x00000000000000 | 0x000000ffbb4400 | 0x00000000000000 | 0x000000ffbb4400 | 0x1de7ec7edbadc0de | 0x000000bbff4400 | 0x1de7ec7edbadc0de | 0x000000ffbb4400 | 0x000000ffbb4400 | 0x00000000000000 |
| `value` | [0, 44, bb, ff, 0, 0, 0, 0] | [00, 44, bb, ff, 00, 00, 00, 00] | [00, 00, 00, 00, 00, 00, 00, 00] | [00, 44, bb, ff, 00, 00, 00, 00] | [00, 00, 00, 00, 00, 00, 00, 00] | [0, 44, bb, ff, 0, 0, 0, 0] | [de, c0, ad, db, 7e, ec, e7, 1d] | [0, 44, 0xff, 0xbb, 0, 0, 0, 0] | [de, c0, ad, db, 7e, ec, e7, 1d] | [00, 44, bb, ff, 00, 00, 00, 00] | [00, 44, bb, ff, 00, 00, 00, 00] | [00, 00, 00, 00, 00, 00] |

## 8. Register `MIDR_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max
3. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host
4. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host
5. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` |
|---|---|---|---|---|---| ---|
| `bits` | 0x000000410fd4c0 | 0x00000000000000 | 0x000000000f0510 | 0x000000612f0250 | 0x000000410fd083 | 0x000000000f0510 |
| `revision` | 0 | 0 | 0 | 0 | 3 | 0 |
| `part_num` | d4c | 0 | 51 | 25 | d08 | 51 |
| `architecture` | f | 0 | f | f | f | f |
| `variant` | 0 | 0 | 0 | 2 | 0 | 0 |
| `implementer` | 41 | 0 | 0 | 61 | 41 | 0 |

## 9. Register `SCTLR_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host, after UEFI
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, after UEFI
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, before UEFI
3. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, after UEFI
4. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, before UEFI
5. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, after UEFI
6. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, before UEFI
7. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
8. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host, before UEFI
9. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu host, after UEFI
10. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, after UEFI
11. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, before UEFI

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` | `10` | `11` |
|---|---|---|---|---|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000030d01805 | 0x00000030d0198d | 0x00000030900180 | 0x00000030d0198d | 0x00000000c50838 | 0x00000030d0198d | 0x00000030d501d8 | 0x00000030901187 | 0x00000000c50838 | 0x00000030d0198d | 0x00000030d0198d | 0x00000000c50838 |
| `m` | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 |
| `a` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 |
| `c` | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 |
| `sa` | 0 | 1 | 0 | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 1 |
| `sa0` | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 1 |
| `cp15ben` | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 1 |
| `n_aa` | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| `itd` | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |
| `sed` | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |
| `uma` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `en_rctx` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `eos` | 1 | 1 | 0 | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 1 |
| `i` | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 |
| `en_db` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `dze` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `uct` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `n_twi` | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 1 |
| `n_twe` | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 | 1 |
| `wxn` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tscxt` | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |
| `iesb` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `eis` | 1 | 1 | 0 | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 1 |
| `span` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| `e0e` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ee` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `uci` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `en_da` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `n_tlsmd` | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |
| `lsmaoe` | 1 | 1 | 1 | 1 | 0 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |
| `en_ib` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `en_ia` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `cmow` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `msc_en` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `bt0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `bt1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `itfsb` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tcf0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tcf` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ata0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ata` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `dssbs` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `twed_en` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `twedel` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tmt0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tmt` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tme0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tme` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `en_asr` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `en_as0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `en_als` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `epan` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tcso0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tcso` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `en_tp2` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `nmi` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `spintmask` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tidcp` | 0 | 0 | 0  | 0 | 0  | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## 10. Register `SPSR_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host, after UEFI
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, after UEFI
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, before UEFI
3. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, after UEFI
4. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, before UEFI
5. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, after UEFI
6. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, before UEFI
7. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
8. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host, before UEFI
9. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu host, after UEFI
10. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, after UEFI
11. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, before UEFI

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` | `10` | `11` |
|---|---|---|---|---|---|---|---|---|---|---|---| ---|
| `bits` | 0x00000060000005 | 0x00000080000205 | 0x00000000000000 | 0x00000020000205 | 0x00000000000000 | 0x00000080000205 | 0x00000000000000 | 0x00000000000000 | 0x00000000000000 | 0x00000060000205 | 0x00000080000a05 | 0x00000000000000 |
| `mode` | EL1h | EL1h | EL0t | EL1h | EL0t | EL1h | EL0t | EL0t | EL0t | EL1h | EL1h | EL0t |
| `aarch32` | false | false | false | false | false | false | false | false | false | false | false | false |
| `f` | false | false | false | false | false | false | false | false | false | false | false | false |
| `i` | false | false | false | false | false | false | false | false | false | false | false | false |
| `a` | false | false | false | false | false | false | false | false | false | false | false | false |
| `d` | false | true | false  | true | false  | true | false | false | false | true | true | false |

## 11. Register `TCR_EL1`

0. ARM64 Dev Kit for Windows, Windows 11, Hyper-V, cpu host, after UEFI
1. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, after UEFI
2. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/HVF, cpu host, before UEFI
3. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, after UEFI
4. Mac Studio M1 Ultra, macOS 15, QEMU 8.0/TCG, cpu max, before UEFI
5. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, after UEFI
6. MacBook M1 Pro, Asahi Linux, QEMU 8.0/KVM, cpu host, before UEFI
7. MacBook M3 Max, macOS 14, QEMU 8.0/HVF, cpu host
8. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/KVM, cpu host, before UEFI
9. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu host, after UEFI
10. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, after UEFI
11. Raspberry Pi 4, Ubuntu 22.04, QEMU 6.2/TCG, cpu max, before UEFI

| Field\Env | `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` | `10` | `11` |
|---|---|---|---|---|---|---|---|---|---|---|---| ---|
| `bits` | 0x0000018080351c | 0x00000380803516 | 0x00000000000000 | 0x00000580803510 | 0x00000000000000 | 0x00000380803516 | 0x00000000000000 | 0x00000580803510 | 0x00000000000000 | 0x00000480803514 | 0x00000580803510 | 0x00000000000000 |
| `t0sz` | 1c | 16 | 0 | 10 | 0 | 16 | 0 | 10 | 0 | 14 | 10 | 0 |
| `epd0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `irgn0` | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 |
| `orgn0` | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 |
| `sh0` | 3 | 3 | 0 | 3 | 0 | 3 | 0 | 3 | 0 | 3 | 3 | 0 |
| `tg0` | _4KB | _4KB | _4KB | _4KB | _4KB | _4KB | _4KB | _4KB | _4KB | _4KB | _4KB | _4KB |
| `t1sz` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `a1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `epd1` | 1 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 1 | 1 | 0 |
| `irgn1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `orgn1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `sh1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tg1` | _4KB | _4KB | _Invalid | _4KB | _Invalid | _4KB | _Invalid | _4KB | _Invalid | _4KB | _4KB | _Invalid |
| `ips` | _36_bits_64GB | _42_bits_4TB | _32_bits_4GB | _48_bits_256TB | _32_bits_4GB | _42_bits_4TB | _32_bits_4GB | _48_bits_256TB | _32_bits_4GB | _44_bits_16TB | _48_bits_256TB | _32_bits_4GB |
| `a_s` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tbi0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tbi1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ha` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hd` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hpd0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hpd1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hwu059` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hwu060` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hwu061` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hwu062` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hwu159` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hwu160` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hwu161` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `hwu162` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tbid0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tbid1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `nfd0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `nfd1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `e0pd0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `e0pd1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tcma0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `tcma1` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `ds` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `mtx0` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `mtx1` | 0 | 0 | 0  | 0 | 0  | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
