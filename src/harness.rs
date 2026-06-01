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
    pub setup: fn(&mut Unicorn<'_, ()>) -> anyhow::Result<()>,
    pub verify: fn(&Unicorn<'_, ()>, &HashMap<String, u64>) -> anyhow::Result<Vec<AssertionResult>>,
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
    static REGISTRY: std::sync::OnceLock<HashMap<String, ProgrammaticSuite>> = std::sync::OnceLock::new();
    
    let map = REGISTRY.get_or_init(|| {
        let mut m = HashMap::new();
        
        // 01_bare_metal
        m.insert(
            "01_bare_metal".to_string(),
            ProgrammaticSuite {
                name: "01_bare_metal",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x1337),
                            ])
                        }
                    }
                ]
            }
        );
        
        // 02_halves
        m.insert(
            "02_halves".to_string(),
            ProgrammaticSuite {
                name: "02_halves",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0xABCD),
                            ])
                        }
                    }
                ]
            }
        );
        
        // 03_addition
        m.insert(
            "03_addition".to_string(),
            ProgrammaticSuite {
                name: "03_addition",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0x0015),
                            ])
                        }
                    }
                ]
            }
        );

        // 04_subtraction
        m.insert(
            "04_subtraction".to_string(),
            ProgrammaticSuite {
                name: "04_subtraction",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check CX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "CX", RegisterX86::CX, 0x0041),
                            ])
                        }
                    }
                ]
            }
        );

        // 05_reg_to_reg
        m.insert(
            "05_reg_to_reg".to_string(),
            ProgrammaticSuite {
                name: "05_reg_to_reg",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX & DX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0xBEEF),
                                check_reg(emu, "DX", RegisterX86::DX, 0xBEEF),
                            ])
                        }
                    }
                ]
            }
        );

        // 06_bitwise_and
        m.insert(
            "06_bitwise_and".to_string(),
            ProgrammaticSuite {
                name: "06_bitwise_and",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x00CD),
                            ])
                        }
                    }
                ]
            }
        );

        // 07_bitwise_or
        m.insert(
            "07_bitwise_or".to_string(),
            ProgrammaticSuite {
                name: "07_bitwise_or",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0x0FF0),
                            ])
                        }
                    }
                ]
            }
        );

        // 08_xor
        m.insert(
            "08_xor".to_string(),
            ProgrammaticSuite {
                name: "08_xor",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check CX & DX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "CX", RegisterX86::CX, 0x0000),
                                check_reg(emu, "DX", RegisterX86::DX, 0x0000),
                            ])
                        }
                    }
                ]
            }
        );

        // 09_shift_left
        m.insert(
            "09_shift_left".to_string(),
            ProgrammaticSuite {
                name: "09_shift_left",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x0030),
                            ])
                        }
                    }
                ]
            }
        );

        // 10_stack
        m.insert(
            "10_stack".to_string(),
            ProgrammaticSuite {
                name: "10_stack",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0xCAFE),
                            ])
                        }
                    }
                ]
            }
        );

        // 11_bitwise_not
        m.insert(
            "11_bitwise_not".to_string(),
            ProgrammaticSuite {
                name: "11_bitwise_not",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0xFF00),
                            ])
                        }
                    }
                ]
            }
        );

        // 12_shift_right
        m.insert(
            "12_shift_right".to_string(),
            ProgrammaticSuite {
                name: "12_shift_right",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0x0010),
                            ])
                        }
                    }
                ]
            }
        );

        // 13_inc_dec
        m.insert(
            "13_inc_dec".to_string(),
            ProgrammaticSuite {
                name: "13_inc_dec",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x000C),
                            ])
                        }
                    }
                ]
            }
        );

        // 14_mul
        m.insert(
            "14_mul".to_string(),
            ProgrammaticSuite {
                name: "14_mul",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX & DX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x001E),
                                check_reg(emu, "DX", RegisterX86::DX, 0x0000),
                            ])
                        }
                    }
                ]
            }
        );

        // 15_div
        m.insert(
            "15_div".to_string(),
            ProgrammaticSuite {
                name: "15_div",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX & DX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x0009),
                                check_reg(emu, "DX", RegisterX86::DX, 0x0009),
                            ])
                        }
                    }
                ]
            }
        );

        // 16_neg
        m.insert(
            "16_neg".to_string(),
            ProgrammaticSuite {
                name: "16_neg",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0xFFFB),
                            ])
                        }
                    }
                ]
            }
        );

        // 17_cmp
        m.insert(
            "17_cmp".to_string(),
            ProgrammaticSuite {
                name: "17_cmp",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0x0001),
                            ])
                        }
                    }
                ]
            }
        );

        // 18_loop
        m.insert(
            "18_loop".to_string(),
            ProgrammaticSuite {
                name: "18_loop",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x000C),
                            ])
                        }
                    }
                ]
            }
        );

        // 19_read_mem
        m.insert(
            "19_read_mem".to_string(),
            ProgrammaticSuite {
                name: "19_read_mem",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0xDEAD),
                            ])
                        }
                    }
                ]
            }
        );

        // 20_write_mem
        m.insert(
            "20_write_mem".to_string(),
            ProgrammaticSuite {
                name: "20_write_mem",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX & Memory result",
                        setup: |_| Ok(()),
                        verify: |emu, labels| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x000F),
                                check_mem(emu, labels, "result", 0x000F, 2),
                            ])
                        }
                    }
                ]
            }
        );

        // 21_reg_ind_address
        m.insert(
            "21_reg_ind_address".to_string(),
            ProgrammaticSuite {
                name: "21_reg_ind_address",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0xF00D),
                            ])
                        }
                    }
                ]
            }
        );

        // 22_source_index
        m.insert(
            "22_source_index".to_string(),
            ProgrammaticSuite {
                name: "22_source_index",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0x0006),
                            ])
                        }
                    }
                ]
            }
        );

        // 23_subroutines
        m.insert(
            "23_subroutines".to_string(),
            ProgrammaticSuite {
                name: "23_subroutines",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x0012),
                            ])
                        }
                    }
                ]
            }
        );

        // 24_xchg
        m.insert(
            "24_xchg".to_string(),
            ProgrammaticSuite {
                name: "24_xchg",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX & BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x2222),
                                check_reg(emu, "BX", RegisterX86::BX, 0x1111),
                            ])
                        }
                    }
                ]
            }
        );

        // 25_carry_flag
        m.insert(
            "25_carry_flag".to_string(),
            ProgrammaticSuite {
                name: "25_carry_flag",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX & DX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x0000),
                                check_reg(emu, "DX", RegisterX86::DX, 0x0002),
                            ])
                        }
                    }
                ]
            }
        );

        // 26_bit_ceck
        m.insert(
            "26_bit_ceck".to_string(),
            ProgrammaticSuite {
                name: "26_bit_ceck",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0x0001),
                            ])
                        }
                    }
                ]
            }
        );

        // 27_pusha_popa
        m.insert(
            "27_pusha_popa".to_string(),
            ProgrammaticSuite {
                name: "27_pusha_popa",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX, BX, CX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x0001),
                                check_reg(emu, "BX", RegisterX86::BX, 0x0002),
                                check_reg(emu, "CX", RegisterX86::CX, 0x0003),
                            ])
                        }
                    }
                ]
            }
        );

        // 28_sign_comp
        m.insert(
            "28_sign_comp".to_string(),
            ProgrammaticSuite {
                name: "28_sign_comp",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0xFFFF),
                            ])
                        }
                    }
                ]
            }
        );

        // 29_rol
        m.insert(
            "29_rol".to_string(),
            ProgrammaticSuite {
                name: "29_rol",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x0003),
                            ])
                        }
                    }
                ]
            }
        );
        
        // 30_test_1 (Absolute value)
        m.insert(
            "30_test_1".to_string(),
            ProgrammaticSuite {
                name: "30_test_1",
                target_label: Some("abs_val"),
                cases: vec![
                    ProgrammaticCase {
                        name: "Neg Input (-10)",
                        setup: |emu| {
                            set_reg(emu, RegisterX86::AX, 0xFFF6)
                        },
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX (|-10|)", RegisterX86::AX, 10),
                            ])
                        }
                    },
                    ProgrammaticCase {
                        name: "Pos Input (7)",
                        setup: |emu| {
                            set_reg(emu, RegisterX86::AX, 7)
                        },
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX (|7|)", RegisterX86::AX, 7),
                            ])
                        }
                    },
                    ProgrammaticCase {
                        name: "Zero Input (0)",
                        setup: |emu| {
                            set_reg(emu, RegisterX86::AX, 0)
                        },
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX (|0|)", RegisterX86::AX, 0),
                            ])
                        }
                    },
                    ProgrammaticCase {
                        name: "Neg Input (-1)",
                        setup: |emu| {
                            set_reg(emu, RegisterX86::AX, 0xFFFF)
                        },
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX (|-1|)", RegisterX86::AX, 1),
                            ])
                        }
                    },
                ]
            }
        );

        // 31_test_2 (Find Maximum)
        m.insert(
            "31_test_2".to_string(),
            ProgrammaticSuite {
                name: "31_test_2",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check AX (max)",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "AX", RegisterX86::AX, 0x003E),
                            ])
                        }
                    }
                ]
            }
        );

        // 32_test_3 (Count Set Bits / Popcount)
        m.insert(
            "32_test_3".to_string(),
            ProgrammaticSuite {
                name: "32_test_3",
                target_label: None,
                cases: vec![
                    ProgrammaticCase {
                        name: "Check BX (popcount)",
                        setup: |_| Ok(()),
                        verify: |emu, _| {
                            Ok(vec![
                                check_reg(emu, "BX", RegisterX86::BX, 0x0009),
                            ])
                        }
                    }
                ]
            }
        );
        
        m
    });
    
    map.get(name)
}
