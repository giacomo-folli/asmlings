use std::{str::FromStr, sync::OnceLock};

use unicorn_engine::RegisterX86;

use crate::harness::{ProgrammaticCase, ProgrammaticSuite, check_mem, check_reg, set_reg};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExerciseName {
    Ex01,
    Ex02,
    Ex03,
    Ex04,
    Ex05,
    Ex06,
    Ex07,
    Ex08,
    Ex09,
    Ex10,
    Ex11,
    Ex12,
    Ex13,
    Ex14,
    Ex15,
    Ex16,
    Ex17,
    Ex18,
    Ex19,
    Ex20,
    Ex21,
    Ex22,
    Ex23,
    Ex24,
    Ex25,
    Ex26,
    Ex27,
    Ex28,
    Ex29,
    Ex30,
    Ex31,
    Ex32,
}

impl FromStr for ExerciseName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "01_bare_metal" => Ok(Self::Ex01),
            "02_halves" => Ok(Self::Ex02),
            "03_addition" => Ok(Self::Ex03),
            "04_subtraction" => Ok(Self::Ex04),
            "05_reg_to_reg" => Ok(Self::Ex05),
            "06_bitwise_and" => Ok(Self::Ex06),
            "07_bitwise_or" => Ok(Self::Ex07),
            "08_xor" => Ok(Self::Ex08),
            "09_shift_left" => Ok(Self::Ex09),
            "10_stack" => Ok(Self::Ex10),
            "11_bitwise_not" => Ok(Self::Ex11),
            "12_shift_right" => Ok(Self::Ex12),
            "13_inc_dec" => Ok(Self::Ex13),
            "14_mul" => Ok(Self::Ex14),
            "15_div" => Ok(Self::Ex15),
            "16_neg" => Ok(Self::Ex16),
            "17_cmp" => Ok(Self::Ex17),
            "18_loop" => Ok(Self::Ex18),
            "19_read_mem" => Ok(Self::Ex19),
            "20_write_mem" => Ok(Self::Ex20),
            "21_reg_ind_address" => Ok(Self::Ex21),
            "22_source_index" => Ok(Self::Ex22),
            "23_subroutines" => Ok(Self::Ex23),
            "24_xchg" => Ok(Self::Ex24),
            "25_carry_flag" => Ok(Self::Ex25),
            "26_bit_ceck" => Ok(Self::Ex26),
            "27_pusha_popa" => Ok(Self::Ex27),
            "28_sign_comp" => Ok(Self::Ex28),
            "29_rol" => Ok(Self::Ex29),
            "30_test_1" => Ok(Self::Ex30),
            "31_test_2" => Ok(Self::Ex31),
            "32_test_3" => Ok(Self::Ex32),
            _ => Err(anyhow::anyhow!("Unknown exercise: {}", s)),
        }
    }
}

#[allow(dead_code)]
impl ExerciseName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ex01 => "01_bare_metal",
            Self::Ex02 => "02_halves",
            Self::Ex03 => "03_addition",
            Self::Ex04 => "04_subtraction",
            Self::Ex05 => "05_reg_to_reg",
            Self::Ex06 => "06_bitwise_and",
            Self::Ex07 => "07_bitwise_or",
            Self::Ex08 => "08_xor",
            Self::Ex09 => "09_shift_left",
            Self::Ex10 => "10_stack",
            Self::Ex11 => "11_bitwise_not",
            Self::Ex12 => "12_shift_right",
            Self::Ex13 => "13_inc_dec",
            Self::Ex14 => "14_mul",
            Self::Ex15 => "15_div",
            Self::Ex16 => "16_neg",
            Self::Ex17 => "17_cmp",
            Self::Ex18 => "18_loop",
            Self::Ex19 => "19_read_mem",
            Self::Ex20 => "20_write_mem",
            Self::Ex21 => "21_reg_ind_address",
            Self::Ex22 => "22_source_index",
            Self::Ex23 => "23_subroutines",
            Self::Ex24 => "24_xchg",
            Self::Ex25 => "25_carry_flag",
            Self::Ex26 => "26_bit_ceck",
            Self::Ex27 => "27_pusha_popa",
            Self::Ex28 => "28_sign_comp",
            Self::Ex29 => "29_rol",
            Self::Ex30 => "30_test_1",
            Self::Ex31 => "31_test_2",
            Self::Ex32 => "32_test_3",
        }
    }
}

