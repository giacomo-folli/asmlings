use std::path::PathBuf;

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
}

impl Exercise {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();

        Ok(Exercise { path, name })
    }
}
