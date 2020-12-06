use adventofcode2020::InputError;

fn main() -> Result<(), InputError> {
    let args: Vec<String> = std::env::args().collect();
    let day = args[1].as_str();
    let part = args[2].as_str().parse::<usize>().map_err(InputError::Parse)?;

    let result = (adventofcode2020::challenge_by_day(day)?)().run_fn(part, &args[3..])?;
    adventofcode2020::print_hash_map(result);

    return Ok(());
}
