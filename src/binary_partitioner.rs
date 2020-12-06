use crate::InputError;

pub fn partition(input: &String) -> Result<std::collections::HashMap<String, usize>, InputError> {
    let mut row_range = [0, max_row_num()];
    let mut column_range = [0, max_column_num()];

    for instruction in input.chars() {
        let new_row_count = (row_range[1] - row_range[0] + 1) / 2;
        let new_column_count = (column_range[1] - column_range[0] + 1) / 2;

        match instruction {
            'F' => row_range = [row_range[0], row_range[1] - new_row_count],
            'B' => row_range = [row_range[0] + new_row_count, row_range[1]],
            'L' => column_range = [column_range[0], column_range[1] - new_column_count],
            'R' => column_range = [column_range[0] + new_column_count, column_range[1]],
            _ => return Err(InputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, "Bad input string"))),
        };
    }

    return Ok(
        [(String::from("row"), row_range[0]),
        (String::from("column"), column_range[0])]
        .iter()
        .cloned()
        .collect()
    );
}

fn max_row_num() -> usize {
    return 127;
}

fn max_column_num() -> usize {
    return 7;
}

#[cfg(test)]
mod tests {
    macro_rules! partitioner_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;

                    let result = super::partition(&String::from(input)).unwrap();
                    let actual = [result["row"], result["column"]];

                    assert_eq!(expected, actual);
                }
            )*
        }
    }

    partitioner_tests! {
        binary_partition_1: ("FBFBBFFRLR", [44, 5]),
        binary_partition_2: ("BFFFBBFRRR", [70, 7]),
        binary_partition_3: ("FFFBBBFRRR", [14, 7]),
        binary_partition_4: ("BBFFBBFRLL", [102, 4]),
    }
}
