use crate::challenges::Challenge;
use crate::ResultHashMap;
use crate::file_lines_to_string_vec;
use crate::MovementPath;
use crate::SlopeMap;

pub fn challenge() -> Challenge {
    return Challenge::new(
        trees_hit_on_single_traversal,
        trees_hit_on_multiple_traversals_product,
        String::from("resources/slope_map.txt"),
    );
}

fn trees_hit_on_single_traversal(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let slope_map = SlopeMap::new(&raw_lines);

    let movement_path = MovementPath::new(3, 1);
    return Ok(
        [(String::from("trees hit"), slope_map.count_trees_on_traversal(&movement_path))]
        .iter()
        .cloned()
        .collect()
    );
}

fn trees_hit_on_multiple_traversals_product(args: &[String]) -> ResultHashMap {
    let raw_lines = file_lines_to_string_vec(&args[0])?;

    let slope_map = SlopeMap::new(&raw_lines);
    let movement_paths = vec!(
        MovementPath::new(1, 1),
        MovementPath::new(3, 1),
        MovementPath::new(5, 1),
        MovementPath::new(7, 1),
        MovementPath::new(1, 2),
    );
    let trees_hit_product = movement_paths.iter()
        .map(|p| slope_map.count_trees_on_traversal(&p))
        .fold(1, |acc, x| acc * x)
        ;

    return Ok(
        [(String::from("trees hit"), trees_hit_product)]
        .iter()
        .cloned()
        .collect()
    );
}
