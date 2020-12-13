use crate::ChallengeFn;
use crate::ResultHashMap;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub struct Challenge {
    functions: [ChallengeFn; 2],
    default_filepath: String,
}

impl Challenge {
    pub fn new(fn_part1: ChallengeFn, fn_part2: ChallengeFn, default_filepath: String) -> Challenge {
        return Challenge {
            functions: [fn_part1, fn_part2],
            default_filepath: default_filepath,
        }
    }

    pub fn run_fn(&self, part: usize, args: &[String]) -> ResultHashMap {
        if args.len() > 0 {
            return (self.functions[part - 1])(args);
        } else {
            return (self.functions[part - 1])(&[self.default_filepath.clone()]);
        }
    }
}

#[macro_export]
macro_rules! challenge_test {
    ($name:ident, $part:literal, $expected:literal) => {
        #[test]
        fn $name() {
            let challenge = challenge();
            let actual = challenge.run_fn($part, &[challenge.default_filepath.clone()]).unwrap();

            assert_eq!($expected, *actual.values().next().unwrap());
        }
    }
}

#[macro_export]
macro_rules! challenge_tests {
    ($part1_expected:literal, $part2_expected:literal) => {
        crate::challenge_test!(test_part1, 1, $part1_expected);
        crate::challenge_test!(test_part2, 2, $part2_expected);
    }
}
