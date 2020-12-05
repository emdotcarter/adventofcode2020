use crate::input_error::InputError;

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
