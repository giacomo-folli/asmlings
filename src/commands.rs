use std::{
    fs,
    path::PathBuf,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

use notify::{EventKind, RecursiveMode, Watcher};
use rust_embed::RustEmbed;

use crate::{
    constants::*,
    emulator::run_exercise,
    exercise::Exercise,
    state::{read_current_index, write_current_index},
    ui::{banner, progress_bar, rule, term_width},
};

#[derive(RustEmbed)]
#[folder = "template_exercises/"]
struct TemplateExercises;

pub fn init_mode(force: bool) -> anyhow::Result<()> {
    init_mode_in_path(PathBuf::from(EXERCISES_FOLDER), force)
}

pub fn init_mode_in_path(dir: PathBuf, force: bool) -> anyhow::Result<()> {
    let dir_exists = dir.exists();

    if dir_exists && !force {
        let mut count = 0;
        for file_path in TemplateExercises::iter() {
            let out_path = dir.join(file_path.as_ref());
            if !out_path.exists() {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let file = TemplateExercises::get(&file_path).expect("Failed to read embedded file");
                fs::write(&out_path, file.data)?;
                count += 1;
            }
        }

        let state_path = dir.join(STATE_FILE);
        if !state_path.exists() {
            write_current_index(&state_path, 0)?;
        }

        if count > 0 {
            println!("  {GREEN}✓{RESET} {BOLD}Added {count} new exercise(s) to '{}'!{RESET}", dir.display());
        } else {
            println!("  {YELLOW}⚠  Directory '{}' already exists and all exercises are present.{RESET}", dir.display());
            println!("  {DIM}No new exercises were added.{RESET}");
        }
        return Ok(());
    }

    if dir_exists && force {
        fs::remove_dir_all(&dir)?;
    }

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
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

    if force && dir_exists {
        println!("  {GREEN}✓{RESET} {BOLD}Force-initialized '{}' folder!{RESET}", dir.display());
        println!("  {DIM}Overwrote all {} exercises and reset progress.{RESET}", count);
    } else {
        println!("  {GREEN}✓{RESET} {BOLD}Initialized {} folder!{RESET}", dir.display());
        println!("  {DIM}Extracted {} exercises.{RESET}", count);
        println!("  {DIM}Run {RESET}{BLUE}asmlings start{RESET}{DIM} to begin.{RESET}");
    }

    Ok(())
}
fn resolve_exercises() -> anyhow::Result<(PathBuf, Vec<PathBuf>, PathBuf)> {
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

    Ok((exercises_dir, paths, state_path))
}

pub fn run_workflow() -> anyhow::Result<()> {
    let w = term_width();

    let (exercises_dir, paths, state_path) = resolve_exercises()?;

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
                println!("  {GREEN_BG} PASS {RESET}  {BOLD}All assertions passed.{RESET}");
                write_current_index(&state_path, current + 1)?;

                if current + 1 >= total {
                    println!(
                        "\n  {GREEN_BG} COMPLETE {RESET}  {BOLD}You've finished every \
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

enum WatchEvent {
    File(Result<notify::Event, notify::Error>),
    Input(String),
}

fn get_current_exercise() -> anyhow::Result<Option<Exercise>> {
    let (_, paths, state_path) = resolve_exercises()?;

    if paths.is_empty() {
        return Ok(None);
    }

    let current = read_current_index(&state_path);
    if current >= paths.len() {
        return Ok(None);
    }

    let ex = Exercise::load(paths[current].clone())?;
    Ok(Some(ex))
}

#[cfg(unix)]
fn read_single_char() -> Option<u8> {
    use std::os::unix::io::AsRawFd;
    let fd = std::io::stdin().as_raw_fd();
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        if libc::tcgetattr(fd, &mut termios) != 0 {
            return None;
        }
        let original_termios = termios;

        termios.c_lflag &= !(libc::ICANON | libc::ECHO);
        termios.c_cc[libc::VMIN] = 1;
        termios.c_cc[libc::VTIME] = 0;

        if libc::tcsetattr(fd, libc::TCSADRAIN, &termios) != 0 {
            return None;
        }

        let mut buf = [0u8; 1];
        let bytes_read = libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, 1);

        let _ = libc::tcsetattr(fd, libc::TCSADRAIN, &original_termios);

        if bytes_read == 1 {
            buf.get(0).copied()
        } else {
            None
        }
    }
}

#[cfg(not(unix))]
fn read_single_char() -> Option<u8> {
    None
}

pub fn watch_mode() -> anyhow::Result<()> {
    crate::utils::clear_screen();
    let _ = run_workflow();

    let (tx, rx) = channel();

    // Spawn file watcher
    let tx_watcher = tx.clone();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx_watcher.send(WatchEvent::File(res));
    })?;

    let exercises_dir = [PathBuf::from(EXERCISES_FOLDER), PathBuf::from("exercises")]
        .into_iter()
        .find(|p| p.is_dir())
        .ok_or_else(|| anyhow::anyhow!("Could not find exercises/ directory to watch"))?;

    watcher.watch(&exercises_dir, RecursiveMode::Recursive)?;

    println!(
        "  {DIM}Watching for file changes in {}... (Press Ctrl+C to stop, h for hint){RESET}",
        exercises_dir.display()
    );

    // Spawn stdin reader thread
    let tx_stdin = tx.clone();
    std::thread::spawn(move || {
        loop {
            if let Some(ch) = read_single_char() {
                let input_char = (ch as char).to_lowercase().to_string();
                if tx_stdin.send(WatchEvent::Input(input_char)).is_err() {
                    break;
                }
            } else {
                // Fallback to normal read_line if raw mode is not supported or fails
                let stdin = std::io::stdin();
                let mut line = String::new();
                loop {
                    match stdin.read_line(&mut line) {
                        Ok(0) => break, // EOF reached
                        Ok(_) => {
                            let input = line.trim().to_lowercase();
                            line.clear();
                            if !input.is_empty() {
                                if tx_stdin.send(WatchEvent::Input(input)).is_err() {
                                    break;
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
                break;
            }
        }
    });
    let mut last_run = Instant::now();
    let mut hint_shown = false;

    loop {
        match rx.recv() {
            Ok(WatchEvent::File(Ok(event))) => {
                if matches!(event.kind, EventKind::Modify(_)) {
                    if last_run.elapsed() > Duration::from_millis(200) {
                        crate::utils::clear_screen();
                        if let Err(e) = run_workflow() {
                            println!("  {RED}Fatal error running workflow:{RESET} {}", e);
                        }
                        println!(
                            "  {DIM}Watching for file changes... (Press Ctrl+C to stop, h for hint){RESET}"
                        );
                        last_run = Instant::now();
                        hint_shown = false; // Reset hint state on file modification / exercise change
                    }
                }
            },
            Ok(WatchEvent::File(Err(e))) => println!("  {RED}Watch error:{RESET} {:?}", e),
            Ok(WatchEvent::Input(input)) => {
                if input == "h" || input == "hint" {
                    if !hint_shown {
                        if let Ok(Some(ex)) = get_current_exercise() {
                            if let Some(hint) = crate::hints::get_hint(&ex.name) {
                                println!();
                                println!("  {YELLOW}💡 Hint for {BOLD}{}{RESET}{YELLOW}:{RESET}", ex.name);
                                println!("  {DIM}──────────────────────────────────────────{RESET}");
                                for line in hint.lines() {
                                    println!("  {YELLOW}{line}{RESET}");
                                }
                                println!("  {DIM}──────────────────────────────────────────{RESET}");
                                println!();
                                hint_shown = true;
                            } else {
                                println!("\n  {YELLOW}⚠  No hint available for {}{RESET}\n", ex.name);
                            }
                        } else {
                            println!("\n  {YELLOW}⚠  Could not load current exercise to show hint.{RESET}\n");
                        }
                        println!(
                            "  {DIM}Watching for file changes... (Press Ctrl+C to stop, h for hint){RESET}"
                        );
                    }
                }
            },
            Err(e) => anyhow::bail!("Channel receive error: {:?}", e),
        }
    }
}

pub fn debug_exercise() -> anyhow::Result<()> {
    let (exercises_dir, paths, state_path) = resolve_exercises()?;

    if paths.is_empty() {
        anyhow::bail!("No .asm exercises found in {}", exercises_dir.display());
    }

    let current = read_current_index(&state_path);
    let total = paths.len();

    if current >= total {
        println!("  {GREEN_BG} COMPLETE {RESET}  All exercises already done, nothing to debug.");
        return Ok(());
    }

    let ex = Exercise::load(paths[current].clone())?;
    let out_path = exercises_dir.join(format!("{}.bin", ex.name));

    let assembled = crate::assembler::assemble(&paths[current])?;
    fs::write(&out_path, &assembled.code)?;

    println!("  {GREEN}✓{RESET}  {BOLD}Dumped binary:{RESET} {BLUE}{}{RESET}", out_path.display());
    println!("  {DIM}size     {RESET}{} bytes", assembled.code.len());

    if !assembled.labels.is_empty() {
        println!("  {DIM}labels:{RESET}");
        let mut labels: Vec<_> = assembled.labels.iter().collect();
        labels.sort_by_key(|(_, addr)| *addr);
        for (name, addr) in labels {
            println!("    {BLUE}{name}{RESET}  {DIM}@ 0x{addr:04x}{RESET}");
        }
    }

    println!();
    println!("  {DIM}ndisasm -b 16 -o 0x100 {}{RESET}", out_path.display());
    println!(
        "  {DIM}objdump -b binary -m i8086 -M intel -D --adjust-vma=0x100 {}{RESET}",
        out_path.display()
    );

    Ok(())
}
