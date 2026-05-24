use std::{fs, path::Path};

pub fn read_current_index(state_path: &Path) -> usize {
    fs::read_to_string(state_path).ok().and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0)
}

pub fn write_current_index(state_path: &Path, index: usize) -> anyhow::Result<()> {
    fs::write(state_path, index.to_string())?;
    Ok(())
}
