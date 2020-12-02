use std::io::BufRead;

#[derive(Debug)]
pub enum InputError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

pub fn buf_reader_from_filepath(filepath: &String) -> Result<std::io::BufReader<std::fs::File>, InputError> {
    return Ok(
        std::io::BufReader::new(
            std::fs::File::open(filepath)
            .map_err(InputError::Io)?
        )
    );
}

pub fn lines_to_i64(reader: &mut std::io::BufReader<std::fs::File>) -> Result<Vec<i64>, InputError> {
    let mut numbers = Vec::<i64>::new();
    for line in reader.lines() {
        numbers.push(line.map_err(InputError::Io)?.parse::<i64>().map_err(InputError::Parse)?);
    }

    return Ok(numbers);
}

pub struct ExpenseReport {
    values: Vec<i64>,
}

impl ExpenseReport {
    pub fn new(values: &Vec<i64>) -> ExpenseReport {
        return ExpenseReport {
            values: values.to_vec(),
        };
    }

    pub fn product_from_target_two_sum(&mut self, target_sum: i64) -> i64 {
        self.values.retain(|&v| v <= target_sum);
        for value1 in &self.values {
            for value2 in (&self.values).into_iter().filter(|&v| value1 + v == target_sum).collect::<Vec<&i64>>() {
                return value1 * value2;
            }
        }

        return -1;
    }

    pub fn product_from_target_three_sum(&mut self, target_sum: i64) -> i64 {
        self.values.retain(|&v| v <= target_sum);
        for value1 in &self.values {
            for value2 in (&self.values).into_iter().filter(|&v| value1 + v <= target_sum).collect::<Vec<&i64>>() {
                for value3 in (&self.values).into_iter().filter(|&v| value1 + value2 + v == target_sum).collect::<Vec<&i64>>() {
                    return value1 * value2 * value3;
                }

            }
        }

        return -1;
    }
}
