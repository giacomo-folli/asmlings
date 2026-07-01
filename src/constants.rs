pub const LOAD_ADDR: u64 = 0x0100;
pub const MEM_BASE: u64 = 0x0000;
pub const MEM_SIZE: u64 = 0x10000;
pub const STATE_FILE: &str = ".asmlings_state";
pub const EXERCISES_FOLDER: &str = "./exercises";

// ── ANSI helpers
// ──────────────────────────────────────────────────────────────
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const GREEN: &str = "\x1b[32m";
pub const RED: &str = "\x1b[31m";
pub const BLUE: &str = "\x1b[34m";
pub const YELLOW: &str = "\x1b[33m";
pub const GREEN_BG: &str = "\x1b[42;30m";
pub const RED_BG: &str = "\x1b[41;30m";

