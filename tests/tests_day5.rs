macro_rules! partitioner_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                let result = binary_partitioner::partition(&String::from(input)).unwrap();
                let actual = [result["row"], result["column"]];

                assert_eq!(expected, actual);
            }
        )*
    }
}

#[cfg(test)]
mod tests_day5 {
    use adventofcode2020::binary_partitioner;

    partitioner_tests! {
        bp_1: ("FBFBBFFRLR", [44, 5]),
        bp_2: ("BFFFBBFRRR", [70, 7]),
        bp_3: ("FFFBBBFRRR", [14, 7]),
        bp_4: ("BBFFBBFRLL", [102, 4]),
    }
}
