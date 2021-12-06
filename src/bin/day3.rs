use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    let input_file = File::open("./data/day3.txt")?;
    let reader = BufReader::new(input_file);

    let report: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let (gamma, epsilon) = calculate_gamma_and_epsilon(&report);

    let power_comsumption = gamma * epsilon;
    println!("Gamma: {}, Epsilon: {}, Power Consumption: {}", gamma, epsilon, power_comsumption);

    Ok(())
}

fn calculate_gamma_and_epsilon(report: &[String]) -> (u32, u32) {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    // Assume all binary numbers in report are the same length
    let length = &report[0].len();

    for i in 0..*length {
        let num_zeroes = report.iter().filter(|num| num.as_bytes()[i] == b'0').count();
        if num_zeroes > report.len() / 2 {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    let gamma: u32 = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon: u32 = u32::from_str_radix(&epsilon, 2).unwrap();

    (gamma, epsilon)
}

#[cfg(test)]
mod day3_tests {
    use super::*;
    mod test_calculate_gamma_and_epsilon {
        use super::*;

        #[test]
        fn sample() {
            let report = vec![
                "00100".to_owned(),
                "11110".to_owned(),
                "10110".to_owned(),
                "10111".to_owned(),
                "10101".to_owned(),
                "01111".to_owned(),
                "00111".to_owned(),
                "11100".to_owned(),
                "10000".to_owned(),
                "11001".to_owned(),
                "00010".to_owned(),
                "01010".to_owned(),
            ];

            assert_eq!(calculate_gamma_and_epsilon(&report), (22, 9));
        }
    }
}