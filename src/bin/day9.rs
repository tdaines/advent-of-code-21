use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
struct Location {
    row: usize,
    col: usize,
    height: u8,
}

impl Location {
    fn new(row: usize, col: usize, height: u8) -> Self {
        Self { row, col, height }
    }
}

fn main() {
    let input_file = File::open("./data/day9.txt").unwrap();
    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let height_map = build_height_map(lines);
    let low_points = find_low_locations(&height_map);
    let risk_level_sum = sum_risk_levels_of_low_points(&low_points);

    println!(
        "The sum of the risk levels of all the low points in the height map is {}",
        risk_level_sum
    );

    let basin_sizes = find_basin_sizes(&height_map);
    let total: i32 = basin_sizes
        .iter()
        .sorted()
        .rev()
        .take(3)
        .map(|x| *x as i32)
        .product();

    println!(
        "The total of the three larges basin sizes multiplied together is {}",
        total
    );
}

fn build_height_map(lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut height_map = Vec::new();

    for line in lines {
        height_map.push(read_height_map_line(&line));
    }

    height_map
}

fn read_height_map_line(line: &str) -> Vec<u8> {
    // parse "2199943210"
    line.chars()
        .map(|chr| chr.to_digit(10).unwrap() as u8)
        .collect()
}

fn get_adjacent_locations(height_map: &[Vec<u8>], location: Location) -> Vec<Location> {
    let mut adjacent = Vec::with_capacity(4);
    let row = location.row;
    let col = location.col;

    //above
    if row > 0 {
        adjacent.push(Location::new(row - 1, col, height_map[row - 1][col]));
    }

    //below
    if row < height_map.len() - 1 {
        adjacent.push(Location::new(row + 1, col, height_map[row + 1][col]));
    }

    //left
    if col > 0 {
        adjacent.push(Location::new(row, col - 1, height_map[row][col - 1]));
    }

    //right
    if col < height_map[0].len() - 1 {
        adjacent.push(Location::new(row, col + 1, height_map[row][col + 1]));
    }

    adjacent
}

fn is_low_point(height_map: &[Vec<u8>], row: usize, col: usize) -> bool {
    let height = height_map[row][col];

    // 9's can't be a low point
    if height == 9 {
        return false;
    }

    // 0's are low points
    if height == 0 {
        return true;
    }

    let adjacent_locations = get_adjacent_locations(height_map, Location::new(row, col, height));
    height
        < adjacent_locations
            .iter()
            .map(|loc| loc.height)
            .min()
            .unwrap()
}

fn find_low_locations(height_map: &[Vec<u8>]) -> Vec<Location> {
    let mut low_points = Vec::new();

    for row in 0..height_map.len() {
        for col in 0..height_map[row].len() {
            if is_low_point(height_map, row, col) {
                low_points.push(Location::new(row, col, height_map[row][col]));
            }
        }
    }

    low_points
}

fn sum_risk_levels_of_low_points(low_points: &[Location]) -> usize {
    low_points
        .iter()
        .map(|low_point| low_point.height as usize + 1)
        .sum()
}

fn find_basin_sizes(height_map: &[Vec<u8>]) -> Vec<usize> {
    let low_points = find_low_locations(height_map);
    let mut basin_sizes = Vec::new();

    for low_point in low_points {
        let mut basin = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back(low_point);

        while !to_visit.is_empty() {
            let current_location = to_visit.pop_front().unwrap();
            if !basin.contains(&current_location) {
                basin.insert(current_location);

                let neighbors = get_adjacent_locations(height_map, current_location);
                for neighbor in neighbors {
                    if neighbor.height < 9 && !basin.contains(&neighbor) {
                        to_visit.push_back(neighbor);
                    }
                }
            }
        }

        basin_sizes.push(basin.len());
    }

    basin_sizes
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn test_read_height_map_line() {
        let line = "2199943210";
        let height_map = read_height_map_line(line);

        assert_eq!(vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0], height_map);
    }

    #[test]
    fn test_build_height_map_from_sample() {
        let input_file = File::open("./data/sample9.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let height_map = build_height_map(lines);

        assert_eq!(vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0], height_map[0]);
        assert_eq!(vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1], height_map[1]);
        assert_eq!(vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2], height_map[2]);
        assert_eq!(vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9], height_map[3]);
        assert_eq!(vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8], height_map[4]);
    }

    #[test]
    fn test_get_adjacent_locations() {
        let input_file = File::open("./data/sample9.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        /*
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        */
        let height_map = build_height_map(lines);

        assert_eq!(
            vec![Location::new(1, 0, 3), Location::new(0, 1, 1)],
            get_adjacent_locations(&height_map, Location::new(0, 0, 2))
        );

        assert_eq!(
            vec![
                Location::new(1, 5, 9),
                Location::new(0, 4, 9),
                Location::new(0, 6, 3)
            ],
            get_adjacent_locations(&height_map, Location::new(0, 5, 4))
        );

        assert_eq!(
            vec![Location::new(1, 9, 1), Location::new(0, 8, 1)],
            get_adjacent_locations(&height_map, Location::new(0, 9, 0))
        );

        assert_eq!(
            vec![
                Location::new(1, 2, 8),
                Location::new(3, 2, 6),
                Location::new(2, 1, 8),
                Location::new(2, 3, 6)
            ],
            get_adjacent_locations(&height_map, Location::new(2, 2, 5))
        );
    }

    #[test]
    fn test_is_low_point() {
        let input_file = File::open("./data/sample9.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        /*
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        */
        let height_map = build_height_map(lines);

        assert!(!is_low_point(&height_map, 0, 0));
        assert!(is_low_point(&height_map, 0, 1));
        assert!(is_low_point(&height_map, 0, 9));

        assert!(!is_low_point(&height_map, 2, 1));
        assert!(is_low_point(&height_map, 2, 2));
        assert!(!is_low_point(&height_map, 2, 3));

        assert!(is_low_point(&height_map, 4, 6));
    }

    #[test]
    fn test_find_low_points() {
        let input_file = File::open("./data/sample9.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        /*
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        */
        let height_map = build_height_map(lines);
        let low_points = find_low_locations(&height_map);
        assert_eq!(
            vec![
                Location::new(0, 1, 1),
                Location::new(0, 9, 0),
                Location::new(2, 2, 5),
                Location::new(4, 6, 5)
            ],
            low_points
        );

        let sum = sum_risk_levels_of_low_points(&low_points);
        assert_eq!(15, sum);
    }

    #[test]
    fn test_find_low_locations() {
        let input_file = File::open("./data/sample9.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        /*
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        */
        let height_map = build_height_map(lines);
        let low_points = find_low_locations(&height_map);

        assert_eq!(Location::new(0, 1, 1), low_points[0]);
        assert_eq!(Location::new(0, 9, 0), low_points[1]);
        assert_eq!(Location::new(2, 2, 5), low_points[2]);
        assert_eq!(Location::new(4, 6, 5), low_points[3]);
    }

    #[test]
    fn test_find_basin_sizes() {
        let input_file = File::open("./data/sample9.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let height_map = build_height_map(lines);
        let basin_sizes = find_basin_sizes(&height_map);
        assert_eq!(vec![3, 9, 14, 9], basin_sizes);

        let mul = basin_sizes
            .iter()
            .sorted()
            .rev()
            .take(3)
            .map(|x| *x as i32)
            .product();
        assert_eq!(1134, mul);
    }
}
