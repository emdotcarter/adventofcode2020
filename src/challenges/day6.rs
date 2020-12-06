use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;

pub fn challenge() -> Challenge {
    return Challenge::new(
        sum_of_any_group_answered_questions,
        sum_of_all_group_answered_questions,
        String::from("resources/customs_answers.txt"),
    );
}

fn sum_of_any_group_answered_questions(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut group_answered_questions = 0;
    let mut group_answers: String = String::from("");
    for line in raw_lines {
        if line == "" {
            group_answered_questions += string_to_char_set(&group_answers).len();

            group_answers = String::from("");
        } else {
            group_answers.push_str(line.trim());
        }
    }

    if group_answers != "" {
        group_answered_questions += string_to_char_set(&group_answers).len();
    }

    return Ok(
        [(String::from("total group-answered questions"), group_answered_questions)]
        .iter()
        .cloned()
        .collect()
    );
}

fn sum_of_all_group_answered_questions(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let mut group_answered_questions = 0;
    let mut group_answer_sets: Vec<std::collections::HashSet<char>> = Vec::new();
    for line in raw_lines {
        if line == "" {
            group_answered_questions += intersection_of_sets(&group_answer_sets).len();

            group_answer_sets = Vec::new();
        } else {
            group_answer_sets.push(string_to_char_set(&line));
        }
    }

    if group_answer_sets.len() > 0 {
        group_answered_questions += intersection_of_sets(&group_answer_sets).len();
    }

    return Ok(
        [(String::from("total group-answered questions"), group_answered_questions)]
        .iter()
        .cloned()
        .collect()
    );
}

fn string_to_char_set(s: &String) -> std::collections::HashSet<char> {
    let mut set = std::collections::HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }

    return set;
}

fn intersection_of_sets<T>(sets: &Vec<std::collections::HashSet<T>>) -> std::collections::HashSet<T> where T: Eq, T: std::hash::Hash, T: Clone {
    let mut intersection: std::collections::HashSet<T> = sets[0].clone();
    for item in &sets[0] {
        let mut found_in_set = sets[1..].iter()
            .map(|s| s.contains(&item));

        if found_in_set.any(|found| !found) {
            intersection.remove(&item);
        }
    }

    return intersection;
}

#[cfg(test)]
mod tests {
    use super::challenge;

    crate::challenge_tests!(6726, 3316);

    #[test]
    fn sum_of_any_group_answered_questions() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(file, "abc").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "b").unwrap();
        writeln!(file, "c").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "ab").unwrap();
        writeln!(file, "ac").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "b").unwrap();

        let args = vec!(String::from(file.path().to_str().unwrap()));
        let actual =
            super::sum_of_any_group_answered_questions(&args)
            .unwrap()
            ["total group-answered questions"];

        assert_eq!(11, actual);
    }

    #[test]
    fn sum_of_all_group_answered_questions() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(file, "abc").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "b").unwrap();
        writeln!(file, "c").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "ab").unwrap();
        writeln!(file, "ac").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "a").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "b").unwrap();

        let args = vec!(String::from(file.path().to_str().unwrap()));
        let actual =
            super::sum_of_all_group_answered_questions(&args)
            .unwrap()
            ["total group-answered questions"];

        assert_eq!(6, actual);
    }
}
