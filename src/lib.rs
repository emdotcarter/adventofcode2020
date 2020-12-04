use std::io::BufRead;

#[derive(Debug)]
pub enum InputError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Regex(regex::Error),
}

pub fn file_lines_to_string_vec(filepath: &str) -> Result<Vec<String>, InputError> {
    let reader = std::io::BufReader::new(
        std::fs::File::open(filepath)
        .map_err(InputError::Io)?
    );
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.map_err(InputError::Io)?);
    }

    return Ok(lines);
}

pub struct ExpenseReport {
    values: Vec<usize>,
}

impl ExpenseReport {
    pub fn new(values: &Vec<String>) -> Result<ExpenseReport, InputError> {
        let mut parsed_values = Vec::<usize>::new();
        for value in values {
            parsed_values.push(
                value
                .parse::<usize>()
                .map_err(InputError::Parse)?
                );
        }

        return Ok(
            ExpenseReport { values: parsed_values },
        );
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

pub struct PasswordDatabaseEntry {
    is_valid_by_count: bool,
    is_valid_by_position: bool,
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

pub struct MovementPath {
    horizontal: i64,
    downward: i64,
}

impl MovementPath {
    pub fn new(horizontal: i64, downward: i64) -> MovementPath {
        return MovementPath {
            horizontal: horizontal,
            downward: downward,
        };
    }
}

pub struct SlopeMap {
    map: Vec<Vec<char>>,
}

impl SlopeMap {
    pub fn new(map_rows: &Vec<String>) -> SlopeMap {
        let mut map: Vec<Vec<char>> = Vec::new();

        for map_row in map_rows {
            let mut row = Vec::new();

            for c in map_row.chars() {
                row.push(c);
            }

            map.push(row);
        }

        return SlopeMap {map: map};
    }

    pub fn count_trees_on_traversal(&self, movement_path: &MovementPath) -> usize {
        let mut trees_hit = 0;
        let mut index: (i64, i64) = (0, 0);

        while index.1 < self.map.len() as i64 {
            let current_row = &self.map[index.1 as usize];

            if current_row[index.0 as usize] == SlopeMap::tree_char() {
                trees_hit += 1;
            }

            let mut new_x = index.0 + movement_path.horizontal;
            if new_x as usize >= current_row.len() {
                new_x -= current_row.len() as i64;
            }
            index = (new_x, index.1 + movement_path.downward);
        }

        return trees_hit;
    }

    #[inline]
    fn tree_char() -> char {
        return '#';
    }
}

pub struct Passport {
    value_map: std::collections::HashMap<String, String>,
}

impl Passport {
    pub fn new(key_value_row: &str) -> Result<Passport, InputError> {
        let mut value_map = std::collections::HashMap::<String, String>::new();
        let key_value_pairs = key_value_row
            .split(" ")
            .map(|p| p.split(":"))
            ;

        for mut pair in key_value_pairs {
            let key = match pair.next() {
                Some(k) => k,
                None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad key:value pair"))),
            };
            let value = match pair.next() {
                Some(v) => v,
                None => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad key:value pair"))),
            };

            value_map.insert(String::from(key), String::from(value));
        }

        return Ok(Passport { value_map: value_map });
    }

    pub fn is_valid(&self, apply_validators: bool) -> bool {
        let required_field_keys = Passport::required_field_keys();
        let validators = Passport::field_validators();

        if required_field_keys.iter().all(|k| self.value_map.contains_key(k)) {
            if apply_validators {
                return self.value_map.iter()
                    .all(|(k, v)| !validators.contains_key(k) || validators[k](v));
            } else {
                return true;
            }
        }

        return false;
    }

    fn required_field_keys() -> Vec<String> {
        return vec!(
            String::from("byr"),
            String::from("iyr"),
            String::from("eyr"),
            String::from("hgt"),
            String::from("hcl"),
            String::from("ecl"),
            String::from("pid"),
        );
    }

    fn field_validators() -> std::collections::HashMap::<String, fn(&String) -> bool> {
        let mut map = std::collections::HashMap::<String, fn(&String) -> bool>::new();
        map.insert(String::from("byr"), Passport::birth_year_valid);
        map.insert(String::from("iyr"), Passport::issue_year_valid);
        map.insert(String::from("eyr"), Passport::expiration_year_valid);
        map.insert(String::from("hgt"), Passport::height_valid);
        map.insert(String::from("hcl"), Passport::hair_color_valid);
        map.insert(String::from("ecl"), Passport::eye_color_valid);
        map.insert(String::from("pid"), Passport::passport_id_valid);

        return map;
    }


    fn birth_year_valid(year: &String) -> bool {
        //byr (Birth Year) - four digits; at least 1920 and at most 2002.

        return match year.parse::<usize>() {
            Ok(y) => y >= 1920 && y <= 2002,
            Err(_) => false,
        }
    }

    fn issue_year_valid(year: &String) -> bool {
        //iyr (Issue Year) - four digits; at least 2010 and at most 2020.

        return match year.parse::<usize>() {
            Ok(y) => y >= 2010 && y <= 2020,
            Err(_) => false,
        }
    }

    fn expiration_year_valid(year: &String) -> bool {
        //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.

        return match year.parse::<usize>().map_err(InputError::Parse) {
            Ok(y) => y >= 2020 && y <= 2030,
            Err(_) => false,
        }
    }

    fn height_valid(height: &String) -> bool {
        //hgt (Height) - a number followed by either cm or in:
        //If cm, the number must be at least 150 and at most 193.
        //If in, the number must be at least 59 and at most 76.

        let height_parts = height.split_at(height.len() - 2);

        let h = match height_parts.0.parse::<usize>() {
            Ok(h) => h,
            Err(_) => return false,
        };

        return match height_parts.1 {
            "cm" => h >= 150 && h <= 193,
            "in" => h >= 59 && h <= 76,
            _ => false,
        };
    }

    fn hair_color_valid(hair_color: &String) -> bool {
        //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.

        return match regex::Regex::new(r"^#[0-9a-f]{6}$") {
            Ok(r) => r.is_match(&hair_color),
            Err(_) => false,
        };
    }

    fn eye_color_valid(eye_color: &String) -> bool {
        //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.

        return vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth").iter()
            .any(|c| c == &eye_color);
    }

    fn passport_id_valid(passport_id: &String) -> bool {
        //pid (Passport ID) - a nine-digit number, including leading zeroes.

        return match regex::Regex::new(r"^[0-9]{9}$") {
            Ok(r) => r.is_match(&passport_id),
            Err(_) => false,
        };
    }

    //cid (Country ID) - ignored, missing or not.
}
