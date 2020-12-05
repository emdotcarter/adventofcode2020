use crate::ChallengeFn;
use crate::ResultHashMap;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub struct Challenge {
    fn_part1: ChallengeFn,
    fn_part2: ChallengeFn,
    default_filepath: String,
}

impl Challenge {
    pub fn new(fn_part1: ChallengeFn, fn_part2: ChallengeFn, default_filepath: String) -> Challenge {
        return Challenge {
            fn_part1: fn_part1,
            fn_part2: fn_part2,
            default_filepath: default_filepath,
        }
    }

    pub fn run_part1(&self, args: &[String]) -> ResultHashMap {
        if args.len() > 0 {
            return (self.fn_part1)(args);
        } else {
            return (self.fn_part1)(&[self.default_filepath.clone()]);
        }
    }

    pub fn run_part2(&self, args: &[String]) -> ResultHashMap {
        if args.len() > 0 {
            return (self.fn_part2)(args);
        } else {
            return (self.fn_part2)(&[self.default_filepath.clone()]);
        }
    }
}
