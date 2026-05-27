# Contributing to Asmlings

Thank you for your interest in contributing to **Asmlings**! This guide will help you set up your local development environment and troubleshoot common issues when compiling the project.

Asmlings is written in Rust and relies on the [Unicorn Emulator](https://www.unicorn-engine.org/) (`unicorn-engine` crate) to emulate the Intel 8086 instruction set. Since the emulator includes C/C++ source code, building the project requires a C/C++ compiler toolchain, CMake, and LLVM/Clang (for generating Rust bindings via `bindgen`).

---

## 🛠️ Development Prerequisites & Setup

### 🪟 Windows Setup

1. **Rust**: Install via [rustup.rs](https://rustup.rs/).
2. **C++ Build Tools**: Install [Visual Studio Community](https://visualstudio.microsoft.com/downloads/) or **Build Tools for Visual Studio**, ensuring the **Desktop development with C++** workload is selected.
3. **LLVM (for Clang/libclang)**:
   You need LLVM installed so `bindgen` can locate `libclang.dll`.
   * **Via Winget (Recommended)**: Run the following in PowerShell:
     ```powershell
     winget install LLVM.LLVM
     ```
   * **Manual**: Download the installer from the [LLVM Releases Page](https://github.com/llvm/llvm-project/releases).
4. **Environment Variables**:
   Set `LIBCLANG_PATH` to the `bin` directory of your LLVM installation so the compiler can find it:
   * **PowerShell (Temporary/Current Session)**:
     ```powershell
     $env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
     ```
   * **System-wide (Permanent)**:
     Add a new user/system environment variable named `LIBCLANG_PATH` with the value `C:\Program Files\LLVM\bin` (or your custom install path).
5. **Enable Developer Mode**:
   Windows restricts non-administrator accounts from creating symbolic links. The `unicorn-engine-sys` build script uses CMake, which creates a symbolic link at the end of compilation. To allow this:
   * Open Windows **Settings** (press `Win + I`).
   * Navigate to **System** $\rightarrow$ **For developers** (or **Privacy & security** $\rightarrow$ **For developers**).
   * Toggle **Developer Mode** to **On**.
   * *Alternative*: Run your build terminal as **Administrator**.

---

### 🍎 macOS Setup

1. **Rust**: Install via [rustup.rs](https://rustup.rs/).
2. **Command Line Tools**:
   ```bash
   xcode-select --install
   ```
3. **CMake & LLVM**:
   Install via [Homebrew](https://brew.sh/):
   ```bash
   brew install cmake llvm
   ```
4. **Environment Variables**:
   Set `LIBCLANG_PATH` so `bindgen` uses Homebrew's LLVM instead of the default system compiler:
   * **Shell (zsh/bash)**:
     ```bash
     export LIBCLANG_PATH="$(brew --prefix llvm)/lib"
     ```
     Add this line to your `~/.zshrc` or `~/.bashrc` to make it permanent.

---

### 🐧 Linux Setup (Ubuntu/Debian)

1. **Rust**: Install via [rustup.rs](https://rustup.rs/).
2. **Build Essentials, CMake, & LLVM/Clang**:
   ```bash
   sudo apt update
   sudo apt install build-essential cmake clang libclang-dev
   ```
3. **Environment Variables**:
   On most Linux distributions, `libclang` is placed in standard search paths. If you encounter errors, point to your system's `libclang` directory manually:
   ```bash
   export LIBCLANG_PATH=/usr/lib/llvm-<version>/lib
   ```

---

## ⚡ Speeding Up Builds (Compile x86 Only)

By default, the `unicorn-engine` crate builds emulation support for **all** CPU architectures (ARM, MIPS, RISC-V, SPARC, etc.). This makes the initial build extremely slow and increases binary size.

If you are only working on the Intel 8086 emulation, you can compile **only** the `x86` engine. 

In [Cargo.toml](Cargo.toml), configure the `unicorn-engine` dependency with `default-features = false` and enable `arch_x86`:

```toml
[dependencies]
unicorn-engine = { version = "2.1.5", default-features = false, features = ["arch_x86"] }
```

---

## 🔍 Troubleshooting Common Issues

### ❌ `Unable to find libclang: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll']"`
* **Cause**: `bindgen` cannot locate your LLVM installation.
* **Fix**: Ensure LLVM is installed and the `LIBCLANG_PATH` environment variable is correctly set to the directory containing `libclang.dll` (Windows) or `libclang.so`/`libclang.dylib` (Linux/macOS).

### ❌ `CMake Error: failed to create symbolic link '...': Il privilegio richiesto non appartiene al client.`
* **Cause**: Windows requires elevated privileges to create symlinks by default.
* **Fix**: Turn on **Developer Mode** in your Windows Settings, or run your terminal as an Administrator.

### ❌ `CMake Error: CMake was unable to find a build program corresponding to "Ninja"`
* **Cause**: The build system requires `ninja` to compile CMake dependencies, but it's not in your system's `PATH`.
* **Fix**:
  * **Windows**: If you have Visual Studio installed, Ninja is included. Add it to your current session `PATH`:
    ```powershell
    $env:PATH = "C:\Program Files\Microsoft Visual Studio\18\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja;" + $env:PATH
    ```
  * **macOS**: `brew install ninja`
  * **Linux**: `sudo apt install ninja-build`
