use std::{fs, path::PathBuf};

use tempfile::TempDir;

// Import from the specific modules created during the refactor
use crate::exercise::Exercise;
use crate::{
    assembler::{assemble, parse_labels},
    emulator::{name_to_reg, run_exercise},
    state::{read_current_index, write_current_index},
    utils::parse_u64,
};

#[test]
fn parse_u64_decimal() {
    assert_eq!(parse_u64("0"), Some(0));
    assert_eq!(parse_u64("255"), Some(255));
    assert_eq!(parse_u64("65535"), Some(65535));
}

#[test]
fn parse_u64_hex_lowercase_prefix() {
    assert_eq!(parse_u64("0xff"), Some(255));
    assert_eq!(parse_u64("0x0100"), Some(256));
    assert_eq!(parse_u64("0xffff"), Some(65535));
}

#[test]
fn parse_u64_hex_uppercase_prefix() {
    assert_eq!(parse_u64("0XFF"), Some(255));
    assert_eq!(parse_u64("0X1A2B"), Some(0x1A2B));
}

#[test]
fn parse_u64_trims_whitespace() {
    assert_eq!(parse_u64("  42  "), Some(42));
    assert_eq!(parse_u64(" 0xFF "), Some(255));
}

#[test]
fn parse_u64_invalid_returns_none() {
    assert_eq!(parse_u64(""), None);
    assert_eq!(parse_u64("abc"), None);
    assert_eq!(parse_u64("0xGG"), None);
    assert_eq!(parse_u64("12.5"), None);
    assert_eq!(parse_u64("-1"), None);
}

// read_current_index / write_current_index
fn tmp_state(dir: &TempDir) -> PathBuf {
    dir.path().join(".asmlings_state")
}

#[test]
fn state_roundtrip() {
    let dir = TempDir::new().unwrap();
    let p = tmp_state(&dir);
    write_current_index(&p, 7).unwrap();
    assert_eq!(read_current_index(&p), 7);
}

#[test]
fn state_missing_file_returns_zero() {
    let dir = TempDir::new().unwrap();
    let p = tmp_state(&dir);
    // file does not exist
    assert_eq!(read_current_index(&p), 0);
}

#[test]
fn state_corrupted_file_returns_zero() {
    let dir = TempDir::new().unwrap();
    let p = tmp_state(&dir);
    fs::write(&p, "not-a-number").unwrap();
    assert_eq!(read_current_index(&p), 0);
}

#[test]
fn state_write_overwrites_previous() {
    let dir = TempDir::new().unwrap();
    let p = tmp_state(&dir);
    write_current_index(&p, 3).unwrap();
    write_current_index(&p, 9).unwrap();
    assert_eq!(read_current_index(&p), 9);
}

#[test]
fn state_zero_roundtrip() {
    let dir = TempDir::new().unwrap();
    let p = tmp_state(&dir);
    write_current_index(&p, 0).unwrap();
    assert_eq!(read_current_index(&p), 0);
}

// name_to_reg
#[test]
fn name_to_reg_known_registers() {
    for name in &[
        "AX", "BX", "CX", "DX", "AH", "AL", "BH", "BL", "CH", "CL", "DH", "DL", "SP", "BP", "SI",
        "DI",
    ] {
        assert!(name_to_reg(name).is_ok(), "Expected Ok for register {name}");
    }
}

#[test]
fn name_to_reg_unknown_register_errors() {
    assert!(name_to_reg("XX").is_err());
    assert!(name_to_reg("").is_err());
    assert!(name_to_reg("EAX").is_err()); // 32-bit, not supported
    assert!(name_to_reg("ax").is_err()); // lowercase not handled
}

// parse_labels  (map-file parser)
fn write_map(dir: &TempDir, content: &str) -> PathBuf {
    let p = dir.path().join("test.map");
    fs::write(&p, content).unwrap();
    p
}

#[test]
fn parse_labels_empty_file() {
    let dir = TempDir::new().unwrap();
    let p = write_map(&dir, "");
    let labels = parse_labels(&p);
    assert!(labels.is_empty());
}

#[test]
fn parse_labels_typical_map() {
    let dir = TempDir::new().unwrap();
    let map_content = "\
- NASM Map file ---------------------------------------------------------------

Source file:  test.asm
Output file:  test.bin

-- Symbols --------------------------------------------------------------------

---- Section .text ------------------------------------------------------------

Real              Virtual           Name
             100               100  _start
             10A               10A  result
";
    let p = write_map(&dir, map_content);
    let labels = parse_labels(&p);
    assert_eq!(labels.get("_start"), Some(&0x100));
    assert_eq!(labels.get("result"), Some(&0x10A));
}

