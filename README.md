# ASMLings Project

A lightweight educational sandbox for Intel 8086 assembly programming with a Rust-based emulator runner.

## Repository Overview

This codebase is composed of two main parts:

- `exercises/`
  - Contains NASM assembly exercise files (`.asm`) that demonstrate basic 8086 instructions and concepts.
  - Each exercise file includes a `_start` entry point and one or more `ASSERT_REG:` comments that describe expected register values after execution.
- `src/`
  - Contains a Rust binary that assembles a chosen exercise, executes it in a Unicorn Engine 16-bit x86 emulator, and verifies the expected register results.

## Exercise Format

Each exercise in `exercises/` is a self-contained `.asm` source file.

- Uses `global _start` and `section .text`.
- Includes standard 8086 instructions such as `mov`, `add`, `sub`, `and`, `or`, `xor`, `shl`, `push`, and `pop`.
- Contains assertions in comments using the pattern:

```asm
; ASSERT_REG: AX == 0x1337
```

The runner parses these directives and compares them against the registers after emulation.

## Runner Architecture

The Rust runner in `src/main.rs` performs the following steps:

1. Locate the `exercises/` directory.
2. Read all `.asm` files and sort them alphabetically.
3. Track progress with a small state file named `.asmlings_state` placed in `exercises/`.
4. Load the current exercise and parse `ASSERT_REG:` lines into expected register values.
5. Assemble the exercise using `nasm -f bin` into a temporary binary.
6. Create a Unicorn Engine emulator instance configured for 16-bit x86 mode.
7. Map 64 KB of memory and load the assembled machine code at address `0x0100`.
8. Initialize the stack pointer to `0xFFF0`.
9. Execute the machine code from the loaded entry address to the end of the binary.
10. Read the requested register values from Unicorn and compare them to the asserted expectations.

If all assertions pass, the runner increments the current exercise index and reports success.
If any assertion fails, the current exercise is left unchanged so the user can continue working on it.

## Important Files

- `Cargo.toml`
  - Rust project manifest.
  - Depends on `unicorn-engine` and `anyhow`.
- `src/main.rs`
  - Main runner implementation.
  - Defines assembly loading, assertion parsing, emulator setup, and exercise progression.
- `exercises/`
  - Collection of 8086 assembly exercises.
  - Each file includes comments and assertions for automated verification.

## Requirements

- `cargo` and Rust toolchain
- `nasm` assembler
- `libunicorn` available for the `unicorn-engine` Rust dependency

## Usage

From the repository root:

```bash
cargo run
```

The runner will load the next exercise, execute it, and print whether each asserted register value matches the expected result.

## Extending the Codebase

To add a new exercise:

1. Create a new file in `exercises/` with a `.asm` extension.
2. Add the assembly code under `global _start` and `section .text`.
3. Include one or more `; ASSERT_REG:` directives for the register values to verify.

The runner will automatically discover the new exercise the next time it runs.

## Purpose

This repository is intended as a learning platform for 8086 assembly fundamentals and for experimenting with emulator-driven verification. It is small by design, making it easy to understand, extend, and use as a teaching aid.
