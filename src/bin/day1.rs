use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    let input_file = File::open("./data/day1.txt")?;
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
            num_increases += 1;
        }

        current_depth = *depth;
    }

    if num_increases > 0 {
        num_increases -= 1;
    }

    num_increases
}

fn calculate_window_increases(measurements: &[u32]) -> u32 {
    let mut current_window_sum: u32 = 0;
    let mut num_increases: u32 = 0;

    for depths in measurements.windows(3) {
        let sum: u32 = depths.iter().sum();
        if sum > current_window_sum {
            num_increases += 1;
        }

        current_window_sum = sum;
    }

    if num_increases > 0 {
        num_increases -= 1;
    }

    num_increases
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_calculate_num_increases {
        use super::*;

        #[test]
        fn empty_vec() {
            let measurements = Vec::new();
            assert_eq!(calculate_num_increases(&measurements), 0);
        }

        #[test]
        fn vec_with_one_item() {
            let measurements = Vec::new();
            assert_eq!(calculate_num_increases(&measurements), 0);
        }

        #[test]
        fn no_increases() {
            let measurements = vec![5, 4, 3, 2, 1, 0];
            assert_eq!(calculate_num_increases(&measurements), 0);
        }

        #[test]
        fn some_increases() {
            let measurements = vec![5, 4, 6, 7, 3, 8];
            assert_eq!(calculate_num_increases(&measurements), 3);
        }
    }

    mod test_calculate_window_increases {
        use super::*;

        #[test]
        fn empty_vec() {
            let measurements = Vec::new();
            assert_eq!(calculate_window_increases(&measurements), 0);
        }

        #[test]
        fn vec_with_one_item() {
            let measurements = Vec::new();
            assert_eq!(calculate_window_increases(&measurements), 0);
        }

        #[test]
        fn no_increases() {
            let measurements = vec![5, 4, 3, 2, 1, 0];
            assert_eq!(calculate_window_increases(&measurements), 0);
        }

        #[test]
        fn some_increases() {
            let measurements = vec![5, 4, 6, 7, 3, 8];
            assert_eq!(calculate_window_increases(&measurements), 2);
        }
    }
}