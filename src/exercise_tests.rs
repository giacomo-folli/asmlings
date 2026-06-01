use std::sync::OnceLock;
use std::str::FromStr;
use unicorn_engine::RegisterX86;
use crate::harness::{ProgrammaticSuite, ProgrammaticCase, set_reg, check_reg, check_mem};

macro_rules! define_exercises {
    (
        $(
            $variant:ident = $string_name:expr => $field:ident {
                name: $name:expr,
                target_label: $target:expr,
                cases: $cases:expr
            }
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ExerciseName {
            $($variant),*
        }

        impl FromStr for ExerciseName {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($string_name => Ok(Self::$variant),)*
                    _ => Err(anyhow::anyhow!("Unknown exercise: {}", s)),
                }
            }
        }

        #[allow(dead_code)]
        impl ExerciseName {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $string_name,)*
                }
            }
        }

        pub struct ExerciseTests {
            $(pub $field: ProgrammaticSuite),*
        }

        pub fn get_test_suite(name: ExerciseName) -> Option<&'static ProgrammaticSuite> {
            let tests = EXERCISE_TESTS.get_or_init(|| ExerciseTests::new());
            match name {
                $(ExerciseName::$variant => Some(&tests.$field),)*
            }
        }

        impl ExerciseTests {
            pub fn new() -> Self {
                Self {
                    $($field: ProgrammaticSuite {
                        name: $name,
                        target_label: $target,
                        cases: $cases,
                    }),*
                }
            }
        }
    };
}

pub static EXERCISE_TESTS: OnceLock<ExerciseTests> = OnceLock::new();

define_exercises! {
    Ex01BareMetal = "01_bare_metal" => bare_metal_01 {
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
    },
    Ex02Halves = "02_halves" => halves_02 {
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
    },
    Ex03Addition = "03_addition" => addition_03 {
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
    },
    Ex04Subtraction = "04_subtraction" => subtraction_04 {
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
    },
    Ex05RegToReg = "05_reg_to_reg" => reg_to_reg_05 {
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
    },
    Ex06BitwiseAnd = "06_bitwise_and" => bitwise_and_06 {
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
    },
    Ex07BitwiseOr = "07_bitwise_or" => bitwise_or_07 {
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
    },
    Ex08Xor = "08_xor" => xor_08 {
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
    },
    Ex09ShiftLeft = "09_shift_left" => shift_left_09 {
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
    },
    Ex10Stack = "10_stack" => stack_10 {
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
    },
    Ex11BitwiseNot = "11_bitwise_not" => bitwise_not_11 {
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
    },
    Ex12ShiftRight = "12_shift_right" => shift_right_12 {
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
    },
    Ex13IncDec = "13_inc_dec" => inc_dec_13 {
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
    },
    Ex14Mul = "14_mul" => mul_14 {
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
    },
    Ex15Div = "15_div" => div_15 {
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
    },
    Ex16Neg = "16_neg" => neg_16 {
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
    },
    Ex17Cmp = "17_cmp" => cmp_17 {
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
    },
    Ex18Loop = "18_loop" => loop_18 {
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
    },
    Ex19ReadMem = "19_read_mem" => read_mem_19 {
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
    },
    Ex20WriteMem = "20_write_mem" => write_mem_20 {
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
    },
    Ex21RegIndAddress = "21_reg_ind_address" => reg_ind_address_21 {
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
    },
    Ex22SourceIndex = "22_source_index" => source_index_22 {
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
    },
    Ex23Subroutines = "23_subroutines" => subroutines_23 {
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
    },
    Ex24Xchg = "24_xchg" => xchg_24 {
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
    },
    Ex25CarryFlag = "25_carry_flag" => carry_flag_25 {
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
    },
    Ex26BitCeck = "26_bit_ceck" => bit_ceck_26 {
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
    },
    Ex27PushaPopa = "27_pusha_popa" => pusha_popa_27 {
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
    },
    Ex28SignComp = "28_sign_comp" => sign_comp_28 {
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
    },
    Ex29Rol = "29_rol" => rol_29 {
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
    },
    Ex30Test1 = "30_test_1" => test_1_30 {
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
    },
    Ex31Test2 = "31_test_2" => test_2_31 {
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
    },
    Ex32Test3 = "32_test_3" => test_3_32 {
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
    },
}
