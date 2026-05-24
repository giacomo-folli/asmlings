use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

use notify::{EventKind, RecursiveMode, Watcher};
use rust_embed::RustEmbed;
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
const YELLOW_BG: &str = "\x1b[43;30m";

#[derive(RustEmbed)]
#[folder = "template_exercises/"]
struct TemplateExercises;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "asmlings")]
#[command(version, about = "x86 · 16-bit assembly exercises", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a folder with all the blank exercises and needed files
    Init,
    /// Launches watch mode on the exercises folder
    Start,
    /// Runs the current exercise once (without watching)
    Run,
}

fn init_mode() -> anyhow::Result<()> {
    let dir = PathBuf::from(EXERCISES_FOLDER);

    if dir.exists() {
        println!("  {YELLOW}⚠  Directory '{}' already exists.{RESET}", EXERCISES_FOLDER);
        return Ok(());
    }

    fs::create_dir_all(&dir)?;
    write_current_index(&dir.join(STATE_FILE), 0)?;

    let mut count = 0;
    for file_path in TemplateExercises::iter() {
        let file = TemplateExercises::get(&file_path).expect("Failed to read embedded file");
        let out_path = dir.join(file_path.as_ref());

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&out_path, file.data)?;
        count += 1;
    }

    println!("  {GREEN}✓{RESET} {BOLD}Initialized {} folder!{RESET}", EXERCISES_FOLDER);
    println!("  {DIM}Extracted {} exercises.{RESET}", count);
    println!("  {DIM}Run {RESET}{BLUE}asmlings start{RESET}{DIM} to begin.{RESET}");

    Ok(())
}

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
    80
}

// ── Drawing helpers
// ───────────────────────────────────────────────────────────

fn rule(ch: &str, w: usize) {
    let inner = w.saturating_sub(2);
    println!("  {DIM}{}{RESET}", ch.repeat(inner));
}

