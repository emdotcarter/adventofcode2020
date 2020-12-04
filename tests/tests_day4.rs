#[cfg(test)]
mod tests_day4 {
    #[test]
    fn count_valid_passports() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
            String::from(""),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
        );

        let mut passports: Vec<adventofcode2020::Passport> = Vec::new();
        let mut key_value_row: String = String::from("");
        for i in 0..values.len() {
            let line = &values[i];

            if line == "" {
                passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
                key_value_row = String::from("");
            } else {
                key_value_row.push_str(" ");
                key_value_row.push_str(line);
            }
        }

        if key_value_row != "" {
            passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
        }

        let actual = passports.iter().filter(|p| p.is_valid(false)).count();

        assert_eq!(2, actual);

        return Ok(());
    }

    #[test]
    fn count_invalid_passports_with_validators() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
            String::from("eyr:1972 cid:100"),
            String::from("hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
            String::from(""),
            String::from("iyr:2019"),
            String::from("hcl:#602927 eyr:1967 hgt:170cm"),
            String::from("ecl:grn pid:012533040 byr:1946"),
            String::from(""),
            String::from("hcl:dab227 iyr:2012"),
            String::from("ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
            String::from(""),
            String::from("hgt:59cm ecl:zzz"),
            String::from("eyr:2038 hcl:74454a iyr:2023"),
            String::from("pid:3556412378 byr:2007"),
        );

        let mut passports: Vec<adventofcode2020::Passport> = Vec::new();
        let mut key_value_row: String = String::from("");
        for i in 0..values.len() {
            let line = &values[i];

            if line == "" {
                passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
                key_value_row = String::from("");
            } else {
                key_value_row.push_str(" ");
                key_value_row.push_str(line);
            }
        }

        if key_value_row != "" {
            passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
        }

        let actual = passports.iter().filter(|p| p.is_valid(true)).count();

        assert_eq!(0, actual);

        return Ok(());
    }

    #[test]
    fn count_valid_passports_with_validators() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
            String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"),
            String::from("hcl:#623a2f"),
            String::from(""),
            String::from("eyr:2029 ecl:blu cid:129 byr:1989"),
            String::from("iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
            String::from(""),
            String::from("hcl:#888785"),
            String::from("hgt:164cm byr:2001 iyr:2015 cid:88"),
            String::from("pid:545766238 ecl:hzl"),
            String::from("eyr:2022"),
            String::from(""),
            String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
        );

        let mut passports: Vec<adventofcode2020::Passport> = Vec::new();
        let mut key_value_row: String = String::from("");
        for i in 0..values.len() {
            let line = &values[i];

            if line == "" {
                passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
                key_value_row = String::from("");
            } else {
                key_value_row.push_str(" ");
                key_value_row.push_str(line);
            }
        }

        if key_value_row != "" {
            passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
        }

        let actual = passports.iter().filter(|p| p.is_valid(true)).count();

        assert_eq!(4, actual);

        return Ok(());
    }
}
