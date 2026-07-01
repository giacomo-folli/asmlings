use unicorn_engine::{
    RegisterX86, Unicorn,
    unicorn_const::{Arch, Mode, Prot},
};

use crate::{
    assembler::{AssembleOutput, assemble},
    constants::{LOAD_ADDR, MEM_BASE, MEM_SIZE},
    exercise::{AssertionResult, Exercise},
};

#[allow(dead_code)]
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
    let suite = crate::harness::get_test_suite(&ex.name)
        .ok_or_else(|| anyhow::anyhow!("No test suite found for exercise: {}", ex.name))?;
    run_programmatic_suite(ex, suite)
}

fn new_emu<'a>(code: &[u8]) -> anyhow::Result<Unicorn<'a, ()>> {
    let mut emu = Unicorn::new(Arch::X86, Mode::MODE_16)
        .map_err(|e| anyhow::anyhow!("Unicorn init failed: {:?}", e))?;

    emu.mem_map(MEM_BASE, MEM_SIZE, Prot::ALL)
        .map_err(|e| anyhow::anyhow!("mem_map failed: {:?}", e))?;

    emu.mem_write(LOAD_ADDR, code)
        .map_err(|e| anyhow::anyhow!("mem_write failed: {:?}", e))?;

    // Initialize stack
    emu.reg_write(RegisterX86::SP, 0xFFF0)
        .map_err(|e| anyhow::anyhow!("reg_write SP failed: {:?}", e))?;

    Ok(emu)
}

pub fn run_programmatic_suite(
    ex: &Exercise,
    suite: &crate::harness::ProgrammaticSuite,
) -> anyhow::Result<Vec<AssertionResult>> {
    let AssembleOutput { code, labels } = assemble(&ex.path)?;
    let mut results = Vec::new();

    for case in &suite.cases {
        let mut emu = new_emu(&code)?;

        // Run setup callback
        (case.setup)(&mut emu, &labels)?;

        let start_addr;
        let end_addr;

        if let Some(target_label) = suite.target_label {
            // Subroutine execution
            let sub_addr = *labels.get(target_label).ok_or_else(|| {
                anyhow::anyhow!(
                    "Label '{}' not found in assembled output. Make sure the subroutine is defined in the exercise.",
                    target_label
                )
            })?;

            // Push sentinel return address (0x0000) onto the stack
            // Standard 16-bit push: decrement SP by 2, write 16-bit word
            let current_sp = emu.reg_read(RegisterX86::SP)?;
            let new_sp = current_sp - 2;
            emu.reg_write(RegisterX86::SP, new_sp)?;
            
            let sentinel_addr: u16 = 0x0000;
            emu.mem_write(new_sp, &sentinel_addr.to_le_bytes())?;

            start_addr = sub_addr;
            end_addr = 0x0000; // Stop when IP reaches 0x0000 (after ret)
        } else {
            // Entire binary execution
            start_addr = LOAD_ADDR;
            end_addr = LOAD_ADDR + code.len() as u64;
        }

        // Execute the sandbox
        if let Err(e) = emu.emu_start(start_addr, end_addr, 0, 10_000) {
            anyhow::bail!(
                "Emulation failed or timed out in test case '{}': {:?}",
                case.name,
                e
            );
        }

        // Verify the results
        let mut case_results = (case.verify)(&emu, &labels)?;
        // Prefix the name of assertion with the case name to make UI output nice
        for res in &mut case_results {
            res.name_str = format!("{}: {}", case.name, res.name_str);
        }
        results.extend(case_results);
    }

    Ok(results)
}

