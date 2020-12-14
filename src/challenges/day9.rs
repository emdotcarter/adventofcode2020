use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;

pub fn challenge() -> Challenge {
    return Challenge::new(
        first_number_not_sum_of_previous_2,
        sum_from_contiguous_numbers,
        String::from("resources/xmas_values.txt"),
    );
}

fn first_number_not_sum_of_previous_2(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;
    let values: Vec<usize> = raw_lines.iter().map(|l| l.parse().unwrap()).collect();

    let preamble_size = 25;

    let mut current_index = preamble_size;
    while current_index < values.len() {
        let candidate_values = &values[current_index - preamble_size..current_index];
        if !values_summing_to_target_exist(candidate_values, values[current_index]) {
            break;
        }

        current_index += 1;
    }

    return Ok(
        [(String::from("target number"), values[current_index])]
        .iter()
        .cloned()
        .collect()
    );
}

fn sum_from_contiguous_numbers(args: &[String]) -> ResultHashMap {
    use itertools::Itertools;

    let raw_lines = file_lines_to_string_vec(&args[0])?;
    let values: Vec<usize> = raw_lines.iter().map(|l| l.parse().unwrap()).collect();

    let target_value = 756008079;

    let mut target_sum = 0;
    for v in values.iter().enumerate() {
        let mut i = v.0 + 1;
        let mut summed_values = vec!(v.1);
        while summed_values.iter().fold(0, |acc, val| acc + *val) < target_value && i < values.len() {
            summed_values.push(&values[i]);
            i += 1;
        }

        if summed_values.iter().fold(0, |acc, val| acc + *val) == target_value {
            let min_and_max = summed_values.iter().minmax().into_option().unwrap();
            target_sum = *min_and_max.0 + *min_and_max.1;
            break;
        }
    }

    return Ok(
        [(String::from("target number"), target_sum)]
        .iter()
        .cloned()
        .collect()
    );
}

fn values_summing_to_target_exist(values: &[usize], target: usize) -> bool {
    use itertools::Itertools;

    let combinations = values.iter().cartesian_product(values.iter());

    for c in combinations {
        if c.0 + c.1 == target && c.0 != c.1 {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::challenge;

    crate::challenge_tests!(756008079, 93727241);
}
