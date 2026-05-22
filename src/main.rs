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
const MEM_SIZE: u64 = 0x10000;
const STATE_FILE: &str = ".asmlings_state";
const EXERCISES_FOLDER: &str = "./exercises";

// ── ANSI helpers
// ──────────────────────────────────────────────────────────────

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const YELLOW: &str = "\x1b[33m";
const GREEN_BG: &str = "\x1b[42;30m";
const RED_BG: &str = "\x1b[41;30m";

// ── Terminal width
// ────────────────────────────────────────────────────────────

fn term_width() -> usize {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        #[repr(C)]
        struct Winsize {
            rows: u16,
            cols: u16,
            xpix: u16,
            ypix: u16,
        }
        let mut ws = Winsize { rows: 0, cols: 0, xpix: 0, ypix: 0 };
        // TIOCGWINSZ = 0x5413 on Linux, 0x40087468 on macOS
        #[cfg(target_os = "macos")]
        const TIOCGWINSZ: u64 = 0x40087468;
        #[cfg(not(target_os = "macos"))]
        const TIOCGWINSZ: u64 = 0x5413;
        let fd = std::io::stderr().as_raw_fd();
        let ok = unsafe { libc::ioctl(fd, TIOCGWINSZ, &mut ws) };
        if ok == 0 && ws.cols > 0 {
            return ws.cols as usize;
        }
    }
    80 // fallback
}

// ── Drawing helpers
// ───────────────────────────────────────────────────────────

/// Print a full-width horizontal rule using `ch`, with 2-space left margin.
fn rule(ch: &str, w: usize) {
    // w is the total terminal width; subtract 2 for the leading "  "
    let inner = w.saturating_sub(2);
    println!("  {DIM}{}{RESET}", ch.repeat(inner));
}

/// Build a full-width banner box.
///  ┌── A S M L I N G S ── · x86 assembly exercises ──────────── v0.1.0 ──┐
///  └────────────────────────────────────────────────────────────────────────┘
fn banner(w: usize, version: &str) {
    // inner width = total - 2 (left margin) - 2 (│ on each side)
    let inner = w.saturating_sub(4);

    let title = "A S M L I N G S";
    let sub = "x86 · 16-bit assembly exercises";
    let ver_tag = format!("v{version}");

    // "  title  ·  sub  " left portion, version right-aligned
    let left = format!("  {title}  ·  {sub}  ");
    // strip ANSI codes are not present here so len() == display width
    let right = format!("  {ver_tag}  ");
    let pad = inner.saturating_sub(left.len() + right.len());

    println!();
    println!("  {BOLD}┌{}┐{RESET}", "─".repeat(inner));
    println!(
        "  {BOLD}│{RESET}{BOLD}{left}{RESET}{DIM}{}{right}{RESET}{BOLD}│{RESET}",
        " ".repeat(pad)
    );
    println!("  {BOLD}└{}┘{RESET}", "─".repeat(inner));
    println!();
}

/// Progress bar that fills the terminal width.
/// Layout:  "  ████████░░░░░░░░░░░░░  10 / 32"
fn progress_bar(current: usize, total: usize, w: usize) {
    let label = format!("  {} / {}  ", current, total);
    // bar area = terminal width - 2 (margin) - label width
    let bar_w = w.saturating_sub(2 + label.len());
    let filled = if total == 0 { 0 } else { current * bar_w / total };
    let empty = bar_w.saturating_sub(filled);

    println!(
        "  {GREEN}{}{RESET}{DIM}{}{RESET}{DIM}{label}{RESET}",
        "█".repeat(filled),
        "░".repeat(empty),
    );
}

// ── State ─────────────────────────────────────────────────────────────────────

fn read_current_index(state_path: &Path) -> usize {
    fs::read_to_string(state_path).ok().and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0)
}

fn write_current_index(state_path: &Path, index: usize) -> anyhow::Result<()> {
    fs::write(state_path, index.to_string())?;
    Ok(())
}

// ── Assertion
// ─────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct RegAssertion {
    reg:      String,
    expected: u16,
}

// ── Exercise
// ──────────────────────────────────────────────────────────────────

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
    let w = term_width();

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
        println!("  {YELLOW}no .asm exercises found in {}{RESET}", exercises_dir.display());
        return Ok(());
    }

    let total = paths.len();
    let current = read_current_index(&state_path);

    // ── Banner ────────────────────────────────────────────────────────────────
    banner(w, env!("CARGO_PKG_VERSION"));

    if current >= total {
        println!(
            "  {GREEN_BG} COMPLETE {RESET}  {BOLD}All {total} exercises done. You're an assembly \
             wizard.{RESET}"
        );
        println!();
        return Ok(());
    }

    let ex = Exercise::load(paths[current].clone())?;
    let display_name = ex.name.replace('_', " ");

    // ── Exercise header ───────────────────────────────────────────────────────
    println!("  {DIM}exercise {}/{total}{RESET}  {BOLD}{display_name}{RESET}", current + 1);
    rule("─", w);
    println!();

    if ex.assertions.is_empty() {
        println!("  {YELLOW}⚠  no ASSERT_REG directives found in this exercise{RESET}");
        println!();
        return Ok(());
    }

    // ── Run & report ──────────────────────────────────────────────────────────
    match run_exercise(&ex) {
        Err(e) => {
            println!("  {RED}✗  error:{RESET} {e}");
        },
        Ok(results) => {
            let mut all_passed = true;

            for assertion in &ex.assertions {
                let actual = results.get(&assertion.reg).copied().unwrap_or(0);
                if actual == assertion.expected {
                    println!(
                        "  {GREEN}✓{RESET}  {BLUE}{}{RESET}  {DIM}=={RESET}  \
                         {GREEN}0x{:04X}{RESET}",
                        assertion.reg, assertion.expected
                    );
                } else {
                    println!(
                        "  {RED}✗{RESET}  {BLUE}{}{RESET}  {DIM}expected{RESET} \
                         {GREEN}0x{:04X}{RESET}  {DIM}got{RESET}  {RED}0x{:04X}{RESET}",
                        assertion.reg, assertion.expected, actual
                    );
                    all_passed = false;
                }
            }

            println!();

            if all_passed {
                println!("  {GREEN_BG} PASS {RESET}  {BOLD}All assertions passed.{RESET}");
                write_current_index(&state_path, current + 1)?;

                if current + 1 >= total {
                    println!();
                    println!(
                        "  {GREEN_BG} COMPLETE {RESET}  {BOLD}You've finished every \
                         exercise!{RESET}"
                    );
                } else {
                    let next = Exercise::load(paths[current + 1].clone())?;
                    let next_display = next.name.replace('_', " ");
                    println!(
                        "  {DIM}next up  {RESET}{BLUE}exercises/{}.asm{RESET}  \
                         {DIM}({next_display}){RESET}",
                        next.name
                    );
                }
            } else {
                println!(
                    "  {RED_BG} FAIL {RESET}  fix the assertions above, then run {DIM}cargo \
                     run{RESET}"
                );
                println!("  {DIM}file     {RESET}{BLUE}exercises/{}.asm{RESET}", ex.name);
            }
        },
    }

    // ── Progress ──────────────────────────────────────────────────────────────
    println!();
    rule("─", w);
    progress_bar(current, total, w);
    println!();

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
