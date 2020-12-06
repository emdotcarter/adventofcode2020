pub struct MovementPath {
    horizontal: i64,
    downward: i64,
}

impl MovementPath {
    pub fn new(horizontal: i64, downward: i64) -> MovementPath {
        return MovementPath {
            horizontal: horizontal,
            downward: downward,
        };
    }
}

pub struct SlopeMap {
    map: Vec<Vec<char>>,
}

impl SlopeMap {
    pub fn new(map_rows: &Vec<String>) -> SlopeMap {
        let mut map: Vec<Vec<char>> = Vec::new();

        for map_row in map_rows {
            let mut row = Vec::new();

            for c in map_row.chars() {
                row.push(c);
            }

            map.push(row);
        }

        return SlopeMap {map: map};
    }

    pub fn count_trees_on_traversal(&self, movement_path: &MovementPath) -> usize {
        let mut trees_hit = 0;
        let mut index: (i64, i64) = (0, 0);

        while index.1 < self.map.len() as i64 {
            let current_row = &self.map[index.1 as usize];

            if current_row[index.0 as usize] == SlopeMap::tree_char() {
                trees_hit += 1;
            }

            let mut new_x = index.0 + movement_path.horizontal;
            if new_x as usize >= current_row.len() {
                new_x -= current_row.len() as i64;
            }
            index = (new_x, index.1 + movement_path.downward);
        }

        return trees_hit;
    }

    #[inline]
    fn tree_char() -> char {
        return '#';
    }
}

#[cfg(test)]
mod tests {
    use crate::InputError;
    use super::MovementPath;
    use super::SlopeMap;

    #[test]
    fn count_trees_on_traversal() -> Result<(), InputError> {
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
        let movement_path = MovementPath::new(3, 1);

	let slope_map = SlopeMap::new(&values);
	let actual = slope_map.count_trees_on_traversal(&movement_path);

        assert_eq!(7, actual);

        return Ok(());
    }

    #[test]
    fn count_trees_on_multiple_traversals() -> Result<(), InputError> {
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
            MovementPath::new(1, 1),
            MovementPath::new(3, 1),
            MovementPath::new(5, 1),
            MovementPath::new(7, 1),
            MovementPath::new(1, 2),
        );

	let slope_map = SlopeMap::new(&values);
        let actual = movement_paths.iter()
            .map(|p| slope_map.count_trees_on_traversal(&p))
            .fold(1, |acc, x| acc * x);

        assert_eq!(336, actual);

        return Ok(());
    }
}
