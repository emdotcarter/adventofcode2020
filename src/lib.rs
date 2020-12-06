mod input_error;

pub mod challenges;

mod expense_report;
mod password_database;
mod slope_map;
mod passport;
pub mod binary_partitioner;

pub use crate::input_error::InputError;

pub type ResultHashMap = Result<std::collections::HashMap<String, usize>, InputError>;
pub type ChallengeFn = fn(&[String]) -> ResultHashMap;
pub type ResultChallenge = Result<fn() -> challenges::Challenge, InputError>;

pub use expense_report::ExpenseReport;
pub use password_database::PasswordDatabase;
pub use slope_map::MovementPath;
pub use slope_map::SlopeMap;
pub use passport::Passport;

pub fn challenge_by_day(day: &str) -> ResultChallenge {
    let challenges = [
        challenges::day1::challenge,
        challenges::day2::challenge,
        challenges::day3::challenge,
        challenges::day4::challenge,
        challenges::day5::challenge,
        challenges::day6::challenge,
    ];

    let day_index = day
        .parse::<usize>()
        .map_err(InputError::Parse)? - 1;

    return Ok(challenges[day_index]);
}

pub fn file_lines_to_string_vec(filepath: &str) -> Result<Vec<String>, InputError> {
    use std::io::BufRead;

    let reader = std::io::BufReader::new(
        std::fs::File::open(filepath).map_err(InputError::Io)?
    );
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.map_err(InputError::Io)?);
    }

    return Ok(lines);
}

pub fn print_hash_map(result: std::collections::HashMap<String, usize>) {
    println!("Result:");
    for (key, value) in result {
        println!("  {}: {}", key, value);
    }
}
