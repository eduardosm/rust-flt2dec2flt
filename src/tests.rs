use crate::{FloatExt, PreFormatted, PreParsed};

trait FloatApprox {
    fn approx(self, other: Self, error: Self) -> bool;
}

impl FloatApprox for f32 {
    fn approx(self, other: Self, error: Self) -> bool {
        (self - other).abs() <= error
    }
}

impl FloatApprox for f64 {
    fn approx(self, other: Self, error: Self) -> bool {
        (self - other).abs() <= error
    }
}

struct Test<'a, F: Copy + FloatExt + FloatApprox> {
    value: F,
    preparsed: &'a [(PreParsed<'a>, F)],
    preformatted_shortest: PreFormatted<'a>,
    preformatted_exact_exp: &'a [(usize, PreFormatted<'a>)],
    preformatted_exact_fixed: &'a [(usize, PreFormatted<'a>)],
}

impl<F: Copy + FloatExt + FloatApprox + std::fmt::Display> Test<'_, F> {
    fn run(&self) {
        for &(preparsed, allowed_error) in self.preparsed.iter() {
            let v = F::from_preparsed(preparsed).unwrap();
            assert!(
                v.approx(self.value, allowed_error),
                "failed approx({}, {}, {})",
                v,
                self.value,
                allowed_error,
            );
        }

        let mut buf = std::vec![0; crate::PREFORMAT_SHORTEST_BUF_LEN];
        assert_eq!(
            self.value.preformat_shortest(&mut buf),
            self.preformatted_shortest
        );

        for &(num_digits, preformatted) in self.preformatted_exact_exp.iter() {
            let mut buf = std::vec![0; num_digits];
            assert_eq!(
                self.value.preformat_exact_exp(&mut buf, num_digits),
                preformatted
            );
        }

        for &(num_frac_digits, preformatted) in self.preformatted_exact_fixed.iter() {
            let mut buf = std::vec![0; crate::PREFORMAT_EXACT_FIXED_BASE_BUF_LEN + num_frac_digits];
            assert_eq!(
                self.value.preformat_exact_fixed(&mut buf, num_frac_digits),
                preformatted
            );
        }
    }
}

