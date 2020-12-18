use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;
use crate::n_ary_tree;

pub fn challenge() -> Challenge {
    return Challenge::new(
        product_of_sorted_value_differences,
        count_valid_joltage_paths,
        String::from("resources/joltage_ratings.txt"),
    );
}

fn product_of_sorted_value_differences(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut values: Vec<usize> = raw_lines.iter()
        .map(|l| l.parse().unwrap())
        .collect();
    values.sort();
    values.insert(0, 0);
    values.push(values.last().unwrap() + 3);

    let mut one_diffs = 0;
    let mut three_diffs = 0;
    for i in 1..values.len() {
        match values[i] - values[i - 1] {
            1 => one_diffs += 1,
            3 => three_diffs += 1,
            _ => {},
        }
    }

    return Ok(
        [(String::from("product of joltage differences"), one_diffs * three_diffs)]
        .iter()
        .cloned()
        .collect()
    );
}

fn count_valid_joltage_paths(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut values: Vec<usize> = raw_lines.iter()
        .map(|l| l.parse().unwrap())
        .collect();
    values.sort();
    values.insert(0, 0);
    values.push(values.last().unwrap() + 3);

    let count = count_valid_paths(&values);

    return Ok(
        [(String::from("valid joltage paths"), count)]
        .iter()
        .cloned()
        .collect()
    );
}

//fn count_valid_paths(joltage_adapters: &Vec<usize>, current_index: usize, mut total_count: usize) -> usize {
//let current_adapter_joltage = joltage_adapters[current_index];
//let valid_children_indices: Vec<usize> = joltage_adapters.iter().enumerate()
//.filter(|e| e.1 - current_adapter_joltage <= 3)
//.map(|e| e.0)
//.collect();

//if valid_children_indices.len() == 0 {
//return 1;
//} else {
//for c in valid_children_indices {
//total_count += count_valid_paths(joltage_adapters, c, total_count);
//}

//return total_count;
//}
//}

//fn count_valid_paths(joltage_adapters: &Vec<usize>) -> usize {
    //let mut total_count = 0;

    //let current_joltage: usize = joltage_adapters[0];
    //let mut joltage_stack = vec!(current_joltage);
    //while !joltage_stack.is_empty() {
        //println!("cj: {}", current_joltage);

        //let valid_children: Vec<&usize> = joltage_adapters.iter()
            //.filter(|j| **j <= current_joltage + 3)
            //.filter(|j| !used_joltages.contains(j))
            //.collect();

        //if valid_children.is_empty() {
            //total_count += 1;
        //} else {
            //for s in joltage_stack.iter() {
                //println!("pre s: {}", s);
            //}

            //for c in valid_children {
                //joltage_stack.push(*c);
            //}

            //for s in joltage_stack.iter() {
                //println!("post s: {}", s);
            //}
            //println!("\n");
        //}
    //}

    //return total_count;
//}

#[cfg(test)]
mod tests {
    use super::challenge;

    //crate::challenge_tests!(1820, 93727241);

    #[test]
    fn test_product_of_sorted_value_differences_1() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let file_lines = [
            "16",
            "10",
            "15",
            "5",
            "1",
            "11",
            "7",
            "19",
            "6",
            "12",
            "4",
        ];

        for line in &file_lines {
            writeln!(file, "{}", line).unwrap();
        }
        let args = vec!(String::from(file.path().to_str().unwrap()));

        let actual =
            super::product_of_sorted_value_differences(&args).unwrap()["product of joltage differences"];

        assert_eq!(35, actual);
    }

    #[test]
    fn test_product_of_sorted_value_differences_2() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let file_lines = [
            "28",
            "33",
            "18",
            "42",
            "31",
            "14",
            "46",
            "20",
            "48",
            "47",
            "24",
            "23",
            "49",
            "45",
            "19",
            "38",
            "39",
            "11",
            "1",
            "32",
            "25",
            "35",
            "8",
            "17",
            "7",
            "9",
            "4",
            "2",
            "34",
            "10",
            "3",
            ];

        for line in &file_lines {
            writeln!(file, "{}", line).unwrap();
        }
        let args = vec!(String::from(file.path().to_str().unwrap()));

        let actual =
            super::product_of_sorted_value_differences(&args).unwrap()["product of joltage differences"];

        assert_eq!(220, actual);
    }

    #[test]
    fn test_count_valid_joltage_paths_1() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let file_lines = [
            "16",
            "10",
            "15",
            "5",
            "1",
            "11",
            "7",
            "19",
            "6",
            "12",
            "4",
        ];

        for line in &file_lines {
            writeln!(file, "{}", line).unwrap();
        }
        let args = vec!(String::from(file.path().to_str().unwrap()));

        let actual =
            super::count_valid_joltage_paths(&args).unwrap()["product of joltage differences"];

        assert_eq!(8, actual);
    }

    //#[test]
    //fn test_count_valid_joltage_paths_2() {
        //use std::io::Write;

        //let mut file = tempfile::NamedTempFile::new().unwrap();

        //let file_lines = [
            //"28",
            //"33",
            //"18",
            //"42",
            //"31",
            //"14",
            //"46",
            //"20",
            //"48",
            //"47",
            //"24",
            //"23",
            //"49",
            //"45",
            //"19",
            //"38",
            //"39",
            //"11",
            //"1",
            //"32",
            //"25",
            //"35",
            //"8",
            //"17",
            //"7",
            //"9",
            //"4",
            //"2",
            //"34",
            //"10",
            //"3",
            //];

        //for line in &file_lines {
            //writeln!(file, "{}", line).unwrap();
        //}
        //let args = vec!(String::from(file.path().to_str().unwrap()));

        //let actual =
            //super::count_valid_joltage_paths(&args).unwrap()["valid joltage paths"];

        //assert_eq!(19208, actual);
    //}
}