#[test]
fn parse_labels_skips_header_row() {
    let dir = TempDir::new().unwrap();
    let map_content = "\
Real              Virtual           Name
             200               200  myLabel
";
    let p = write_map(&dir, map_content);
    let labels = parse_labels(&p);
    assert_eq!(labels.get("myLabel"), Some(&0x200));
}

#[test]
fn parse_labels_ignores_malformed_lines() {
    let dir = TempDir::new().unwrap();
    let map_content = "\
just one token
ZZZZZZZZ ZZZZZZZZ badHex
             100               100  goodLabel
";
    let p = write_map(&dir, map_content);
    let labels = parse_labels(&p);
    assert_eq!(labels.get("goodLabel"), Some(&0x100));
    assert!(!labels.contains_key("badHex"));
}

// Exercise::load  –  assertion parsing from .asm source

fn write_asm(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let p = dir.path().join(format!("{name}.asm"));
    fs::write(&p, content).unwrap();
    p
}

#[test]
fn exercise_load_name_derived_from_filename() {
    let dir = TempDir::new().unwrap();
    let p = write_asm(&dir, "01_mov_basics", "");
    let ex = Exercise::load(p).unwrap();
    assert_eq!(ex.name, "01_mov_basics");
}

// Integration tests  (require NASM + Unicorn — skipped by default)
// Run with: cargo test -- --include-ignored

// /// Write a minimal 16-bit flat-binary .asm file and return its path.
// fn write_integration_asm(dir: &TempDir, name: &str, body: &str) -> PathBuf {
//     let content = format!("BITS 16\nORG 0x0100\n{body}\n");
//     write_asm(dir, name, &content)
// }



#[test]
#[ignore]
fn integration_nasm_syntax_error_returns_err() {
    let dir = TempDir::new().unwrap();
    let p = write_asm(&dir, "bad_syntax", "BITS 16\nORG 0x0100\nthis is not valid asm\n");
    assert!(assemble(&p).is_err(), "Bad ASM should produce an error from NASM");
}

// ── NEW COMPREHENSIVE TESTS ──

// 1. Parser Edge Case Tests (Unit Tests)

// 2. Emulator Runtime and Error Tests (Integration Tests - #[ignore])



// 3. State and Utility Tests (Unit Tests)

#[test]
fn parse_u64_overflow() {
    assert_eq!(parse_u64("18446744073709551616"), None);
    assert_eq!(parse_u64("0x10000000000000000"), None);
}

#[test]
fn parse_u64_invalid_prefixes() {
    assert_eq!(parse_u64("0x"), None);
    assert_eq!(parse_u64("0X"), None);
    assert_eq!(parse_u64("0xG"), None);
    assert_eq!(parse_u64("0x12G"), None);
}

#[test]
fn parse_labels_duplicate_labels() {
    let dir = TempDir::new().unwrap();
    let map_content = "\
Real              Virtual           Name
             100               100  myLabel
             200               200  myLabel
";
    let p = write_map(&dir, map_content);
    let labels = parse_labels(&p);
    assert_eq!(labels.get("myLabel"), Some(&0x200));
}

#[test]
fn parse_labels_extra_columns() {
    let dir = TempDir::new().unwrap();
    let map_content = "\
Real              Virtual           Name            Extra
             100               100  myLabel         something
";
    let p = write_map(&dir, map_content);
    let labels = parse_labels(&p);
    assert!(!labels.contains_key("myLabel"), "Rows with extra columns must be ignored");
}

// 4. Programmatic Harness Tests
#[test]
fn harness_get_test_suite_known_exercises() {
    let suite_01 = crate::harness::get_test_suite("01_bare_metal");
    assert!(suite_01.is_some());
    let s = suite_01.unwrap();
    assert_eq!(s.name, "01_bare_metal");
    assert!(s.target_label.is_none());
    assert_eq!(s.cases.len(), 1);

    let suite_32 = crate::harness::get_test_suite("32_abs_val");
    assert!(suite_32.is_some());
    let s = suite_32.unwrap();
    assert_eq!(s.name, "32_abs_val");
    assert_eq!(s.target_label, Some("abs_val"));
    assert_eq!(s.cases.len(), 4);
}

#[test]
fn harness_get_test_suite_unknown_exercise() {
    assert!(crate::harness::get_test_suite("unknown_exercise_name").is_none());
}

#[test]
#[ignore]
fn integration_programmatic_01_bare_metal_passes() {
    let dir = TempDir::new().unwrap();
    let src = "\
BITS 16
ORG 0x0100
global _start
_start:
    mov ax, 0x1337
";
    // We name the file "01_bare_metal" to trigger the programmatic suite
    let p = write_asm(&dir, "01_bare_metal", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].passed, "AX should equal 0x1337 under programmatic check");
    assert!(results[0].name_str.contains("Check AX"));
}