pub struct ExerciseTests {
    pub bare_metal_01:      ProgrammaticSuite,
    pub halves_02:          ProgrammaticSuite,
    pub addition_03:        ProgrammaticSuite,
    pub subtraction_04:     ProgrammaticSuite,
    pub reg_to_reg_05:      ProgrammaticSuite,
    pub bitwise_and_06:     ProgrammaticSuite,
    pub bitwise_or_07:      ProgrammaticSuite,
    pub xor_08:             ProgrammaticSuite,
    pub shift_left_09:      ProgrammaticSuite,
    pub stack_10:           ProgrammaticSuite,
    pub bitwise_not_11:     ProgrammaticSuite,
    pub shift_right_12:     ProgrammaticSuite,
    pub inc_dec_13:         ProgrammaticSuite,
    pub mul_14:             ProgrammaticSuite,
    pub div_15:             ProgrammaticSuite,
    pub neg_16:             ProgrammaticSuite,
    pub cmp_17:             ProgrammaticSuite,
    pub loop_18:            ProgrammaticSuite,
    pub read_mem_19:        ProgrammaticSuite,
    pub write_mem_20:       ProgrammaticSuite,
    pub reg_ind_address_21: ProgrammaticSuite,
    pub source_index_22:    ProgrammaticSuite,
    pub subroutines_23:     ProgrammaticSuite,
    pub xchg_24:            ProgrammaticSuite,
    pub carry_flag_25:      ProgrammaticSuite,
    pub bit_ceck_26:        ProgrammaticSuite,
    pub pusha_popa_27:      ProgrammaticSuite,
    pub sign_comp_28:       ProgrammaticSuite,
    pub rol_29:             ProgrammaticSuite,
    pub test_1_30:          ProgrammaticSuite,
    pub test_2_31:          ProgrammaticSuite,
    pub test_3_32:          ProgrammaticSuite,
}

pub static EXERCISE_TESTS: OnceLock<ExerciseTests> = OnceLock::new();

pub fn get_test_suite(name: ExerciseName) -> Option<&'static ProgrammaticSuite> {
    let tests = EXERCISE_TESTS.get_or_init(|| ExerciseTests::new());
    match name {
        ExerciseName::Ex01 => Some(&tests.bare_metal_01),
        ExerciseName::Ex02 => Some(&tests.halves_02),
        ExerciseName::Ex03 => Some(&tests.addition_03),
        ExerciseName::Ex04 => Some(&tests.subtraction_04),
        ExerciseName::Ex05 => Some(&tests.reg_to_reg_05),
        ExerciseName::Ex06 => Some(&tests.bitwise_and_06),
        ExerciseName::Ex07 => Some(&tests.bitwise_or_07),
        ExerciseName::Ex08 => Some(&tests.xor_08),
        ExerciseName::Ex09 => Some(&tests.shift_left_09),
        ExerciseName::Ex10 => Some(&tests.stack_10),
        ExerciseName::Ex11 => Some(&tests.bitwise_not_11),
        ExerciseName::Ex12 => Some(&tests.shift_right_12),
        ExerciseName::Ex13 => Some(&tests.inc_dec_13),
        ExerciseName::Ex14 => Some(&tests.mul_14),
        ExerciseName::Ex15 => Some(&tests.div_15),
        ExerciseName::Ex16 => Some(&tests.neg_16),
        ExerciseName::Ex17 => Some(&tests.cmp_17),
        ExerciseName::Ex18 => Some(&tests.loop_18),
        ExerciseName::Ex19 => Some(&tests.read_mem_19),
        ExerciseName::Ex20 => Some(&tests.write_mem_20),
        ExerciseName::Ex21 => Some(&tests.reg_ind_address_21),
        ExerciseName::Ex22 => Some(&tests.source_index_22),
        ExerciseName::Ex23 => Some(&tests.subroutines_23),
        ExerciseName::Ex24 => Some(&tests.xchg_24),
        ExerciseName::Ex25 => Some(&tests.carry_flag_25),
        ExerciseName::Ex26 => Some(&tests.bit_ceck_26),
        ExerciseName::Ex27 => Some(&tests.pusha_popa_27),
        ExerciseName::Ex28 => Some(&tests.sign_comp_28),
        ExerciseName::Ex29 => Some(&tests.rol_29),
        ExerciseName::Ex30 => Some(&tests.test_1_30),
        ExerciseName::Ex31 => Some(&tests.test_2_31),
        ExerciseName::Ex32 => Some(&tests.test_3_32),
    }
}

