use std::collections::HashMap;
use unicorn_engine::{Unicorn, RegisterX86};
use crate::exercise::AssertionResult;

pub struct ProgrammaticSuite {
    pub name: &'static str,
    pub target_label: Option<&'static str>,
    pub cases: Vec<ProgrammaticCase>,
}

pub struct ProgrammaticCase {
    pub name: &'static str,
    pub setup: fn(&mut Unicorn<'_, ()>, &std::collections::HashMap<String, u64>) -> anyhow::Result<()>,
    pub verify: fn(&Unicorn<'_, ()>, &std::collections::HashMap<String, u64>) -> anyhow::Result<Vec<AssertionResult>>,
}

pub fn set_reg(emu: &mut Unicorn<'_, ()>, reg: RegisterX86, val: u16) -> anyhow::Result<()> {
    emu.reg_write(reg, val as u64)
        .map_err(|e| anyhow::anyhow!("Failed to write register: {:?}", e))
}

pub fn check_reg(emu: &Unicorn<'_, ()>, name: &str, reg: RegisterX86, expected: u16) -> AssertionResult {
    let val = emu.reg_read(reg).unwrap_or(0) as u16;
    AssertionResult {
        passed: val == expected,
        name_str: name.to_string(),
        expected_str: format!("0x{:04X}", expected),
        actual_str: format!("0x{:04X}", val),
    }
}

#[allow(dead_code)]
pub fn check_flag(emu: &Unicorn<'_, ()>, name: &str, flag: &str, expected: bool) -> AssertionResult {
    let eflags = emu.reg_read(RegisterX86::EFLAGS).unwrap_or(0) as u32;
    let bit = match flag {
        "CF" => 0,
        "PF" => 2,
        "AF" => 4,
        "ZF" => 6,
        "SF" => 7,
        "OF" => 11,
        _ => panic!("Unknown flag in test verify: {}", flag),
    };
    let val = (eflags & (1 << bit)) != 0;
    AssertionResult {
        passed: val == expected,
        name_str: name.to_string(),
        expected_str: if expected { "1".into() } else { "0".into() },
        actual_str: if val { "1".into() } else { "0".into() },
    }
}

pub fn check_mem(
    emu: &Unicorn<'_, ()>,
    labels: &HashMap<String, u64>,
    label_name: &str,
    expected: u16,
    size: usize,
) -> AssertionResult {
    let resolved = match labels.get(label_name) {
        Some(&addr) => addr,
        None => {
            return AssertionResult {
                passed: false,
                name_str: format!("[{}]", label_name),
                expected_str: format!("0x{:04X}", expected),
                actual_str: format!("Label '{}' not found", label_name),
            };
        }
    };

    let mut buf = vec![0u8; size];
    if emu.mem_read(resolved, &mut buf).is_err() {
        return AssertionResult {
            passed: false,
            name_str: format!("[{}]", label_name),
            expected_str: format!("0x{:04X}", expected),
            actual_str: "Read error".into(),
        };
    }

    let val = if size == 2 {
        u16::from_le_bytes([buf[0], buf[1]])
    } else {
        buf[0] as u16
    };

    AssertionResult {
        passed: val == expected,
        name_str: format!("[{}]", label_name),
        expected_str: if size == 2 { format!("0x{:04X}", expected) } else { format!("0x{:02X}", expected) },
        actual_str: if size == 2 { format!("0x{:04X}", val) } else { format!("0x{:02X}", val) },
    }
}

// Global registry of programmatic test suites
pub fn get_test_suite(name: &str) -> Option<&'static ProgrammaticSuite> {
    let name_enum = name.parse::<crate::exercise_tests::ExerciseName>().ok()?;
    crate::exercise_tests::get_test_suite(name_enum)
}
