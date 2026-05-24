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

pub fn init_mode() -> anyhow::Result<()> {
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

pub fn run_workflow() -> anyhow::Result<()> {
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

pub fn watch_mode() -> anyhow::Result<()> {
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
