use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct AssertionResult {
    pub passed:       bool,
    pub name_str:     String,
    pub expected_str: String,
    pub actual_str:   String,
}

#[derive(Debug)]
pub struct Exercise {
    pub path:    PathBuf,
    pub name:    String,
    pub is_done: bool,
}

impl Exercise {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let src = fs::read_to_string(&path)?;
        let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

        let mut is_done = true;

        for line in src.lines() {
            let line = line.trim();

            if line == "; I AM NOT DONE" {
                is_done = false;
                break;
            }
        }

        Ok(Exercise { path, name, is_done })
    }
}
