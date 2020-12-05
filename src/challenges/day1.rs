use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;
use crate::ExpenseReport;

pub fn challenge() -> Challenge {
    return Challenge::new(
        expense_report_product_from_2_sum,
        expense_report_product_from_3_sum,
        String::from("resources/expense_report.txt"),
    );
}

pub fn expense_report_product_from_2_sum(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut expense_report = ExpenseReport::new(&raw_lines)?;

    let target_sum = 2020;
    return Ok(
        [(String::from("expense report values product"), expense_report.product_from_target_two_sum(target_sum))]
        .iter()
        .cloned()
        .collect()
    );
}

fn expense_report_product_from_3_sum(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut expense_report = ExpenseReport::new(&raw_lines)?;

    let target_sum = 2020;
    return Ok(
        [(String::from("expense report values product"), expense_report.product_from_target_three_sum(target_sum))]
        .iter()
        .cloned()
        .collect()
    );
}
