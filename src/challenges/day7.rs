use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;
use crate::DirectedGraph;

pub fn challenge() -> Challenge {
    return Challenge::new(
        count_outermost_bags,
        count_contained_bags,
        String::from("resources/luggage_rules.txt"),
    );
}

fn count_outermost_bags(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let graph = build_graph_from_input(&raw_lines);

    let mut outermost_bags = 0;
    for start in graph.get_node_names().iter() {
        if *start == "shiny gold" {
            continue;
        }

        for n in graph.iter_from_node(start) {
            if n.0.get_name() == "shiny gold" {
                outermost_bags += 1;
                break;
            }
        }
    }

    return Ok(
        [(String::from("outermost bags"), outermost_bags)]
        .iter()
        .cloned()
        .collect()
    );
}

fn count_contained_bags(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let graph = build_graph_from_input(&raw_lines);

    let start = "shiny gold";
    let contained_bags = graph.sum_of_path_weight_products(start);

    return Ok(
        [(String::from("contained bags"), contained_bags as usize)]
        .iter()
        .cloned()
        .collect()
    );
}

fn build_graph_from_input(input: &Vec<String>) -> DirectedGraph {
    let mut graph = DirectedGraph::new();
    for line in input {
        let parsed_line = parse_input_line(&line);

        graph.add_node_if_not_exists(parsed_line.0.clone());
        for edge in parsed_line.1 {
            graph.add_node_if_not_exists(edge.0.clone());
            graph.add_edge_by_names(&parsed_line.0, &edge.0, edge.1);
        }
    }

    return graph;
}

fn parse_input_line(line: &str) -> (String, Vec<(String, i64)>) {
    let from_re = regex::Regex::new(r"^([\w\s]+?) bag").unwrap();
    let from_node = from_re.captures(line).unwrap().get(1).unwrap().as_str();

    let to_re = regex::Regex::new(r"(?:contain|,) (\d+ [\w\s]+?) bag").unwrap();
    let mut to_nodes = Vec::new();
    for m in to_re.captures_iter(line) {
        let strs: Vec<&str> = m.get(1).unwrap().as_str().splitn(2, " ").collect();
        let weight = strs[0].parse::<i64>().unwrap();

        to_nodes.push((strs[1].to_string(), weight));
    }


    return (from_node.to_string(), to_nodes);
}

#[cfg(test)]
mod tests {
    use super::challenge;

    crate::challenge_tests!(192, 12128);

    #[test]
    fn test_count_outermost_bags() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let file_lines = [
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        for line in &file_lines {
            writeln!(file, "{}", line).unwrap();
        }
        let args = vec!(String::from(file.path().to_str().unwrap()));
        let actual =
            super::count_outermost_bags(&args).unwrap()["outermost bags"];

        assert_eq!(4, actual);
    }

    #[test]
    fn test_count_contained_bags_1() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let file_lines = [
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        for line in &file_lines {
            writeln!(file, "{}", line).unwrap();
        }
        let args = vec!(String::from(file.path().to_str().unwrap()));
        let actual =
            super::count_contained_bags(&args).unwrap()["contained bags"];

        assert_eq!(32, actual);
    }

    #[test]
    fn test_count_contained_bags_2() {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let file_lines = [
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];

        for line in &file_lines {
            writeln!(file, "{}", line).unwrap();
        }
        let args = vec!(String::from(file.path().to_str().unwrap()));
        let actual =
            super::count_contained_bags(&args).unwrap()["contained bags"];

        assert_eq!(126, actual);
    }
}