fn banner(w: usize, version: &str) {
    let inner = w.saturating_sub(4);
    let title = "A S M L I N G S";
    let sub = "x86 · 16-bit assembly exercises";
    let ver_tag = format!("v{version}");

    let left = format!("  {title}  ·  {sub}  ");
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

fn progress_bar(current: usize, total: usize, w: usize) {
    let label = format!("  {} / {}  ", current, total);
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

#[derive(Debug, Clone)]
enum MemAddr {
    Literal(u64),
    Label(String),
}

#[derive(Debug, Clone)]
enum Assertion {
    Register { reg: String, expected: u16 },
    Memory { addr: MemAddr, expected: u16, size: usize },
    Flag { flag: String, expected: bool },
}

#[derive(Debug)]
struct AssertionResult {
    passed:       bool,
    name_str:     String,
    expected_str: String,
    actual_str:   String,
}

// ── Exercise
// ──────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Exercise {
    path:       PathBuf,
    name:       String,
    assertions: Vec<Assertion>,
    is_done:    bool,
}

impl Exercise {
    fn load(path: PathBuf) -> anyhow::Result<Self> {
        let src = fs::read_to_string(&path)?;
        let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

        let mut assertions = Vec::new();
        let mut is_done = true;

        for line in src.lines() {
            let line = line.trim();

            if line == "; I AM NOT DONE" {
                is_done = false;
                continue;
            }

            if let Some(rest) = line.strip_prefix(';').map(str::trim) {
                if let Some(reg_rest) = rest.strip_prefix("ASSERT_REG:").map(str::trim) {
                    let parts: Vec<&str> = reg_rest.splitn(3, ' ').collect();
                    if parts.len() == 3 && parts[1] == "==" {
                        let reg = parts[0].to_uppercase();
                        let Some(raw) = parse_u64(parts[2]) else { continue };
                        assertions.push(Assertion::Register { reg, expected: raw as u16 });
                    }
                } else if let Some(mem_rest) = rest.strip_prefix("ASSERT_MEM:").map(str::trim) {
                    let parts: Vec<&str> = mem_rest.splitn(3, ' ').collect();
                    if parts.len() == 3 && parts[1] == "==" {
                        let addr = if let Some(n) = parse_u64(parts[0]) {
                            MemAddr::Literal(n)
                        } else {
                            MemAddr::Label(parts[0].to_string())
                        };

                        let raw_str = parts[2].trim();
                        // Dynamically determine test size based on string definition length
                        let size = if raw_str.starts_with("0x") || raw_str.starts_with("0X") {
                            if raw_str.len() > 4 { 2 } else { 1 }
                        } else {
                            if parse_u64(raw_str).unwrap_or(0) > 255 { 2 } else { 1 }
                        };

                        let Some(raw) = parse_u64(raw_str) else { continue };
                        assertions.push(Assertion::Memory { addr, expected: raw as u16, size });
                    }
                } else if let Some(flag_rest) = rest.strip_prefix("ASSERT_FLAG:").map(str::trim) {
                    let parts: Vec<&str> = flag_rest.splitn(3, ' ').collect();
                    if parts.len() == 3 && parts[1] == "==" {
                        let flag = parts[0].to_uppercase();
                        let expected = parts[2].trim() == "1";
                        assertions.push(Assertion::Flag { flag, expected });
                    }
                }
            }
        }

        Ok(Exercise { path, name, assertions, is_done })
    }
}

// ── Assembly + label resolution
// ───────────────────────────────────────────────

struct AssembleOutput {
    code:   Vec<u8>,
    labels: HashMap<String, u64>,
}

fn assemble(asm_path: &Path) -> anyhow::Result<AssembleOutput> {
    let src = fs::read_to_string(asm_path)?;

    // We enforce org 0x0100 so the absolute addresses match Unicorn's LOAD_ADDR.
    // We also COMMENT OUT `section` directives so NASM evaluates the file as a
    // single continuous block. This guarantees the `.lst` file offsets are
    // absolute and correctly match the generated flat binary layout.
    let mut modified_src = String::from("org 0x0100\n");
    for line in src.lines() {
        let trimmed = line.trim().to_lowercase();
        if trimmed.starts_with("section ")
            || trimmed.starts_with("segment ")
            || trimmed.starts_with("org ")
        {
            modified_src.push_str("; ");
            modified_src.push_str(line);
        } else {
            modified_src.push_str(line);
        }
        modified_src.push('\n');
    }

    let temp_asm_path = asm_path.with_extension("temp.asm");
    let out_path = asm_path.with_extension("bin");
    let lst_path = asm_path.with_extension("lst");

    fs::write(&temp_asm_path, modified_src)?;

    let output_res = Command::new("nasm")
        .args([
            "-f",
            "bin",
            "-l",
            lst_path.to_str().unwrap(),
            "-o",
            out_path.to_str().unwrap(),
            temp_asm_path.to_str().unwrap(),
        ])
        .output();

    let _ = fs::remove_file(&temp_asm_path);

    let output = match output_res {
        Ok(o) => o,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            anyhow::bail!(
                "NASM is not installed or not in your PATH.\n\n  \
                {YELLOW}Asmlings requires the NASM assembler to run.{RESET}\n  \
                {BOLD}To install NASM:{RESET}\n  \
                • macOS:   {GREEN}brew install nasm{RESET}\n  \
                • Ubuntu:  {GREEN}sudo apt install nasm{RESET}\n  \
                • Arch:    {GREEN}sudo pacman -S nasm{RESET}\n  \
                • Windows: {GREEN}winget install NASM{RESET}  {DIM}(or visit https://nasm.us){RESET}"
            );
        },
        Err(e) => anyhow::bail!("Failed to execute NASM: {}", e),
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("NASM syntax error:\n{}", stderr);
    }

    let code = fs::read(&out_path)?;
    let labels = parse_labels(&lst_path);

    let _ = fs::remove_file(&out_path);
    let _ = fs::remove_file(&lst_path);

    Ok(AssembleOutput { code, labels })
}

/// Parse a NASM listing file to extract absolute real addresses for each label.
fn parse_labels(lst_path: &Path) -> HashMap<String, u64> {
    let mut map = HashMap::new();
    let Ok(text) = fs::read_to_string(lst_path) else { return map };

    for line in text.lines() {
        let line_no_comment = line.split(';').next().unwrap_or(line);
        let tokens: Vec<&str> = line_no_comment.split_whitespace().collect();

        if tokens.len() < 3 {
            continue;
        }

        let Ok(addr) = u64::from_str_radix(tokens[1], 16) else { continue };

        for idx in 2..tokens.len() {
            if let Some(colon_idx) = tokens[idx].find(':') {
                let mut label = &tokens[idx][..colon_idx];

                if label.is_empty() && idx > 2 {
                    label = tokens[idx - 1];
                }

                let label = label.trim_start_matches('%');
                if !label.is_empty() && label.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    map.insert(label.to_string(), addr);
                }
                break;
            }
        }
    }

    map
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

