use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let input_file = File::open("./data/day9.txt").unwrap();
    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let height_map = build_height_map(lines);
    let low_points = find_low_points(&height_map);
    let risk_level_sum = sum_risk_levels_of_low_points(&low_points);

    println!("The sum of the risk levels of all the low points in the height map is {}", risk_level_sum);

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
    // let x = line.chars().map(|chr| chr.to_digit(10).unwrap() as u8);

    line.chars().map(|chr| chr.to_digit(10).unwrap() as u8).collect()
}

fn get_adjacent_heights(height_map: &[Vec<u8>], row: usize, col: usize) -> Vec<u8> {
    let mut adjacent = Vec::with_capacity(4);

    //above
    if row > 0 {
        adjacent.push(height_map[row - 1][col]);
    }

    //below
    if row < height_map.len() - 1 {
        adjacent.push(height_map[row + 1][col]);
    }

    //left
    if col > 0 {
        adjacent.push(height_map[row][col - 1]);
    }

    //right
    if col < height_map[0].len() - 1 {
        adjacent.push(height_map[row][col + 1]);
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

    let adjacent_heights = get_adjacent_heights(height_map, row, col);
    height < *adjacent_heights.iter().min().unwrap()
}

fn find_low_points(height_map: &[Vec<u8>]) -> Vec<u8> {
    let mut low_points = Vec::new();

    for row in 0..height_map.len() {
        for col in 0..height_map[row].len() {
            if is_low_point(height_map, row, col) {
                low_points.push(height_map[row][col]);
            }
        }
    }

    low_points
}

fn sum_risk_levels_of_low_points(low_points: &[u8]) -> usize {
    low_points.iter().map(|low_point| *low_point as usize + 1).sum()
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
    fn test_get_adjacent_heights() {
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

        assert_eq!(vec![3, 1],       get_adjacent_heights(&height_map, 0, 0));
        assert_eq!(vec![9, 9, 3],    get_adjacent_heights(&height_map, 0, 5));
        assert_eq!(vec![1, 1],       get_adjacent_heights(&height_map, 0, 9));

        assert_eq!(vec![3, 8, 8],    get_adjacent_heights(&height_map, 2, 0));
        assert_eq!(vec![8, 6, 8, 6], get_adjacent_heights(&height_map, 2, 2));
        assert_eq!(vec![1, 9, 9],    get_adjacent_heights(&height_map, 2, 9));

        assert_eq!(vec![8, 8],       get_adjacent_heights(&height_map, 4, 0));
        assert_eq!(vec![7, 5, 7],    get_adjacent_heights(&height_map, 4, 7));
        assert_eq!(vec![9, 7],       get_adjacent_heights(&height_map, 4, 9));
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
        let low_points = find_low_points(&height_map);
        assert_eq!(vec![1, 0, 5, 5], low_points);

        let sum = sum_risk_levels_of_low_points(&low_points);
        assert_eq!(15, sum);
    }
}