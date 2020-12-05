use adventofcode2020::InputError;

fn main() -> Result<(), InputError> {
    let args: Vec<String> = std::env::args().collect();
    let day = args[1].as_str();
    let part = args[2].as_str();

    let result = match part {
        "1" => (adventofcode2020::challenge_by_day(day)?)().run_part1(&args[3..])?,
        "2" => (adventofcode2020::challenge_by_day(day)?)().run_part2(&args[3..])?,
        _ => return Err(
            InputError::Io(
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad day/part values")
            )
        ),
    };

    adventofcode2020::print_hash_map(result);

    return Ok(());
}