fn run_exercise(ex: &Exercise) -> anyhow::Result<Vec<AssertionResult>> {
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

// ── Workflow
// ──────────────────────────────────────────────────────────────────

fn run_workflow() -> anyhow::Result<()> {
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

    println!("  {DIM}exercise {}/{total}{RESET}  {BOLD}{display_name}{RESET}", current + 1);
    rule("─", w);
    println!();

    if ex.assertions.is_empty() {
        println!("  {YELLOW}⚠  no assertions found in this exercise{RESET}");
        println!();
        return Ok(());
    }

    match run_exercise(&ex) {
        Err(e) => {
            println!("  {RED}✗  error:{RESET} {e}");
        },
        Ok(results) => {
            let mut all_passed = true;

            for res in &results {
                if res.passed {
                    println!(
                        "  {GREEN}✓{RESET}  {BLUE}{:<12}{RESET}  {DIM}=={RESET}  {GREEN}{}{RESET}",
                        res.name_str, res.expected_str
                    );
                } else {
                    println!(
                        "  {RED}✗{RESET}  {BLUE}{:<12}{RESET}  {DIM}expected{RESET} \
                         {GREEN}{:<8}{RESET} {DIM}got{RESET} {RED}{}{RESET}",
                        res.name_str, res.expected_str, res.actual_str
                    );
                    all_passed = false;
                }
            }

            println!();

            if all_passed {
                if !ex.is_done {
                    println!(
                        "  {YELLOW_BG} IN PROGRESS {RESET}  {BOLD}Assertions passed, but remove \
                         '; I AM NOT DONE' to advance.{RESET}"
                    );
                } else {
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
                }
            } else {
                println!(
                    "  {RED_BG} FAIL {RESET}  fix the assertions above and save the file to \
                     re-run{RESET}"
                );
                println!("  {DIM}file     {RESET}{BLUE}exercises/{}.asm{RESET}", ex.name);
            }
        },
    }

    println!();
    rule("─", w);
    progress_bar(current, total, w);
    println!();

    Ok(())
}

// ── Watch mode
// ────────────────────────────────────────────────────────────────

fn watch_mode() -> anyhow::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    let _ = run_workflow();

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;

    let exercises_dir = [PathBuf::from(EXERCISES_FOLDER), PathBuf::from("exercises")]
        .into_iter()
        .find(|p| p.is_dir())
        .ok_or_else(|| anyhow::anyhow!("Could not find exercises/ directory to watch"))?;

    watcher.watch(&exercises_dir, RecursiveMode::Recursive)?;

    println!("  {DIM}Watching for file changes in {}...{RESET}", exercises_dir.display());

    let mut last_run = Instant::now();

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if matches!(event.kind, EventKind::Modify(_)) {
                    if last_run.elapsed() > Duration::from_millis(200) {
                        print!("\x1B[2J\x1B[1;1H");

                        if let Err(e) = run_workflow() {
                            println!("  {RED}Fatal error running workflow:{RESET} {}", e);
                        }

                        println!(
                            "  {DIM}Watching for file changes... (Press Ctrl+C to stop){RESET}"
                        );
                        last_run = Instant::now();
                    }
                }
            },
            Ok(Err(e)) => println!("  {RED}Watch error:{RESET} {:?}", e),
            Err(e) => anyhow::bail!("Channel receive error: {:?}", e),
        }
    }
}

// ── Entry point
// ───────────────────────────────────────────────────────────────

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_mode(),
        Commands::Start => watch_mode(),
        Commands::Run => run_workflow(),
    }
}

// ── Utilities
// ─────────────────────────────────────────────────────────────────

fn parse_u64(s: &str) -> Option<u64> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u64>().ok()
    }
}

#[cfg(test)]
mod tests;
