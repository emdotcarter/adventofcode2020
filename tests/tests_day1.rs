#[cfg(test)]
mod tests_day1 {
    #[test]
    fn target_sum_two_numbers() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
            String::from("1721"),
            String::from("979"),
            String::from("366"),
            String::from("299"),
            String::from("675"),
            String::from("1456"),
        );
        let mut expense_report = adventofcode2020::ExpenseReport::new(&values)?;

        let target_sum = 2020;
        let actual = expense_report.product_from_target_two_sum(target_sum);

        assert_eq!(514579, actual);

        return Ok(());
    }

    #[test]
    fn target_sum_three_numbers() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
            String::from("1721"),
            String::from("979"),
            String::from("366"),
            String::from("299"),
            String::from("675"),
            String::from("1456"),
        );
        let mut expense_report = adventofcode2020::ExpenseReport::new(&values)?;

        let target_sum = 2020;
        let actual = expense_report.product_from_target_three_sum(target_sum);

        assert_eq!(241861950, actual);

        return Ok(());
    }
}
