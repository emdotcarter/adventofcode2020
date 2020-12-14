use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;
use crate::BootCodeComputer;

pub fn challenge() -> Challenge {
    return Challenge::new(
        accumulator_value_before_first_loop,
        accumulator_value_after_swap_completion,
        String::from("resources/boot_code.txt"),
    );
}

fn accumulator_value_before_first_loop(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut computer = BootCodeComputer::new(&raw_lines);
    computer.run_until_first_loop();

    return Ok(
        [(String::from("accumulator value"), computer.get_accumulator_value() as usize)]
        .iter()
        .cloned()
        .collect()
    );
}

fn accumulator_value_after_swap_completion(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut computer = BootCodeComputer::new(&raw_lines);
    computer.run_until_complete_with_instruction_swaps();

    return Ok(
        [(String::from("accumulator value"), computer.get_accumulator_value() as usize)]
        .iter()
        .cloned()
        .collect()
    );
}

#[cfg(test)]
mod tests {
    use super::challenge;

    crate::challenge_tests!(2051, 2304);
}
