use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;
use crate::PasswordDatabase;

pub fn challenge() -> Challenge {
    return Challenge::new(
        valid_passwords_by_charater_count,
        valid_passwords_by_character_position,
        String::from("resources/password_database.txt"),
    );
}

fn valid_passwords_by_charater_count(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut password_database = PasswordDatabase::new(&raw_lines)?;

    return Ok(
        [(String::from("valid password count"), password_database.valid_passwords_by_character_count())]
        .iter()
        .cloned()
        .collect()
    );
}

fn valid_passwords_by_character_position(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut password_database = PasswordDatabase::new(&raw_lines)?;

    return Ok(
        [(String::from("valid password count"), password_database.valid_passwords_by_character_position())]
        .iter()
        .cloned()
        .collect()
    );
}
