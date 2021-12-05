use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    let input_file = File::open("./data/day2.txt")?;
    let reader = BufReader::new(input_file);

    let mut forward_position: u32 = 0;
    let mut depth: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("forward ") {
            forward_position += line["forward ".len()..].parse::<u32>().unwrap();
        } else if line.starts_with("up ") {
            depth -= line["up ".len()..].parse::<u32>().unwrap();
        } else if line.starts_with("down ") {
            depth += line["down ".len()..].parse::<u32>().unwrap();
        }
    }

    println!("Forward: {}, Depth: {}, F x D = {}", forward_position, depth, forward_position * depth);
    Ok(())
}
