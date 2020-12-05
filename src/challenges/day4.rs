use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;
use crate::Passport;

pub fn challenge() -> Challenge {
    return Challenge::new(
        valid_passports_without_validations,
        valid_passports_with_validations,
        String::from("resources/passport_database.txt"),
    );
}

fn valid_passports_without_validations(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut passports: Vec<Passport> = Vec::new();
    let mut key_value_row: String = String::from("");
    for line in raw_lines {
        if line == "" {
            passports.push(Passport::new(&key_value_row.trim())?);
            key_value_row = String::from("");
        } else {
            key_value_row.push_str(" ");
            key_value_row.push_str(&line);
        }
    }

    if key_value_row != "" {
        passports.push(Passport::new(&key_value_row.trim())?);
    }

    return Ok(
        [(String::from("valid passports"), passports.iter().filter(|p| p.is_valid(false)).count())]
        .iter()
        .cloned()
        .collect()
    );
}

fn valid_passports_with_validations(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut passports: Vec<Passport> = Vec::new();
    let mut key_value_row: String = String::from("");
    for line in raw_lines {
        if line == "" {
            passports.push(Passport::new(&key_value_row.trim())?);
            key_value_row = String::from("");
        } else {
            key_value_row.push_str(" ");
            key_value_row.push_str(&line);
        }
    }

    if key_value_row != "" {
        passports.push(Passport::new(&key_value_row.trim())?);
    }

    return Ok(
        [(String::from("valid passports"), passports.iter().filter(|p| p.is_valid(true)).count())]
        .iter()
        .cloned()
        .collect()
    );
}
