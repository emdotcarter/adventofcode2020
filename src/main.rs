fn main() -> Result<(), adventofcode2020::InputError> {
    let args: Vec<String> = std::env::args().collect();
    let day_identifier = args[1].as_str();

    let mut passthrough_args = Vec::new();
    let default_args = match day_default_filepath_map().get(day_identifier) {
        Some(s) => vec!(s.clone()),
        None => Vec::new(),
    };
    if args.len() >= 3 {
        for arg in args[2..].iter() {
            passthrough_args.push(arg.clone());
        }
    } else {
        passthrough_args = default_args;
    }

    let result = match day_function_map().get(day_identifier) {
        Some(f) => f(&passthrough_args[..])?,
        None => panic!("Bad day argument: {}", day_identifier),
    };

    print_result(result);

    return Ok(());
}

fn day_function_map() -> std::collections::HashMap<String, fn(&[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError>> {
    let mut map = std::collections::HashMap::<String, fn(&[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError>>::new();
    map.insert(String::from("day1_part1"), expense_report_product_from_2_sum);
    map.insert(String::from("day1_part2"), expense_report_product_from_3_sum);
    map.insert(String::from("day2_part1"), valid_passwords_by_charater_count);
    map.insert(String::from("day2_part2"), valid_passwords_by_character_position);
    map.insert(String::from("day3_part1"), trees_hit_on_single_traversal);
    map.insert(String::from("day3_part2"), trees_hit_on_multiple_traversals_product);
    map.insert(String::from("day4_part1"), valid_passports_without_validations);
    map.insert(String::from("day4_part2"), valid_passports_with_validations);

    return map;
}

fn day_default_filepath_map() -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    map.insert(String::from("day1_part1"), String::from("resources/expense_report.txt"));
    map.insert(String::from("day1_part2"), String::from("resources/expense_report.txt"));
    map.insert(String::from("day2_part1"), String::from("resources/password_database.txt"));
    map.insert(String::from("day2_part2"), String::from("resources/password_database.txt"));
    map.insert(String::from("day3_part1"), String::from("resources/slope_map.txt"));
    map.insert(String::from("day3_part2"), String::from("resources/slope_map.txt"));
    map.insert(String::from("day4_part1"), String::from("resources/passport_database.txt"));
    map.insert(String::from("day4_part2"), String::from("resources/passport_database.txt"));

    return map;
}


fn print_result(result: std::collections::HashMap<&str, usize>) {
    println!("Result:");
    for (key, value) in result {
        println!("  {}: {}", key, value);
    }
}

fn expense_report_product_from_2_sum(args: &[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError> {
    let raw_lines = adventofcode2020::file_lines_to_string_vec(&args[0])?;

    let mut expense_report = adventofcode2020::ExpenseReport::new(&raw_lines)?;

    let target_sum = 2020;
    return Ok(
        [("expense report values product", expense_report.product_from_target_two_sum(target_sum))]
        .iter()
        .cloned()
        .collect()
    );
}

fn expense_report_product_from_3_sum(args: &[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError> {
    let raw_lines = adventofcode2020::file_lines_to_string_vec(&args[0])?;

    let mut expense_report = adventofcode2020::ExpenseReport::new(&raw_lines)?;

    let target_sum = 2020;
    return Ok(
        [("expense report values product", expense_report.product_from_target_three_sum(target_sum))]
        .iter()
        .cloned()
        .collect()
    );
}

fn valid_passwords_by_charater_count(args: &[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError> {
    let raw_lines = adventofcode2020::file_lines_to_string_vec(&args[0])?;

    let mut password_database = adventofcode2020::PasswordDatabase::new(&raw_lines)?;

    return Ok(
        [("valid password count", password_database.valid_passwords_by_character_count())]
        .iter()
        .cloned()
        .collect()
    );
}

fn valid_passwords_by_character_position(args: &[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError> {
    let raw_lines = adventofcode2020::file_lines_to_string_vec(&args[0])?;

    let mut password_database = adventofcode2020::PasswordDatabase::new(&raw_lines)?;

    return Ok(
        [("valid password count", password_database.valid_passwords_by_character_position())]
        .iter()
        .cloned()
        .collect()
    );
}

fn trees_hit_on_single_traversal(args: &[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError> {
    let raw_lines = adventofcode2020::file_lines_to_string_vec(&args[0])?;

    let slope_map = adventofcode2020::SlopeMap::new(&raw_lines);

    let movement_path = adventofcode2020::MovementPath::new(3, 1);
    return Ok(
        [("trees hit", slope_map.count_trees_on_traversal(&movement_path))]
        .iter()
        .cloned()
        .collect()
    );
}

fn trees_hit_on_multiple_traversals_product(args: &[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError> {
    let raw_lines = adventofcode2020::file_lines_to_string_vec(&args[0])?;

    let slope_map = adventofcode2020::SlopeMap::new(&raw_lines);
    let movement_paths = vec!(
        adventofcode2020::MovementPath::new(1, 1),
        adventofcode2020::MovementPath::new(3, 1),
        adventofcode2020::MovementPath::new(5, 1),
        adventofcode2020::MovementPath::new(7, 1),
        adventofcode2020::MovementPath::new(1, 2),
    );
    let trees_hit_product = movement_paths.iter()
        .map(|p| slope_map.count_trees_on_traversal(&p))
        .fold(1, |acc, x| acc * x)
        ;

    return Ok(
        [("trees hit", trees_hit_product)]
        .iter()
        .cloned()
        .collect()
    );
}

fn valid_passports_without_validations(args: &[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError> {
    let raw_lines = adventofcode2020::file_lines_to_string_vec(&args[0])?;

    let mut passports: Vec<adventofcode2020::Passport> = Vec::new();
    let mut key_value_row: String = String::from("");
    for line in raw_lines {
        if line == "" {
            passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
            key_value_row = String::from("");
        } else {
            key_value_row.push_str(" ");
            key_value_row.push_str(&line);
        }
    }

    if key_value_row != "" {
        passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
    }

    return Ok(
        [("valid passports", passports.iter().filter(|p| p.is_valid(false)).count())]
        .iter()
        .cloned()
        .collect()
    );
}

fn valid_passports_with_validations(args: &[String]) -> Result<std::collections::HashMap<&str, usize>, adventofcode2020::InputError> {
    let raw_lines = adventofcode2020::file_lines_to_string_vec(&args[0])?;

    let mut passports: Vec<adventofcode2020::Passport> = Vec::new();
    let mut key_value_row: String = String::from("");
    for line in raw_lines {
        if line == "" {
            passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
            key_value_row = String::from("");
        } else {
            key_value_row.push_str(" ");
            key_value_row.push_str(&line);
        }
    }

    if key_value_row != "" {
        passports.push(adventofcode2020::Passport::new(&key_value_row.trim())?);
    }

    return Ok(
        [("valid passports", passports.iter().filter(|p| p.is_valid(true)).count())]
        .iter()
        .cloned()
        .collect()
    );
}
