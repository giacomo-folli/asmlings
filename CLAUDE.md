# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

ASMLings is a Rust CLI that gives an interactive, `rustlings`-style feedback loop for learning Intel 8086 (16-bit x86) assembly. Users edit `.asm` files; on save the tool assembles them with NASM, runs them in a Unicorn emulator sandbox, and checks register/memory/flag state.

## Commands

- Build: `cargo build` (first build is slow — see Build environment)
- Run a subcommand during dev: `cargo run -- <init|start|run|debug>`
- Test: `cargo test`
- Single test: `cargo test <name>` (unit tests live in `src/tests.rs`, gated by `#[cfg(test)]`)

CLI subcommands (`src/main.rs`): `init [--force]` extracts exercises, `start` runs watch mode, `run` runs the current exercise once, `debug` dumps the assembled `.bin` + label addresses for `ndisasm`/`objdump`.

## Build environment

- **Runtime:** NASM must be installed and on PATH — assembly is shelled out to `nasm -f bin`.
- **Compile:** `unicorn-engine` bundles C/C++, so building needs a C/C++ toolchain, CMake, and LLVM/Clang (bindgen needs `libclang`; set `LIBCLANG_PATH` if not found). See CONTRIBUTING.md for per-OS setup.
- `Cargo.toml` already pins `unicorn-engine` to `default-features = false, features = ["arch_x86"]` — do not re-enable other arches, it makes builds far slower and the binary huge.
- `.cargo/config.toml` links `-latomic` on Linux and sets CMake parallelism.

## Architecture

The verification pipeline, per exercise, is: **parse → assemble → emulate → assert**.

- `src/exercise.rs` — `Exercise::load` parses an `.asm` file for the `; I AM NOT DONE` sentinel into `is_done`. An exercise only advances when programmatic assertions pass and the sentinel is removed.
- `src/assembler.rs` — `assemble()` shells to NASM. It injects `org 0x0100` + a `[map symbols]` directive and comments out any user `section`/`segment`/`org` lines, then parses the generated `.map` to resolve label → address. All temp files (`.temp.asm`, `.bin`, `.map`) are cleaned up.
- `src/emulator.rs` — `run_exercise` is the entry point. Sets up Unicorn (`Arch::X86`, `Mode::MODE_16`), maps 64KB (`constants.rs`: `MEM_BASE`/`MEM_SIZE`), loads code at `LOAD_ADDR` (0x0100), `SP=0xFFF0`, runs with a 10k-instruction cap. It executes the programmatic test suite (`run_programmatic_suite`) registered for the exercise name. It runs each test case with its own fresh emulator + `setup` callback. If the suite has a `target_label`, it calls that label as a subroutine (pushes a `0x0000` sentinel return address and stops when `ret` returns to it) — this lets exercises be tested as callable functions with varied inputs.
- `src/harness.rs` — `ProgrammaticSuite` / `ProgrammaticCase` types and the `check_reg` / `check_mem` / `check_flag` assertion helpers. `get_test_suite(name)` bridges a string name to a registered suite.
- `src/exercise_tests.rs` — where advanced/multi-case exercise tests are registered, via the `define_exercises!` macro. Each entry declares an `ExerciseName` enum variant, its string name (must equal the `.asm` file stem), an optional `target_label`, and its `cases`.
- `src/hints.rs` — `get_hint(name)` returns the hint shown when the user presses `h` in watch mode.
- `src/state.rs` — reads/writes the current exercise index to `.asmlings_state` inside the exercises dir.
- `src/commands.rs` — `init`, `run_workflow` (single pass: load current exercise, run, print, advance on pass), and `watch_mode` (notify file watcher + a raw-mode stdin thread for the `h` hint key, 200ms debounce).

## Exercises

`template_exercises/` is the source of truth: it is embedded into the binary via `rust-embed` and extracted by `init`. The `exercises/` dir at the repo root is a generated local workspace and is **gitignored** — edit exercises in `template_exercises/`, not there.

To add an exercise:
1. Add `NN_name.asm` to `template_exercises/` with `; I AM NOT DONE`.
2. Register a programmatic suite in `src/exercise_tests.rs` (string name must match the file stem).
3. Optionally add a hint in `src/hints.rs` keyed by the same name.
