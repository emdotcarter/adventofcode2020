fn main() -> Result<(), adventofcode2020::InputError> {
    let args: Vec<String> = std::env::args().collect();
    let day = args[1].as_str();

    let result = match day {
        "day1_part1" => run_day1_part1(&args[2..])?,
        "day1_part2" => run_day1_part2(&args[2..])?,
        _ => panic!("Bad day argument: {}", day),
    };

    print_result(result);

    Ok(())
}

fn print_result(result: std::collections::HashMap<&str, i64>) {
    println!("Result:");
    for (key, value) in result {
        println!("{}: {}", key, value);
    }
}

fn run_day1_part1(args: &[String]) -> Result<std::collections::HashMap<&str, i64>, adventofcode2020::InputError> {
    let mut expense_report_reader = adventofcode2020::buf_reader_from_filepath(&args[0])?;
    let expense_report_values = adventofcode2020::lines_to_i64(&mut expense_report_reader)?;

    let mut expense_report = adventofcode2020::ExpenseReport::new(&expense_report_values);

    let target_sum = 2020;
    return Ok(
        [("expense report values product", expense_report.product_from_target_two_sum(target_sum))]
        .iter()
        .cloned()
        .collect()
    );
}

fn run_day1_part2(args: &[String]) -> Result<std::collections::HashMap<&str, i64>, adventofcode2020::InputError> {
    let mut expense_report_reader = adventofcode2020::buf_reader_from_filepath(&args[0])?;
    let expense_report_values = adventofcode2020::lines_to_i64(&mut expense_report_reader)?;

    let mut expense_report = adventofcode2020::ExpenseReport::new(&expense_report_values);

    let target_sum = 2020;
    return Ok(
        [("expense report values product", expense_report.product_from_target_three_sum(target_sum))]
        .iter()
        .cloned()
        .collect()
    );
}