#[test]
fn test_f32() {
    Test::<f32> {
        value: 0.0,
        preparsed: &[
            (
                PreParsed {
                    sign: false,
                    int_digits: b"",
                    frac_digits: b"",
                    exp: 0,
                },
                0.0,
            ),
            (
                PreParsed {
                    sign: false,
                    int_digits: b"0",
                    frac_digits: b"0",
                    exp: 10,
                },
                0.0,
            ),
        ],
        preformatted_shortest: PreFormatted::Zero(false),
        preformatted_exact_exp: &[(3, PreFormatted::Zero(false))],
        preformatted_exact_fixed: &[(3, PreFormatted::Zero(false))],
    }
    .run();

    Test::<f32> {
        value: -0.0,
        preparsed: &[(
            PreParsed {
                sign: true,
                int_digits: b"",
                frac_digits: b"",
                exp: 0,
            },
            0.0,
        )],
        preformatted_shortest: PreFormatted::Zero(true),
        preformatted_exact_exp: &[(3, PreFormatted::Zero(true))],
        preformatted_exact_fixed: &[(3, PreFormatted::Zero(true))],
    }
    .run();

    Test::<f32> {
        value: core::f32::NAN,
        preparsed: &[],
        preformatted_shortest: PreFormatted::NaN,
        preformatted_exact_exp: &[(3, PreFormatted::NaN)],
        preformatted_exact_fixed: &[(3, PreFormatted::NaN)],
    }
    .run();

    Test::<f32> {
        value: core::f32::INFINITY,
        preparsed: &[],
        preformatted_shortest: PreFormatted::Inf(false),
        preformatted_exact_exp: &[(3, PreFormatted::Inf(false))],
        preformatted_exact_fixed: &[(3, PreFormatted::Inf(false))],
    }
    .run();

    Test::<f32> {
        value: core::f32::NEG_INFINITY,
        preparsed: &[],
        preformatted_shortest: PreFormatted::Inf(true),
        preformatted_exact_exp: &[(3, PreFormatted::Inf(true))],
        preformatted_exact_fixed: &[(3, PreFormatted::Inf(true))],
    }
    .run();

    Test::<f32> {
        value: 3.0,
        preparsed: &[
            (
                PreParsed {
                    sign: false,
                    int_digits: b"3",
                    frac_digits: b"",
                    exp: 0,
                },
                1.0e-6,
            ),
            (
                PreParsed {
                    sign: false,
                    int_digits: b"30",
                    frac_digits: b"",
                    exp: -1,
                },
                1.0e-6,
            ),
            (
                PreParsed {
                    sign: false,
                    int_digits: b"",
                    frac_digits: b"3",
                    exp: 1,
                },
                1.0e-6,
            ),
        ],
        preformatted_shortest: PreFormatted::Finite(false, b"3", 1),
        preformatted_exact_exp: &[(3, PreFormatted::Finite(false, b"300", 1))],
        preformatted_exact_fixed: &[(3, PreFormatted::Finite(false, b"3000", 1))],
    }
    .run();

    Test::<f32> {
        value: -3.0,
        preparsed: &[(
            PreParsed {
                sign: true,
                int_digits: b"3",
                frac_digits: b"",
                exp: 0,
            },
            1.0e-6,
        )],
        preformatted_shortest: PreFormatted::Finite(true, b"3", 1),
        preformatted_exact_exp: &[(3, PreFormatted::Finite(true, b"300", 1))],
        preformatted_exact_fixed: &[(3, PreFormatted::Finite(true, b"3000", 1))],
    }
    .run();

    Test::<f32> {
        value: 0.3,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"0",
                frac_digits: b"03",
                exp: 1,
            },
            1.0e-6,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"3", 0),
        preformatted_exact_exp: &[(3, PreFormatted::Finite(false, b"300", 0))],
        preformatted_exact_fixed: &[(3, PreFormatted::Finite(false, b"300", 0))],
    }
    .run();

    Test::<f32> {
        value: 0.03,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"30",
                frac_digits: b"00",
                exp: -3,
            },
            1.0e-6,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"3", -1),
        preformatted_exact_exp: &[(3, PreFormatted::Finite(false, b"300", -1))],
        preformatted_exact_fixed: &[(3, PreFormatted::Finite(false, b"30", -1))],
    }
    .run();

    Test::<f32> {
        value: 30.0,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"30",
                frac_digits: b"",
                exp: 0,
            },
            1.0e-6,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"3", 2),
        preformatted_exact_exp: &[(3, PreFormatted::Finite(false, b"300", 2))],
        preformatted_exact_fixed: &[(3, PreFormatted::Finite(false, b"30000", 2))],
    }
    .run();

    Test::<f32> {
        value: 0.303,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"30",
                frac_digits: b"3",
                exp: -2,
            },
            1.0e-6,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"303", 0),
        preformatted_exact_exp: &[(4, PreFormatted::Finite(false, b"3030", 0))],
        preformatted_exact_fixed: &[(4, PreFormatted::Finite(false, b"3030", 0))],
    }
    .run();

    Test::<f32> {
        value: 30.3,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"3",
                frac_digits: b"03",
                exp: 1,
            },
            1.0e-6,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"303", 2),
        preformatted_exact_exp: &[(4, PreFormatted::Finite(false, b"3030", 2))],
        preformatted_exact_fixed: &[(4, PreFormatted::Finite(false, b"303000", 2))],
    }
    .run();

    Test::<f32> {
        value: 0.222e10,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"0",
                frac_digits: b"222",
                exp: 10,
            },
            1.0e-6 * 1.0e10,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"222", 10),
        preformatted_exact_exp: &[(4, PreFormatted::Finite(false, b"2220", 10))],
        preformatted_exact_fixed: &[(4, PreFormatted::Finite(false, b"22200000000000", 10))],
    }
    .run();

    Test::<f32> {
        value: 0.222e35,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"0",
                frac_digits: b"222",
                exp: 35,
            },
            1.0e-6 * 1.0e35,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"222", 35),
        preformatted_exact_exp: &[(4, PreFormatted::Finite(false, b"2220", 35))],
        preformatted_exact_fixed: &[],
    }
    .run();

    Test::<f32> {
        value: 0.222e-10,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"0",
                frac_digits: b"222",
                exp: -10,
            },
            1.0e-6 * 1.0e-10,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"222", -10),
        preformatted_exact_exp: &[(4, PreFormatted::Finite(false, b"2220", -10))],
        preformatted_exact_fixed: &[(12, PreFormatted::Finite(false, b"22", -10))],
    }
    .run();

    Test::<f32> {
        value: 0.222e-30,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"0",
                frac_digits: b"222",
                exp: -30,
            },
            1.0e-6 * 1.0e-30,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"222", -30),
        preformatted_exact_exp: &[(4, PreFormatted::Finite(false, b"2220", -30))],
        preformatted_exact_fixed: &[(4, PreFormatted::Zero(false))],
    }
    .run();

    Test::<f32> {
        value: 0.222e-35,
        preparsed: &[],
        preformatted_shortest: PreFormatted::Finite(false, b"222", -35),
        preformatted_exact_exp: &[(4, PreFormatted::Finite(false, b"2220", -35))],
        preformatted_exact_fixed: &[(4, PreFormatted::Zero(false))],
    }
    .run();
}

