use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FLASHED: i8 = -1;

fn main() {
    let input_file = File::open("./data/day11.txt").unwrap();
    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut grid = build_grid(lines);
    let mut total_flashes = 0;
    for step_num in 0..100 {
        let num_flashes = step(&mut grid);
        if num_flashes == grid.len() * grid[0].len() {
            println!("All octopuses flashed on step {}", step_num);
        }

        total_flashes += num_flashes;
    }

    println!("Total number of flashes is {}", total_flashes);

    let mut step_num = 100;
    loop {
        step_num += 1;
        let num_flashes = step(&mut grid);
        if num_flashes == grid.len() * grid[0].len() {
            println!("All octopuses flashed on step {}", step_num);
            break;
        }
    }
}

fn build_grid(lines: Vec<String>) -> Vec<Vec<i8>> {
    let mut grid = Vec::with_capacity(10);

    for line in lines {
        grid.push(read_grid_line(&line));
    }

    grid
}

fn read_grid_line(line: &str) -> Vec<i8> {
    // parse "5483143223"
    line.chars()
        .map(|chr| chr.to_digit(10).unwrap() as i8)
        .collect()
}

fn step(grid: &mut Vec<Vec<i8>>) -> usize {
    part_one(grid);
    part_two(grid);
    part_three(grid);

    let mut num_flashes: usize = 0;
    for row in grid {
        num_flashes += row.iter().filter(|energy| **energy == 0).count();
    }

    num_flashes
}

fn part_one(grid: &mut Vec<Vec<i8>>) {
    // Increase energy level of each by 1
    for row in grid {
        for energy in row {
            *energy += 1;
        }
    }
}

fn part_two(grid: &mut Vec<Vec<i8>>) {
    // Any octopus with an energy level greater than 9 flashes .
    // This increases the energy level of all adjacent octopuses by 1,
    // including octopuses that are diagonally adjacent.
    // If this causes an octopus to have an energy level greater than 9, it also flashes.
    // This process continues as long as new octopuses keep having their energy level increased beyond 9.
    // (An octopus can only flash at most once per step.)

    let mut octopus_flashed = true;

    while octopus_flashed {
        octopus_flashed = false;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] > 9 {
                    octopus_flashed = true;
                    grid[row][col] = FLASHED;
                    increase_adjacent_energy_levels(grid, row, col);
                }
            }
        }
    }
}

fn increase_adjacent_energy_levels(grid: &mut Vec<Vec<i8>>, row: usize, col: usize) {
    // above
    if row > 0 {
        // above left
        if col > 0 && grid[row - 1][col - 1] != FLASHED {
            grid[row - 1][col - 1] += 1;
        }

        // above
        if grid[row - 1][col] != FLASHED {
            grid[row - 1][col] += 1;
        }

        // above right
        if col < grid[0].len() - 1 && grid[row - 1][col + 1] != FLASHED {
            grid[row - 1][col + 1] += 1;
        }
    }

    // left
    if col > 0 && grid[row][col - 1] != FLASHED {
        grid[row][col - 1] += 1;
    }

    // right
    if col < grid[0].len() - 1 && grid[row][col + 1] != FLASHED {
        grid[row][col + 1] += 1;
    }

    // below
    if row < grid.len() - 1 {
        // below left
        if col > 0 && grid[row + 1][col - 1] != FLASHED {
            grid[row + 1][col - 1] += 1;
        }

        // below
        if grid[row + 1][col] != FLASHED {
            grid[row + 1][col] += 1;
        }

        // below right
        if col < grid[0].len() - 1 && grid[row + 1][col + 1] != FLASHED {
            grid[row + 1][col + 1] += 1;
        }
    }
}

fn part_three(grid: &mut Vec<Vec<i8>>) {
    // Any octopus that flashed during this step has its energy level set to 0,
    // as it used all of its energy to flash.
    for row in grid {
        for energy in row {
            if *energy == FLASHED {
                *energy = 0;
            }
        }
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    fn to_string(grid: &[Vec<i8>]) -> String {
        let mut strings = Vec::with_capacity(10);

        for row in 0..grid.len() {
            strings.push(grid[row].iter().map(|d| d.to_string()).collect::<String>());
        }

        strings.join("\n")
    }

    #[test]
    fn test_build_grid() {
        let input_file = File::open("./data/sample11_small.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let grid = build_grid(lines);
        assert_eq!(
            "11111\n\
             19991\n\
             19191\n\
             19991\n\
             11111",
            to_string(&grid)
        );
    }

    #[test]
    fn test_step() {
        let input_file = File::open("./data/sample11_small.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let mut grid = build_grid(lines);
        assert_eq!(
            "11111\n\
             19991\n\
             19191\n\
             19991\n\
             11111",
            to_string(&grid)
        );

        step(&mut grid);
        assert_eq!(
            "34543\n\
             40004\n\
             50005\n\
             40004\n\
             34543",
            to_string(&grid)
        );

        step(&mut grid);
        assert_eq!(
            "45654\n\
             51115\n\
             61116\n\
             51115\n\
             45654",
            to_string(&grid)
        );
    }

    #[test]
    fn test_count_flashes() {
        let input_file = File::open("./data/sample11.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let mut grid = build_grid(lines);
        assert_eq!(
            "5483143223\n\
             2745854711\n\
             5264556173\n\
             6141336146\n\
             6357385478\n\
             4167524645\n\
             2176841721\n\
             6882881134\n\
             4846848554\n\
             5283751526",
            to_string(&grid)
        );

        let mut total_flashes = 0;
        for _ in 0..10 {
            total_flashes += step(&mut grid);
        }

        assert_eq!(204, total_flashes);

        assert_eq!(
            "0481112976\n\
             0031112009\n\
             0041112504\n\
             0081111406\n\
             0099111306\n\
             0093511233\n\
             0442361130\n\
             5532252350\n\
             0532250600\n\
             0032240000",
            to_string(&grid)
        );

        for _ in 0..90 {
            total_flashes += step(&mut grid);
        }

        assert_eq!(1656, total_flashes);

        assert_eq!(
            "0397666866\n\
             0749766918\n\
             0053976933\n\
             0004297822\n\
             0004229892\n\
             0053222877\n\
             0532222966\n\
             9322228966\n\
             7922286866\n\
             6789998766",
            to_string(&grid)
        );
    }
}
