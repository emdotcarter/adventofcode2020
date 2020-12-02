use std::io::BufRead;

#[derive(Debug)]
pub enum InputError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

pub fn buf_reader_from_filepath(filepath: &str) -> Result<std::io::BufReader<std::fs::File>, InputError> {
    return Ok(
        std::io::BufReader::new(
            std::fs::File::open(filepath)
            .map_err(InputError::Io)?
        )
    );
}

pub fn lines_to_usize(reader: &mut std::io::BufReader<std::fs::File>) -> Result<Vec<usize>, InputError> {
    let mut numbers = Vec::<usize>::new();
    for line in reader.lines() {
        numbers.push(line.map_err(InputError::Io)?.parse::<usize>().map_err(InputError::Parse)?);
    }

    return Ok(numbers);
}

pub struct ExpenseReport {
    values: Vec<usize>,
}

pub struct PasswordDatabaseEntry {
    is_valid_by_count: bool,
    is_valid_by_position: bool,
}

pub struct PasswordDatabase {
    entries: Vec<PasswordDatabaseEntry>,
}

impl ExpenseReport {
    pub fn new(values: &Vec<usize>) -> ExpenseReport {
        return ExpenseReport {
            values: values.to_vec(),
        };
    }

    pub fn product_from_target_two_sum(&mut self, target_sum: usize) -> usize {
        self.values.retain(|&v| v <= target_sum);
        for value1 in &self.values {
            for value2 in (&self.values).into_iter().filter(|&v| value1 + v == target_sum).collect::<Vec<&usize>>() {
                return value1 * value2;
            }
        }

        return 0;
    }

    pub fn product_from_target_three_sum(&mut self, target_sum: usize) -> usize {
        self.values.retain(|&v| v <= target_sum);
        for value1 in &self.values {
            for value2 in (&self.values).into_iter().filter(|&v| value1 + v <= target_sum).collect::<Vec<&usize>>() {
                for value3 in (&self.values).into_iter().filter(|&v| value1 + value2 + v == target_sum).collect::<Vec<&usize>>() {
                    return value1 * value2 * value3;
                }

            }
        }

        return 0;
    }
}

impl PasswordDatabaseEntry {
    pub fn new(condition1: usize, condition2: usize, required_character: char, password: &str) -> Result<PasswordDatabaseEntry, InputError> {
        let required_character_count = password.matches(required_character).count();
        let is_valid_by_count = required_character_count >= condition1 && required_character_count <= condition2;

        let is_valid_by_condition1 = match password.chars().nth(condition1) {
            Some(c) => c == required_character,
            None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad character conditions"))),
        };
        let is_valid_by_condition2 = match password.chars().nth(condition2) {
            Some(c) => c == required_character,
            None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad character conditions"))),
        };

        return Ok(
            PasswordDatabaseEntry {
                is_valid_by_count: is_valid_by_count,
                is_valid_by_position: is_valid_by_condition1 ^ is_valid_by_condition2,
            }
        );
    }

    pub fn is_valid(&self) -> bool {
        return self.is_valid_by_count;
    }
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
            let condition1 = conditions[0].parse::<usize>().map_err(InputError::Parse)? - 1;
            let condition2 = conditions[1].parse::<usize>().map_err(InputError::Parse)? - 1;

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
