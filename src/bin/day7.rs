use std::{cmp, fs::File, io::Read};

fn main() {
    let mut input_file = File::open("./data/day7.txt").unwrap();
    let mut input = String::new();
    let _ = input_file.read_to_string(&mut input);

    let positions = parse_positions(&input);
    let fuel_used = find_min_fuel_used(&positions, constant_fuel_burn);
    println!("Fuel used (constant fuel burn): {}", fuel_used);

    let fuel_used = find_min_fuel_used(&positions, increasing_fuel_burn);
    println!("Fuel used (increasing fuel burn): {}", fuel_used);
}

fn parse_positions(positions: &str) -> Vec<isize> {
    // parse 16,1,2,0,4,2,7,1,2,14
    positions
        .split(',')
        .map(|age| age.parse::<isize>().unwrap())
        .collect()
}

fn constant_fuel_burn(distance: isize) -> isize {
    distance
}

fn increasing_fuel_burn(distance: isize) -> isize {
    let n = distance;
    let d = 1;
    let a = 1;

    // same as (1..=distance).sum()
    (n * (2 * a + (n - 1) * d)) / 2
}

fn find_min_fuel_used(positions: &[isize], calculate_fuel_required: fn(isize) -> isize) -> isize {
    let mut min_fuel_used = isize::MAX;

    let min_position = *positions.iter().min().unwrap();
    let max_position = *positions.iter().max().unwrap();

    for target in min_position..=max_position {
        let fuel_used = sum_total_fuel_used_to_position(positions, target, calculate_fuel_required);
        min_fuel_used = cmp::min(min_fuel_used, fuel_used);
    }

    min_fuel_used
}

fn sum_total_fuel_used_to_position(
    positions: &[isize],
    target: isize,
    calculate_fuel_required: fn(isize) -> isize,
) -> isize {
    let mut total = 0;

    for position in positions {
        let distance = (position - target).abs();
        total += calculate_fuel_required(distance);
    }

    total
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn test_parse_positions() {
        let positions = "16,1,2,0,4,2,7,1,2,14";
        let positions = parse_positions(positions);

        assert_eq!(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], positions);
    }

    #[test]
    fn test_sum_total_fuel_used_to_position_constant_fuel_burn() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let target = 2;

        let total = sum_total_fuel_used_to_position(&positions, target, constant_fuel_burn);
        assert_eq!(37, total);
    }

    #[test]
    fn test_sum_total_fuel_used_to_position_increasing_fuel_burn() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let target = 2;
        let total = sum_total_fuel_used_to_position(&positions, target, increasing_fuel_burn);
        assert_eq!(206, total);

        let target = 5;
        let total = sum_total_fuel_used_to_position(&positions, target, increasing_fuel_burn);
        assert_eq!(168, total);
    }

    #[test]
    fn test_find_min_fuel_used_constant_fuel_burn() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let fuel_used = find_min_fuel_used(&positions, constant_fuel_burn);
        assert_eq!(37, fuel_used);
    }

    #[test]
    fn test_find_min_fuel_used_increasing_fuel_burn() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        let fuel_used = find_min_fuel_used(&positions, increasing_fuel_burn);
        assert_eq!(168, fuel_used);
    }

    #[test]
    fn test_constant_fuel_burn() {
        assert_eq!(3, constant_fuel_burn(3));
    }

    #[test]
    fn test_increasing_fuel_burn() {
        assert_eq!(5050, increasing_fuel_burn(100));
        assert_eq!(10, increasing_fuel_burn(4));
        assert_eq!(55, increasing_fuel_burn(10));
        assert_eq!(15, increasing_fuel_burn(5));
    }
}
