#[cfg(test)]
mod tests_day1 {
    #[test]
    fn target_sum_two_numbers() {
        let values = vec!(1721, 979, 366, 299, 675, 1456);
        let mut expense_report = adventofcode2020::ExpenseReport::new(&values);

        let target_sum = 2020;
        let actual = expense_report.product_from_target_two_sum(target_sum);

        assert_eq!(514579, actual);
    }

    #[test]
    fn target_sum_three_numbers() {
        let values = vec!(1721, 979, 366, 299, 675, 1456);
        let mut expense_report = adventofcode2020::ExpenseReport::new(&values);

        let target_sum = 2020;
        let actual = expense_report.product_from_target_three_sum(target_sum);

        assert_eq!(241861950, actual);
    }
}
