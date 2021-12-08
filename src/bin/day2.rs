use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let input_file = File::open("./data/day2.txt")?;
    let reader = BufReader::new(input_file);

    let planned_course: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let (position, depth) = calculate_position_and_depth(&planned_course);

    println!(
        "Position: {}, Depth: {}, P x D = {}",
        position,
        depth,
        position * depth
    );
    Ok(())
}

fn calculate_position_and_depth(planned_course: &[String]) -> (u32, u32) {
    let mut forward_position: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    for course in planned_course {
        if let Some(value) = course.strip_prefix("forward ") {
            let forward = value.parse::<u32>().unwrap();
            forward_position += forward;
            depth += forward * aim;
        } else if let Some(value) = course.strip_prefix("up ") {
            aim -= value.parse::<u32>().unwrap();
        } else if let Some(value) = course.strip_prefix("down ") {
            aim += value.parse::<u32>().unwrap();
        }
    }

    (forward_position, depth)
}

#[cfg(test)]
mod day2_tests {
    use super::*;
    mod test_calculate_position_and_depth {
        use super::*;

        #[test]
        fn sample() {
            let planned_course = vec![
                "forward 5".to_owned(),
                "down 5".to_owned(),
                "forward 8".to_owned(),
                "up 3".to_owned(),
                "down 8".to_owned(),
                "forward 2".to_owned(),
            ];

            assert_eq!(calculate_position_and_depth(&planned_course), (15, 60));
        }
    }
}
