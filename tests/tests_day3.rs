#[cfg(test)]
mod tests_day3 {
    #[test]
    fn trees_hit_traversing() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
	    String::from("..##......."),
	    String::from("#...#...#.."),
	    String::from(".#....#..#."),
	    String::from("..#.#...#.#"),
	    String::from(".#...##..#."),
	    String::from("..#.##....."),
	    String::from(".#.#.#....#"),
	    String::from(".#........#"),
	    String::from("#.##...#..."),
	    String::from("#...##....#"),
	    String::from(".#..#...#.#"),
	);
        let movement_path = adventofcode2020::MovementPath::new(3, 1);

	let slope_map = adventofcode2020::SlopeMap::new(&values);
	let actual = slope_map.count_trees_on_traversal(&movement_path);

        assert_eq!(7, actual);

        return Ok(());
    }

    #[test]
    fn product_of_trees_hit_traversing() -> Result<(), adventofcode2020::InputError> {
        let values = vec!(
	    String::from("..##......."),
	    String::from("#...#...#.."),
	    String::from(".#....#..#."),
	    String::from("..#.#...#.#"),
	    String::from(".#...##..#."),
	    String::from("..#.##....."),
	    String::from(".#.#.#....#"),
	    String::from(".#........#"),
	    String::from("#.##...#..."),
	    String::from("#...##....#"),
	    String::from(".#..#...#.#"),
	);
        let movement_paths = vec!(
            adventofcode2020::MovementPath::new(1, 1),
            adventofcode2020::MovementPath::new(3, 1),
            adventofcode2020::MovementPath::new(5, 1),
            adventofcode2020::MovementPath::new(7, 1),
            adventofcode2020::MovementPath::new(1, 2),
        );

	let slope_map = adventofcode2020::SlopeMap::new(&values);
        let actual = movement_paths.iter()
            .map(|p| slope_map.count_trees_on_traversal(&p))
            .fold(1, |acc, x| acc * x)
            ;

        assert_eq!(336, actual);

        return Ok(());
    }
}
