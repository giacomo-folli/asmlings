use unicorn_engine::{
    RegisterX86, Unicorn,
    unicorn_const::{Arch, Mode, Prot},
};

use crate::{
    assembler::{AssembleOutput, assemble},
    constants::{LOAD_ADDR, MEM_BASE, MEM_SIZE},
    exercise::{Assertion, AssertionResult, Exercise, MemAddr},
};

pub fn name_to_reg(name: &str) -> anyhow::Result<RegisterX86> {
    Ok(match name {
        "AX" => RegisterX86::AX,
        "BX" => RegisterX86::BX,
        "CX" => RegisterX86::CX,
        "DX" => RegisterX86::DX,
        "AH" => RegisterX86::AH,
        "AL" => RegisterX86::AL,
        "BH" => RegisterX86::BH,
        "BL" => RegisterX86::BL,
        "CH" => RegisterX86::CH,
        "CL" => RegisterX86::CL,
        "DH" => RegisterX86::DH,
        "DL" => RegisterX86::DL,
        "SP" => RegisterX86::SP,
        "BP" => RegisterX86::BP,
        "SI" => RegisterX86::SI,
        "DI" => RegisterX86::DI,
        other => anyhow::bail!("Unknown register: {}", other),
    })
}

pub fn run_exercise(ex: &Exercise) -> anyhow::Result<Vec<AssertionResult>> {
    let AssembleOutput { code, labels } = assemble(&ex.path)?;

    let mut emu = Unicorn::new(Arch::X86, Mode::MODE_16)
        .map_err(|e| anyhow::anyhow!("Unicorn init failed: {:?}", e))?;

    emu.mem_map(MEM_BASE, MEM_SIZE, Prot::ALL)
        .map_err(|e| anyhow::anyhow!("mem_map failed: {:?}", e))?;

    emu.mem_write(LOAD_ADDR, &code).map_err(|e| anyhow::anyhow!("mem_write failed: {:?}", e))?;

    emu.reg_write(RegisterX86::SP, 0xFFF0)
        .map_err(|e| anyhow::anyhow!("reg_write SP failed: {:?}", e))?;

    let end_addr = LOAD_ADDR + code.len() as u64;

    if let Err(e) = emu.emu_start(LOAD_ADDR, end_addr, 0, 10_000) {
        anyhow::bail!("Emulation failed or timed out (infinite loop?): {:?}", e);
    }

    let mut results = Vec::new();

    for assertion in &ex.assertions {
        let res = match assertion {
            Assertion::Register { reg, expected } => {
                let r = name_to_reg(reg)?;
                let val = emu.reg_read(r)? as u16;
                AssertionResult {
                    passed:       val == *expected,
                    name_str:     reg.clone(),
                    expected_str: format!("0x{:04X}", expected),
                    actual_str:   format!("0x{:04X}", val),
                }
            },

            Assertion::Memory { addr, expected, size } => {
                let resolved = match addr {
                    MemAddr::Literal(n) => *n,
                    MemAddr::Label(label) => *labels.get(label.as_str()).ok_or_else(|| {
                        anyhow::anyhow!(
                            "Label '{}' not found in assembled output. Make sure the label is \
                             defined in the exercise.",
                            label
                        )
                    })?,
                };

                let mut buf = vec![0u8; *size];
                emu.mem_read(resolved, &mut buf)?;

                let val =
                    if *size == 2 { u16::from_le_bytes([buf[0], buf[1]]) } else { buf[0] as u16 };

                let name_str = match addr {
                    MemAddr::Literal(n) => format!("[0x{:04X}]", n),
                    MemAddr::Label(l) => format!("[{}]", l),
                };

                let (expected_str, actual_str) = if *size == 2 {
                    (format!("0x{:04X}", expected), format!("0x{:04X}", val))
                } else {
                    (format!("0x{:02X}", expected), format!("0x{:02X}", val))
                };

                AssertionResult { passed: val == *expected, name_str, expected_str, actual_str }
            },

            Assertion::Flag { flag, expected } => {
                let eflags = emu.reg_read(RegisterX86::EFLAGS)? as u32;
                let bit = match flag.as_str() {
                    "CF" => 0,
                    "PF" => 2,
                    "AF" => 4,
                    "ZF" => 6,
                    "SF" => 7,
                    "OF" => 11,
                    _ => anyhow::bail!("Unknown flag: {}", flag),
                };
                let val = (eflags & (1 << bit)) != 0;
                AssertionResult {
                    passed:       val == *expected,
                    name_str:     flag.clone(),
                    expected_str: if *expected { "1".into() } else { "0".into() },
                    actual_str:   if val { "1".into() } else { "0".into() },
                }
            },
        };

        results.push(res);
    }

    Ok(results)
}
