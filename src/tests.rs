use std::{fs, path::PathBuf};

use tempfile::TempDir;

use super::{
    Assertion,
    Exercise,
    MemAddr,
    name_to_reg,
    parse_labels,
    // integration helpers (need NASM + Unicorn):
    // assemble, run_exercise,
    parse_u64,
    read_current_index,
    write_current_index,
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
    let lst = "\
     1 00000100 B80500            _start: mov ax, 5
     4 0000010A 00                result: db 0
";
    let p = write_map(&dir, lst);
    let labels = parse_labels(&p);
    assert_eq!(labels.get("_start"), Some(&0x100));
    assert_eq!(labels.get("result"), Some(&0x10A));
}

#[test]
fn parse_labels_skips_header_row() {
    let dir = TempDir::new().unwrap();
    // No header in listing format — just verify a normal label parses
    let lst = "     1 00000200 90                myLabel: nop\n";
    let p = write_map(&dir, lst);
    let labels = parse_labels(&p);
    assert_eq!(labels.get("myLabel"), Some(&0x200));
}

#[test]
fn parse_labels_ignores_malformed_lines() {
    let dir = TempDir::new().unwrap();
    let lst = "\
just one token
     1 ZZZZZZZZ 90                badHex: nop
     1 00000100 90                goodLabel: nop
";
    let p = write_map(&dir, lst);
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
fn exercise_load_no_assertions() {
    let dir = TempDir::new().unwrap();
    let p = write_asm(&dir, "empty", "; just a comment\nmov ax, 1\n");
    let ex = Exercise::load(p).unwrap();
    assert!(ex.assertions.is_empty());
    assert!(ex.is_done); // no "I AM NOT DONE" marker
}

#[test]
fn exercise_load_is_done_false_when_marker_present() {
    let dir = TempDir::new().unwrap();
    let src = "; I AM NOT DONE\nmov ax, 1\n";
    let p = write_asm(&dir, "wip", src);
    let ex = Exercise::load(p).unwrap();
    assert!(!ex.is_done);
}

#[test]
fn exercise_load_register_assertion_hex() {
    let dir = TempDir::new().unwrap();
    let src = "; ASSERT_REG: AX == 0x0005\nmov ax, 5\n";
    let p = write_asm(&dir, "reg_hex", src);
    let ex = Exercise::load(p).unwrap();
    assert_eq!(ex.assertions.len(), 1);
    match &ex.assertions[0] {
        Assertion::Register { reg, expected } => {
            assert_eq!(reg, "AX");
            assert_eq!(*expected, 5u16);
        },
        other => panic!("Expected Register assertion, got {:?}", other),
    }
}

#[test]
fn exercise_load_register_assertion_decimal() {
    let dir = TempDir::new().unwrap();
    let src = "; ASSERT_REG: BX == 42\n";
    let p = write_asm(&dir, "reg_dec", src);
    let ex = Exercise::load(p).unwrap();
    assert_eq!(ex.assertions.len(), 1);
    match &ex.assertions[0] {
        Assertion::Register { reg, expected } => {
            assert_eq!(reg, "BX");
            assert_eq!(*expected, 42u16);
        },
        other => panic!("Expected Register assertion, got {:?}", other),
    }
}

#[test]
fn exercise_load_memory_assertion_literal_addr() {
    let dir = TempDir::new().unwrap();
    let src = "; ASSERT_MEM: 0x0200 == 0xFF\n";
    let p = write_asm(&dir, "mem_lit", src);
    let ex = Exercise::load(p).unwrap();
    assert_eq!(ex.assertions.len(), 1);
    match &ex.assertions[0] {
        Assertion::Memory { addr: MemAddr::Literal(addr), expected } => {
            assert_eq!(*addr, 0x200);
            assert_eq!(*expected, 0xFF);
        },
        other => panic!("Expected Memory(Literal) assertion, got {:?}", other),
    }
}

#[test]
fn exercise_load_memory_assertion_label_addr() {
    let dir = TempDir::new().unwrap();
    let src = "; ASSERT_MEM: result == 0x42\n";
    let p = write_asm(&dir, "mem_label", src);
    let ex = Exercise::load(p).unwrap();
    assert_eq!(ex.assertions.len(), 1);
    match &ex.assertions[0] {
        Assertion::Memory { addr: MemAddr::Label(label), expected } => {
            assert_eq!(label, "result");
            assert_eq!(*expected, 0x42);
        },
        other => panic!("Expected Memory(Label) assertion, got {:?}", other),
    }
}

#[test]
fn exercise_load_flag_assertion() {
    let dir = TempDir::new().unwrap();
    let src = "; ASSERT_FLAG: ZF == 1\n";
    let p = write_asm(&dir, "flag", src);
    let ex = Exercise::load(p).unwrap();
    assert_eq!(ex.assertions.len(), 1);
    match &ex.assertions[0] {
        Assertion::Flag { flag, expected } => {
            assert_eq!(flag, "ZF");
            assert!(*expected);
        },
        other => panic!("Expected Flag assertion, got {:?}", other),
    }
}

#[test]
fn exercise_load_flag_assertion_zero() {
    let dir = TempDir::new().unwrap();
    let src = "; ASSERT_FLAG: CF == 0\n";
    let p = write_asm(&dir, "flag_zero", src);
    let ex = Exercise::load(p).unwrap();
    match &ex.assertions[0] {
        Assertion::Flag { flag, expected } => {
            assert_eq!(flag, "CF");
            assert!(!*expected);
        },
        other => panic!("Unexpected {:?}", other),
    }
}

#[test]
fn exercise_load_multiple_assertions() {
    let dir = TempDir::new().unwrap();
    let src = "\
; ASSERT_REG: AX == 0x0001
; ASSERT_REG: BX == 0x0002
; ASSERT_FLAG: ZF == 0
; ASSERT_MEM: 0x0300 == 0xAB
mov ax, 1
";
    let p = write_asm(&dir, "multi", src);
    let ex = Exercise::load(p).unwrap();
    assert_eq!(ex.assertions.len(), 4);
}

#[test]
fn exercise_load_ignores_prose_assert_lines() {
    // These look like assertions but the value doesn't parse — they're docs.
    let dir = TempDir::new().unwrap();
    let src = "\
; ASSERT_REG: AX == some_description_text
; ASSERT_MEM: 0x0100 == not_a_number
";
    let p = write_asm(&dir, "prose", src);
    let ex = Exercise::load(p).unwrap();
    assert!(ex.assertions.is_empty(), "prose-like assert lines must be skipped");
}

#[test]
fn exercise_load_name_derived_from_filename() {
    let dir = TempDir::new().unwrap();
    let p = write_asm(&dir, "01_mov_basics", "");
    let ex = Exercise::load(p).unwrap();
    assert_eq!(ex.name, "01_mov_basics");
}

#[test]
fn exercise_load_registers_uppercased() {
    let dir = TempDir::new().unwrap();
    // The parser uppercases the register name
    let src = "; ASSERT_REG: ax == 0x0001\n";
    let p = write_asm(&dir, "lower_reg", src);
    let ex = Exercise::load(p).unwrap();
    match &ex.assertions[0] {
        Assertion::Register { reg, .. } => assert_eq!(reg, "AX"),
        other => panic!("Unexpected {:?}", other),
    }
}

#[test]
fn exercise_load_flags_uppercased() {
    let dir = TempDir::new().unwrap();
    let src = "; ASSERT_FLAG: zf == 1\n";
    let p = write_asm(&dir, "lower_flag", src);
    let ex = Exercise::load(p).unwrap();
    match &ex.assertions[0] {
        Assertion::Flag { flag, .. } => assert_eq!(flag, "ZF"),
        other => panic!("Unexpected {:?}", other),
    }
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
fn integration_mov_ax_passes() {
    use super::run_exercise;

    let dir = TempDir::new().unwrap();
    let src = "\
BITS 16
ORG 0x0100
; ASSERT_REG: AX == 0x0005
mov ax, 5
";
    let p = write_asm(&dir, "mov_ax", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].passed, "AX should equal 0x0005");
}

#[test]
#[ignore]
fn integration_mov_ax_fails_wrong_value() {
    use super::run_exercise;

    let dir = TempDir::new().unwrap();
    let src = "\
BITS 16
ORG 0x0100
; ASSERT_REG: AX == 0x0099
mov ax, 5
";
    let p = write_asm(&dir, "mov_ax_fail", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    assert!(!results[0].passed, "Should fail: AX=5 != 0x99");
    assert_eq!(results[0].expected_str, "0x0099");
    assert_eq!(results[0].actual_str, "0x0005");
}

#[test]
#[ignore]
fn integration_zero_flag_after_sub() {
    use super::run_exercise;

    let dir = TempDir::new().unwrap();
    let src = "\
BITS 16
ORG 0x0100
; ASSERT_FLAG: ZF == 1
mov ax, 3
sub ax, 3
";
    let p = write_asm(&dir, "zf_sub", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    assert!(results[0].passed, "ZF should be set after sub ax,ax");
}

#[test]
#[ignore]
fn integration_memory_write_literal() {
    use super::run_exercise;

    let dir = TempDir::new().unwrap();
    let src = "\
BITS 16
ORG 0x0100
; ASSERT_MEM: 0x0200 == 0xAB
mov ax, 0xAB
mov [0x0200], al
";
    let p = write_asm(&dir, "mem_lit_int", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    assert!(results[0].passed, "Byte at 0x0200 should be 0xAB");
}

#[test]
#[ignore]
fn integration_memory_write_label() {
    use super::run_exercise;

    let dir = TempDir::new().unwrap();
    let src = "\
BITS 16
ORG 0x0100
; ASSERT_MEM: result == 0x7F
    mov al, 0x7F
    mov [result], al
    hlt
result: db 0
";
    let p = write_asm(&dir, "mem_label_int", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    assert!(results[0].passed, "Byte at label 'result' should be 0x7F");
}

#[test]
#[ignore]
fn integration_multiple_assertions_all_pass() {
    use super::run_exercise;

    let dir = TempDir::new().unwrap();
    let src = "\
BITS 16
ORG 0x0100
; ASSERT_REG: AX == 0x0001
; ASSERT_REG: BX == 0x0002
; ASSERT_FLAG: ZF == 0
mov ax, 1
mov bx, 2
cmp ax, bx   ; sets flags, ZF=0 because 1 != 2
";
    let p = write_asm(&dir, "multi_int", src);
    let ex = Exercise::load(p).unwrap();
    let results = run_exercise(&ex).unwrap();
    assert!(results.iter().all(|r| r.passed), "All assertions should pass");
}

#[test]
#[ignore]
fn integration_nasm_syntax_error_returns_err() {
    use super::assemble;

    let dir = TempDir::new().unwrap();
    let p = write_asm(&dir, "bad_syntax", "BITS 16\nORG 0x0100\nthis is not valid asm\n");
    assert!(assemble(&p).is_err(), "Bad ASM should produce an error from NASM");
}