#[test]
#[ignore]
fn integration_programmatic_32_abs_val_fails_cheat() {
    let dir = TempDir::new().unwrap();
    // Cheat implementation: just load 10 into AX, ignoring the input
    let src = "\
BITS 16
ORG 0x0100
global _start
_start:
    call abs_val
    hlt

abs_val:
    mov ax, 10
    ret
";
    let p = write_asm(&dir, "32_abs_val", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    // There are 4 test cases (Neg 10, Pos 7, Zero 0, Neg 1)
    // Positive 7 input should fail because the cheat always returns 10!
    assert_eq!(results.len(), 4);
    
    let pos_7_passed = results.iter()
        .find(|r| r.name_str.contains("Pos Input (7)"))
        .map(|r| r.passed)
        .unwrap_or(false);
        
    assert!(!pos_7_passed, "Cheat implementation should fail for positive input 7");
}

#[test]
#[ignore]
fn integration_programmatic_32_abs_val_passes_correct() {
    let dir = TempDir::new().unwrap();
    // Correct absolute value implementation using jump/neg
    let src = "\
BITS 16
ORG 0x0100
global _start
_start:
    call abs_val
    hlt

abs_val:
    cmp ax, 0
    jge .done
    neg ax
.done:
    ret
";
    let p = write_asm(&dir, "32_abs_val", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    assert_eq!(results.len(), 4);
    assert!(results.iter().all(|r| r.passed), "Correct absolute value implementation must pass all programmatic tests");
}

#[test]
fn test_init_creates_new_dir() {
    let dir = TempDir::new().unwrap();
    let exercises_path = dir.path().join("exercises");
    
    // Ensure directory does not exist initially
    assert!(!exercises_path.exists());
    
    crate::commands::init_mode_in_path(exercises_path.clone(), false).unwrap();
    
    // Ensure directory was created
    assert!(exercises_path.exists());
    
    // Ensure files were extracted
    let mut files: Vec<_> = fs::read_dir(&exercises_path).unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .collect();
    files.sort();
    
    assert!(files.len() > 0);
    assert!(files.contains(&std::ffi::OsString::from("01_bare_metal.asm")));
    
    // Ensure state file was created and contains 0
    let state_file = exercises_path.join(crate::constants::STATE_FILE);
    assert!(state_file.exists());
    assert_eq!(read_current_index(&state_file), 0);
}

#[test]
fn test_init_updates_existing_dir() {
    let dir = TempDir::new().unwrap();
    let exercises_path = dir.path().join("exercises");
    
    // 1. Initial creation
    crate::commands::init_mode_in_path(exercises_path.clone(), false).unwrap();
    
    // Write something to state to check it isn't reset
    let state_file = exercises_path.join(crate::constants::STATE_FILE);
    write_current_index(&state_file, 5).unwrap();
    
    // Modify an existing file's contents
    let bare_metal_path = exercises_path.join("01_bare_metal.asm");
    fs::write(&bare_metal_path, "modified content").unwrap();
    
    // Delete one file to see if it gets restored
    let halves_path = exercises_path.join("02_halves.asm");
    fs::remove_file(&halves_path).unwrap();
    assert!(!halves_path.exists());
    
    // 2. Re-run init
    crate::commands::init_mode_in_path(exercises_path.clone(), false).unwrap();
    
    // Verify state file was NOT overwritten/reset
    assert_eq!(read_current_index(&state_file), 5);
    
    // Verify modified file was NOT overwritten
    let content = fs::read_to_string(&bare_metal_path).unwrap();
    assert_eq!(content, "modified content");
    
    // Verify deleted file was restored
    assert!(halves_path.exists());
    let halves_content = fs::read_to_string(&halves_path).unwrap();
    assert_ne!(halves_content, "modified content");
    assert!(halves_content.contains("I AM NOT DONE") || halves_content.len() > 10);
}

#[test]
fn test_init_force_overwrites() {
    let dir = TempDir::new().unwrap();
    let exercises_path = dir.path().join("exercises");
    
    // 1. Initial creation
    crate::commands::init_mode_in_path(exercises_path.clone(), false).unwrap();
    
    // Write something to state to check it gets reset
    let state_file = exercises_path.join(crate::constants::STATE_FILE);
    write_current_index(&state_file, 5).unwrap();
    
    // Modify an existing file's contents
    let bare_metal_path = exercises_path.join("01_bare_metal.asm");
    fs::write(&bare_metal_path, "modified content").unwrap();
    
    // 2. Re-run init with force = true
    crate::commands::init_mode_in_path(exercises_path.clone(), true).unwrap();
    
    // Verify state file WAS reset to 0
    assert_eq!(read_current_index(&state_file), 0);
    
    // Verify modified file WAS overwritten back to template
    let content = fs::read_to_string(&bare_metal_path).unwrap();
    assert_ne!(content, "modified content");
    assert!(content.contains("I AM NOT DONE"));
}


