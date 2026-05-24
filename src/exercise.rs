use std::{fs, path::PathBuf};

use crate::utils::parse_u64;

#[derive(Debug, Clone)]
pub enum MemAddr {
    Literal(u64),
    Label(String),
}

#[derive(Debug, Clone)]
pub enum Assertion {
    Register { reg: String, expected: u16 },
    Memory { addr: MemAddr, expected: u16, size: usize },
    Flag { flag: String, expected: bool },
}

#[derive(Debug)]
pub struct AssertionResult {
    pub passed:       bool,
    pub name_str:     String,
    pub expected_str: String,
    pub actual_str:   String,
}

#[derive(Debug)]
pub struct Exercise {
    pub path:       PathBuf,
    pub name:       String,
    pub assertions: Vec<Assertion>,
    pub is_done:    bool,
}

impl Exercise {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
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
