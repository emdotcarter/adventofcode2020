use crate::challenges::Challenge;
use crate::InputError;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;
use crate::binary_partitioner;

pub fn challenge() -> Challenge {
    return Challenge::new(
        max_seat_id,
        my_seat_id,
        String::from("resources/binary_partitioning_seats.txt"),
    );
}

fn max_seat_id(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let max_seat_id = raw_lines.iter()
        .map(|l| binary_partitioner::partition(&l).unwrap())
        .map(|hm| hm["row"] * 8 + hm["column"])
        .max()
        ;

    return match max_seat_id {
        Some(id) => Ok(
            [(String::from("max seat id"), id)]
            .iter()
            .cloned()
            .collect()
        ),
        None => Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad input"))),
    }
}

fn my_seat_id(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let wrapped_seat_ids = raw_lines.iter()
        .map(|l| binary_partitioner::partition(&l).unwrap())
        .map(|hm| hm["row"] * 8 + hm["column"])
        ;

    let mut sorted_seat_ids = Vec::new();
    for seat_id in wrapped_seat_ids {
        sorted_seat_ids.push(seat_id);
    }
    sorted_seat_ids.sort();

    let mut my_seat_id = 0;
    for i in 0..sorted_seat_ids.len() {
        if sorted_seat_ids[i + 1] - sorted_seat_ids[i] == 2 {
            my_seat_id = sorted_seat_ids[i] + 1;
            break;
        }
    }

    return Ok(
        [(String::from("max seat id"), my_seat_id)]
        .iter()
        .cloned()
        .collect()
    )
}
