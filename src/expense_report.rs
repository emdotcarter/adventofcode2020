use crate::input_error::InputError;

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
