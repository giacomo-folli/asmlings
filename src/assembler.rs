use std::{collections::HashMap, fs, path::Path, process::Command};

use crate::constants::{BOLD, DIM, GREEN, RESET, YELLOW};

pub struct AssembleOutput {
    pub code:   Vec<u8>,
    pub labels: HashMap<String, u64>,
}

pub fn assemble(asm_path: &Path) -> anyhow::Result<AssembleOutput> {
    let src = fs::read_to_string(asm_path)?;

    let map_path = asm_path.with_extension("map");
    let temp_asm_path = asm_path.with_extension("temp.asm");
    let out_path = asm_path.with_extension("bin");

    let mut modified_src = format!("[map symbols {}]\norg 0x0100\n", map_path.to_str().unwrap());
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

    fs::write(&temp_asm_path, modified_src)?;

    let output_res = Command::new("nasm")
        .args([
            "-f",
            "bin",
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
    let labels = parse_labels(&map_path);

    let _ = fs::remove_file(&out_path);
    let _ = fs::remove_file(&map_path);

    Ok(AssembleOutput { code, labels })
}

pub fn parse_labels(map_path: &Path) -> HashMap<String, u64> {
    let mut map = HashMap::new();
    let Ok(text) = fs::read_to_string(map_path) else { return map };

    for line in text.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 3 {
            if tokens[0] != "Real" {
                if let Ok(addr) = u64::from_str_radix(tokens[0], 16) {
                    let label = tokens[2].trim_start_matches('%');
                    if !label.is_empty() && label.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        map.insert(label.to_string(), addr);
                    }
                }
            }
        }
    }

    map
}
