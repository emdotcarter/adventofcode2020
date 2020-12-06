use crate::input_error::InputError;

pub struct PasswordDatabaseEntry {
    is_valid_by_count: bool,
    is_valid_by_position: bool,
}

impl PasswordDatabaseEntry {
    pub fn new(condition1: usize, condition2: usize, required_character: char, password: &str) -> Result<PasswordDatabaseEntry, InputError> {
        return Ok(
            PasswordDatabaseEntry {
                is_valid_by_count: PasswordDatabaseEntry::password_valid_by_character_count(condition1, condition2, &required_character, &password),
                is_valid_by_position: PasswordDatabaseEntry::password_valid_by_character_position(condition1 - 1, condition2 - 1, &required_character, &password)?,
            }
        );
    }

    fn password_valid_by_character_count(min_count: usize, max_count: usize, required_character: &char, password: &str) -> bool {
        let actual_character_count = password.matches(&required_character.to_string()).count();
        return actual_character_count >= min_count && actual_character_count <= max_count;
    }

    fn password_valid_by_character_position(position1: usize, position2: usize, required_character: &char, password: &str) -> Result<bool, InputError> {
        let found_in_position1 = match password.chars().nth(position1) {
            Some(c) => c == *required_character,
            None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad character conditions"))),
        };
        let found_in_position2 = match password.chars().nth(position2) {
            Some(c) => c == *required_character,
            None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad character conditions"))),
        };

        return Ok(found_in_position1 ^ found_in_position2);
    }
}

pub struct PasswordDatabase {
    entries: Vec<PasswordDatabaseEntry>,
}

impl PasswordDatabase {
    pub fn new(raw_entries: &Vec<String>) -> Result<PasswordDatabase, InputError> {
        let mut entries:Vec<PasswordDatabaseEntry> = Vec::new();
        for raw_entry in raw_entries {
            let mut elements = raw_entry.split(" ");

            let conditions:Vec<&str> = match elements.next() {
                Some(r) => r.split("-").collect(),
                None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad character conditions"))),
            };
            let condition1 = conditions[0].parse::<usize>().map_err(InputError::Parse)?;
            let condition2 = conditions[1].parse::<usize>().map_err(InputError::Parse)?;

            let required_character_blob = match elements.next() {
                Some(c) => c.chars().next(),
                None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad required character"))),
            };
            let required_character = match required_character_blob {
                Some(c) => c,
                None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad required character"))),
            };

            let password = match elements.next() {
                Some(p) => p,
                None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad password"))),
            };

            entries.push(PasswordDatabaseEntry::new(condition1, condition2, required_character, password)?);
        }

        return Ok(
            PasswordDatabase {
                entries: entries,
            }
        );
    }

    pub fn valid_passwords_by_character_count(&mut self) -> usize {
        return (&self.entries).into_iter().filter(|e| e.is_valid_by_count).count();
    }

    pub fn valid_passwords_by_character_position(&mut self) -> usize {
        return (&self.entries).into_iter().filter(|e| e.is_valid_by_position).count();
    }
}

#[cfg(test)]
mod tests {
    use super::InputError;
    use super::PasswordDatabase;

    #[test]
    fn valid_passwords_by_count() -> Result<(), InputError> {
        let values = vec!(
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        );
        let mut password_database = PasswordDatabase::new(&values)?;

        let actual = password_database.valid_passwords_by_character_count();

        assert_eq!(2, actual);

        return Ok(());
    }

    #[test]
    fn valid_passwords_by_character_position() -> Result<(), InputError> {
        let values = vec!(
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        );
        let mut password_database = PasswordDatabase::new(&values)?;

        let actual = password_database.valid_passwords_by_character_position();

        assert_eq!(1, actual);

        return Ok(());
    }
}