impl ExerciseTests {
    pub fn new() -> Self {
        Self {
            bare_metal_01:      ProgrammaticSuite {
                name:         "01_bare_metal",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0x1337)]),
                }],
            },
            halves_02:          ProgrammaticSuite {
                name:         "02_halves",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0xABCD)]),
                }],
            },
            addition_03:        ProgrammaticSuite {
                name:         "03_addition",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0x0015)]),
                }],
            },
            subtraction_04:     ProgrammaticSuite {
                name:         "04_subtraction",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check CX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "CX", RegisterX86::CX, 0x0041)]),
                }],
            },
            reg_to_reg_05:      ProgrammaticSuite {
                name:         "05_reg_to_reg",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX & DX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| {
                        Ok(vec![
                            check_reg(emu, "AX", RegisterX86::AX, 0xBEEF),
                            check_reg(emu, "DX", RegisterX86::DX, 0xBEEF),
                        ])
                    },
                }],
            },
            bitwise_and_06:     ProgrammaticSuite {
                name:         "06_bitwise_and",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0x00CD)]),
                }],
            },
            bitwise_or_07:      ProgrammaticSuite {
                name:         "07_bitwise_or",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0x0FF0)]),
                }],
            },
            xor_08:             ProgrammaticSuite {
                name:         "08_xor",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check CX & DX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| {
                        Ok(vec![
                            check_reg(emu, "CX", RegisterX86::CX, 0x0000),
                            check_reg(emu, "DX", RegisterX86::DX, 0x0000),
                        ])
                    },
                }],
            },
            shift_left_09:      ProgrammaticSuite {
                name:         "09_shift_left",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0x0030)]),
                }],
            },
            stack_10:           ProgrammaticSuite {
                name:         "10_stack",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0xCAFE)]),
                }],
            },
            bitwise_not_11:     ProgrammaticSuite {
                name:         "11_bitwise_not",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0xFF00)]),
                }],
            },
            shift_right_12:     ProgrammaticSuite {
                name:         "12_shift_right",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0x0010)]),
                }],
            },
            inc_dec_13:         ProgrammaticSuite {
                name:         "13_inc_dec",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0x000C)]),
                }],
            },
            mul_14:             ProgrammaticSuite {
                name:         "14_mul",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX & DX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| {
                        Ok(vec![
                            check_reg(emu, "AX", RegisterX86::AX, 0x001E),
                            check_reg(emu, "DX", RegisterX86::DX, 0x0000),
                        ])
                    },
                }],
            },
            div_15:             ProgrammaticSuite {
                name:         "15_div",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX & DX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| {
                        Ok(vec![
                            check_reg(emu, "AX", RegisterX86::AX, 0x0009),
                            check_reg(emu, "DX", RegisterX86::DX, 0x0009),
                        ])
                    },
                }],
            },
            neg_16:             ProgrammaticSuite {
                name:         "16_neg",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0xFFFB)]),
                }],
            },
            cmp_17:             ProgrammaticSuite {
                name:         "17_cmp",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0x0001)]),
                }],
            },
            loop_18:            ProgrammaticSuite {
                name:         "18_loop",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0x000C)]),
                }],
            },
            read_mem_19:        ProgrammaticSuite {
                name:         "19_read_mem",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0xDEAD)]),
                }],
            },
            write_mem_20:       ProgrammaticSuite {
                name:         "20_write_mem",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX & Memory result",
                    setup:  |_| Ok(()),
                    verify: |emu, labels| {
                        Ok(vec![
                            check_reg(emu, "AX", RegisterX86::AX, 0x000F),
                            check_mem(emu, labels, "result", 0x000F, 2),
                        ])
                    },
                }],
            },
            reg_ind_address_21: ProgrammaticSuite {
                name:         "21_reg_ind_address",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0xF00D)]),
                }],
            },
            source_index_22:    ProgrammaticSuite {
                name:         "22_source_index",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0x0006)]),
                }],
            },
            subroutines_23:     ProgrammaticSuite {
                name:         "23_subroutines",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0x0012)]),
                }],
            },
            xchg_24:            ProgrammaticSuite {
                name:         "24_xchg",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX & BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| {
                        Ok(vec![
                            check_reg(emu, "AX", RegisterX86::AX, 0x2222),
                            check_reg(emu, "BX", RegisterX86::BX, 0x1111),
                        ])
                    },
                }],
            },
            carry_flag_25:      ProgrammaticSuite {
                name:         "25_carry_flag",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX & DX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| {
                        Ok(vec![
                            check_reg(emu, "AX", RegisterX86::AX, 0x0000),
                            check_reg(emu, "DX", RegisterX86::DX, 0x0002),
                        ])
                    },
                }],
            },
            bit_ceck_26:        ProgrammaticSuite {
                name:         "26_bit_ceck",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0x0001)]),
                }],
            },
            pusha_popa_27:      ProgrammaticSuite {
                name:         "27_pusha_popa",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX, BX, CX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| {
                        Ok(vec![
                            check_reg(emu, "AX", RegisterX86::AX, 0x0001),
                            check_reg(emu, "BX", RegisterX86::BX, 0x0002),
                            check_reg(emu, "CX", RegisterX86::CX, 0x0003),
                        ])
                    },
                }],
            },
            sign_comp_28:       ProgrammaticSuite {
                name:         "28_sign_comp",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0xFFFF)]),
                }],
            },
            rol_29:             ProgrammaticSuite {
                name:         "29_rol",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0x0003)]),
                }],
            },
            test_1_30:          ProgrammaticSuite {
                name:         "30_test_1",
                target_label: Some("abs_val"),
                cases:        vec![
                    ProgrammaticCase {
                        name:   "Neg Input (-10)",
                        setup:  |emu| set_reg(emu, RegisterX86::AX, 0xFFF6),
                        verify: |emu, _| {
                            Ok(vec![check_reg(emu, "AX (|-10|)", RegisterX86::AX, 10)])
                        },
                    },
                    ProgrammaticCase {
                        name:   "Pos Input (7)",
                        setup:  |emu| set_reg(emu, RegisterX86::AX, 7),
                        verify: |emu, _| Ok(vec![check_reg(emu, "AX (|7|)", RegisterX86::AX, 7)]),
                    },
                    ProgrammaticCase {
                        name:   "Zero Input (0)",
                        setup:  |emu| set_reg(emu, RegisterX86::AX, 0),
                        verify: |emu, _| Ok(vec![check_reg(emu, "AX (|0|)", RegisterX86::AX, 0)]),
                    },
                    ProgrammaticCase {
                        name:   "Neg Input (-1)",
                        setup:  |emu| set_reg(emu, RegisterX86::AX, 0xFFFF),
                        verify: |emu, _| Ok(vec![check_reg(emu, "AX (|-1|)", RegisterX86::AX, 1)]),
                    },
                ],
            },
            test_2_31:          ProgrammaticSuite {
                name:         "31_test_2",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check AX (max)",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "AX", RegisterX86::AX, 0x003E)]),
                }],
            },
            test_3_32:          ProgrammaticSuite {
                name:         "32_test_3",
                target_label: None,
                cases:        vec![ProgrammaticCase {
                    name:   "Check BX (popcount)",
                    setup:  |_| Ok(()),
                    verify: |emu, _| Ok(vec![check_reg(emu, "BX", RegisterX86::BX, 0x0009)]),
                }],
            },
        }
    }
}
