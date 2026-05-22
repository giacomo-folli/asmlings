<div align="center">

# ⚙️ ASMLings

**A lightweight, interactive educational sandbox for Intel 8086 assembly programming.**

[![Crates.io](https://img.shields.io/crates/v/asmlings.svg)](https://crates.io/crates/asmlings)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

</div>

ASMLings provides a sandboxed feedback loop powered by a Rust-based 16-bit x86 emulator. You just write code, save the file and instantly check the results.

> Inspired by the `rustlings` project


## Installation

To install Asmlings, you need Rust and Cargo installed on your system. 

> Note: Installation will take some time to complete the `unicorn-engine-sys` build step.

```bash
cargo install asmlings

```

**System Dependency:** You must also have the **NASM** assembler installed, as Asmlings uses it under the hood to compile your code before emulation.

* **macOS:** `brew install nasm`
* **Ubuntu/Debian:** `sudo apt install nasm`
* **Arch Linux:** `sudo pacman -S nasm`
* **Windows:** `winget install NASM`

### Troubleshooting Installation (Linux)

If `cargo install asmlings` fails with a linker error mentioning `__atomic_compare_exchange_16` or `undefined symbol`, your system needs the `libatomic` library to compile the underlying CPU emulator.

To fix this, run the installation with the atomic linker flag:

```bash
RUSTFLAGS="-Clink-arg=-latomic" cargo install asmlings

```

*If the compiler says `-latomic` cannot be found, install it via your package manager:*

* **Ubuntu/Debian:** `sudo apt install libatomic1`
* **Arch Linux:** `sudo pacman -S gcc-libs`
* **Fedora/RHEL:** `sudo dnf install libatomic`

---

## Quick Start

Getting started is as simple as running two commands. Navigate to a folder where you want to store your coursework, and run:

### 1. Initialize the Workspace

Extracts the exercise files and sets up your progress tracker.

```bash
asmlings init

```

### 2. Start Watch Mode

This launches a persistent watch loop. Leave this running in your terminal!

```bash
asmlings start

```

### 3. Solve the Exercises

Open the newly created `exercises/` folder in your favorite text editor. Asmlings will tell you which file to look at. Follow the instructions in the comments, fix the assembly code, and hit save.

Every time you save, Asmlings will automatically re-assemble and verify your code, providing instant terminal feedback.

*(Note: If you just want to run the current exercise once without watching for file changes, you can use `asmlings run`).*

## How to use
Each exercise in the `exercises/` directory is a self-contained `.asm` file demonstrating standard 8086 instructions (e.g., `mov`, `add`, `push`, `pop`, `lodsb`).

To complete an exercise, you must do two things:

1. **Satisfy the Assertions:** The exercise contains commented directives like `; ASSERT_REG: AX == 0x1337` or `; ASSERT_MEM: [0x0200] == 0x42`. Your code must result in the exact register, flag, and memory state requested.
2. **Remove the Sentinel:** Every file contains an `; I AM NOT DONE` comment. Even if your code compiles and passes the assertions, Asmlings will not advance to the next exercise until you manually delete this line. This ensures you deliberately complete the exercise and understand the solution.

## Contributing

Contributions are welcome!
