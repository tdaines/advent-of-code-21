use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    let input_file = File::open("./src/input.txt")?;
    let reader = BufReader::new(input_file);

    let measurements: Vec<u32> = reader.lines()
        .map(|depth| depth.unwrap().parse().unwrap())
        .collect();

    println!("Num increases =        {}", calculate_num_increases(&measurements));
    println!("Num window increases = {}", calculate_window_increases(&measurements));
    Ok(())
}

fn calculate_num_increases(measurements: &[u32]) -> u32 {
    let mut current_depth: u32 = 0;
    let mut num_increases: u32 = 0;

    for depth in measurements {
        if *depth > current_depth {
            num_increases = num_increases + 1;
        }

        current_depth = *depth;
    }

    num_increases - 1
}

fn calculate_window_increases(measurements: &[u32]) -> u32 {
    let mut current_window_sum: u32 = 0;
    let mut num_increases: u32 = 0;

    for depths in measurements.windows(3) {
        let sum: u32 = depths.iter().sum();
        if sum > current_window_sum {
            num_increases = num_increases + 1;
        }

        current_window_sum = sum;
    }

    num_increases - 1
}