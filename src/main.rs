use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use unicorn_engine::{
    RegisterX86, Unicorn,
    unicorn_const::{Arch, Mode, Prot},
};

const LOAD_ADDR: u64 = 0x0100;
const MEM_BASE: u64 = 0x0000;
const MEM_SIZE: u64 = 0x10000; // 64 KB
const STATE_FILE: &str = ".asmlings_state";
const EXERCISES_FOLDER: &str = "./exercises";

// ── State ─────────────────────────────────────────────────────────────────────

fn read_current_index(state_path: &Path) -> usize {
    fs::read_to_string(state_path).ok().and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0)
}

fn write_current_index(state_path: &Path, index: usize) -> anyhow::Result<()> {
    fs::write(state_path, index.to_string())?;
    Ok(())
}

// ── Assertion parsed from exercise comments
// ───────────────────────────────────

#[derive(Debug)]
struct RegAssertion {
    reg:      String,
    expected: u16,
}

// ── Exercise metadata
// ─────────────────────────────────────────────────────────

#[derive(Debug)]
struct Exercise {
    path:       PathBuf,
    name:       String,
    assertions: Vec<RegAssertion>,
}

impl Exercise {
    fn load(path: PathBuf) -> anyhow::Result<Self> {
        let src = fs::read_to_string(&path)?;
        let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

        let mut assertions = Vec::new();
        for line in src.lines() {
            let line = line.trim();
            if let Some(rest) = line
                .strip_prefix(';')
                .map(str::trim)
                .and_then(|l| l.strip_prefix("ASSERT_REG:").map(str::trim))
            {
                let parts: Vec<&str> = rest.splitn(3, ' ').collect();
                if parts.len() == 3 && parts[1] == "==" {
                    let reg = parts[0].to_uppercase();
                    let val_str = parts[2].trim();
                    let expected = parse_u16(val_str).ok_or_else(|| {
                        anyhow::anyhow!("Cannot parse assertion value: {}", val_str)
                    })?;
                    assertions.push(RegAssertion { reg, expected });
                }
            }
        }

        Ok(Exercise { path, name, assertions })
    }
}

// ── Assembly + Emulation
// ──────────────────────────────────────────────────────

fn assemble(asm_path: &Path) -> anyhow::Result<Vec<u8>> {
    let out_path = asm_path.with_extension("bin");
    let output = Command::new("nasm")
        .args(["-f", "bin", "-o", out_path.to_str().unwrap(), asm_path.to_str().unwrap()])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("NASM error:\n{}", stderr);
    }

    let bytes = fs::read(&out_path)?;
    let _ = fs::remove_file(&out_path);
    Ok(bytes)
}

fn name_to_reg(name: &str) -> anyhow::Result<RegisterX86> {
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

fn run_exercise(ex: &Exercise) -> anyhow::Result<HashMap<String, u16>> {
    let code = assemble(&ex.path)?;

    let mut emu = Unicorn::new(Arch::X86, Mode::MODE_16)
        .map_err(|e| anyhow::anyhow!("Unicorn init failed: {:?}", e))?;

    emu.mem_map(MEM_BASE, MEM_SIZE, Prot::ALL)
        .map_err(|e| anyhow::anyhow!("mem_map failed: {:?}", e))?;

    emu.mem_write(LOAD_ADDR, &code).map_err(|e| anyhow::anyhow!("mem_write failed: {:?}", e))?;

    emu.reg_write(RegisterX86::SP, 0xFFF0)
        .map_err(|e| anyhow::anyhow!("reg_write SP failed: {:?}", e))?;

    let end_addr = LOAD_ADDR + code.len() as u64;
    emu.emu_start(LOAD_ADDR, end_addr, 0, 0)
        .map_err(|e| anyhow::anyhow!("emu_start failed: {:?}", e))?;

    let mut results = HashMap::new();
    for assertion in &ex.assertions {
        let reg = name_to_reg(&assertion.reg)?;
        let val = emu
            .reg_read(reg)
            .map_err(|e| anyhow::anyhow!("reg_read {} failed: {:?}", assertion.reg, e))?;
        results.insert(assertion.reg.clone(), val as u16);
    }
    Ok(results)
}

// ── Entry point
// ───────────────────────────────────────────────────────────────

fn main() -> anyhow::Result<()> {
    let exercises_dir = [PathBuf::from(EXERCISES_FOLDER), PathBuf::from("exercises")]
        .into_iter()
        .find(|p| p.is_dir())
        .ok_or_else(|| anyhow::anyhow!("Could not find exercises/ directory"))?;

    let state_path = exercises_dir.join(STATE_FILE);

    let mut paths: Vec<PathBuf> = fs::read_dir(&exercises_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("asm"))
        .collect();
    paths.sort();

    if paths.is_empty() {
        println!("No .asm exercises found in {}", exercises_dir.display());
        return Ok(());
    }

    let current = read_current_index(&state_path);

    if current >= paths.len() {
        println!("\n  🎉  All {} exercises complete!", paths.len());
        return Ok(());
    }

    let ex = Exercise::load(paths[current].clone())?;
    let display_name = ex.name.replace('_', " ").to_uppercase();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                  A S M L I N G S                        ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("\n  Exercise {}/{}: {}", current + 1, paths.len(), display_name);
    println!("{}", "─".repeat(60));

    if ex.assertions.is_empty() {
        println!("  ⚠  No ASSERT_REG directives found in this exercise.");
        return Ok(());
    }

    match run_exercise(&ex) {
        Err(e) => {
            println!("  ✗  {}", e);
        },
        Ok(results) => {
            let mut all_passed = true;
            for assertion in &ex.assertions {
                let actual = results.get(&assertion.reg).copied().unwrap_or(0);
                if actual == assertion.expected {
                    println!("  ✓  {} == 0x{:04X}", assertion.reg, assertion.expected);
                } else {
                    println!(
                        "  ✗  {}: expected 0x{:04X}, got 0x{:04X}",
                        assertion.reg, assertion.expected, actual
                    );
                    all_passed = false;
                }
            }

            if all_passed {
                println!("\n  🎉  Passed! Moving to the next exercise.");
                write_current_index(&state_path, current + 1)?;

                if current + 1 >= paths.len() {
                    println!("  🏆  You've completed all exercises!");
                } else {
                    let next = Exercise::load(paths[current + 1].clone())?;
                    println!("  👉  Next up: exercises/{}.asm", next.name);
                }
            } else {
                println!("\n  👉  Keep working on: exercises/{}.asm", ex.name);
            }
        },
    }

    println!("{}\n", "─".repeat(60));
    Ok(())
}

// ── Utilities
// ─────────────────────────────────────────────────────────────────

fn parse_u16(s: &str) -> Option<u16> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u16::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u16>().ok()
    }
}
