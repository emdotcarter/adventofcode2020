#[cfg(test)]
mod tests_day2 {
    #[test]
    fn valid_passwords_by_count() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        );
        let mut password_database = adventofcode2020::PasswordDatabase::new(&values)?;

        let actual = password_database.valid_passwords_by_character_count();

        assert_eq!(2, actual);

        return Ok(());
    }

    #[test]
    fn valid_passwords_by_position() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        );
        let mut password_database = adventofcode2020::PasswordDatabase::new(&values)?;

        let actual = password_database.valid_passwords_by_character_position();

        assert_eq!(1, actual);

        return Ok(());
    }
}