#[test]
fn test_f64() {
    Test::<f64> {
        value: 0.0,
        preparsed: &[
            (
                PreParsed {
                    sign: false,
                    int_digits: b"",
                    frac_digits: b"",
                    exp: 0,
                },
                0.0,
            ),
            (
                PreParsed {
                    sign: false,
                    int_digits: b"0",
                    frac_digits: b"0",
                    exp: 10,
                },
                0.0,
            ),
        ],
        preformatted_shortest: PreFormatted::Zero(false),
        preformatted_exact_exp: &[(3, PreFormatted::Zero(false))],
        preformatted_exact_fixed: &[(3, PreFormatted::Zero(false))],
    }
    .run();

    Test::<f64> {
        value: -0.0,
        preparsed: &[(
            PreParsed {
                sign: true,
                int_digits: b"",
                frac_digits: b"",
                exp: 0,
            },
            0.0,
        )],
        preformatted_shortest: PreFormatted::Zero(true),
        preformatted_exact_exp: &[(3, PreFormatted::Zero(true))],
        preformatted_exact_fixed: &[(3, PreFormatted::Zero(true))],
    }
    .run();

    Test::<f64> {
        value: core::f64::NAN,
        preparsed: &[],
        preformatted_shortest: PreFormatted::NaN,
        preformatted_exact_exp: &[(3, PreFormatted::NaN)],
        preformatted_exact_fixed: &[(3, PreFormatted::NaN)],
    }
    .run();

    Test::<f64> {
        value: core::f64::INFINITY,
        preparsed: &[],
        preformatted_shortest: PreFormatted::Inf(false),
        preformatted_exact_exp: &[(3, PreFormatted::Inf(false))],
        preformatted_exact_fixed: &[(3, PreFormatted::Inf(false))],
    }
    .run();

    Test::<f64> {
        value: core::f64::NEG_INFINITY,
        preparsed: &[],
        preformatted_shortest: PreFormatted::Inf(true),
        preformatted_exact_exp: &[(3, PreFormatted::Inf(true))],
        preformatted_exact_fixed: &[(3, PreFormatted::Inf(true))],
    }
    .run();

    Test::<f64> {
        value: 0.123_456_789,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"0",
                frac_digits: b"123456789",
                exp: 0,
            },
            1.0e-12,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"123456789", 0),
        preformatted_exact_exp: &[(10, PreFormatted::Finite(false, b"1234567890", 0))],
        preformatted_exact_fixed: &[(4, PreFormatted::Finite(false, b"1235", 0))],
    }
    .run();

    Test::<f64> {
        value: 0.123_456_789e200,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"0",
                frac_digits: b"123456789",
                exp: 200,
            },
            1.0e-12 * 1.0e200,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"123456789", 200),
        preformatted_exact_exp: &[(10, PreFormatted::Finite(false, b"1234567890", 200))],
        preformatted_exact_fixed: &[],
    }
    .run();

    Test::<f64> {
        value: 0.123_456_789e-200,
        preparsed: &[(
            PreParsed {
                sign: false,
                int_digits: b"0",
                frac_digits: b"123456789",
                exp: -200,
            },
            1.0e-12 * 1.0e-200,
        )],
        preformatted_shortest: PreFormatted::Finite(false, b"123456789", -200),
        preformatted_exact_exp: &[(10, PreFormatted::Finite(false, b"1234567890", -200))],
        preformatted_exact_fixed: &[(10, PreFormatted::Zero(false))],
    }
    .run();
}
