use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet, iter};

fn main() {
    let input_file = File::open("./data/day8.txt").unwrap();
    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut count = 0;
    let mut sum: usize = 0;

    for line in &lines {
        let (input_digits, output_digits) = parse_input_signals_and_output_digits(line);
        for digit in &output_digits {
            // 1 has 2 segments
            // 4 has 4 segments
            // 7 has 3 segments
            // 8 has 7 segments
            if [2, 4, 3, 7].contains(&digit.len()) {
                count += 1;
            }
        }

        let input = determine_input_numbers(&input_digits);
        let output_number = determine_output_number(&input, &output_digits);

        sum += output_number;
    }

    println!("The digits 1, 4, 7, and 8 appear {} times", count);
    println!("The sum of the output values is {}", sum);
}

fn parse_input_signals_and_output_digits(input: &str) -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
    // parse "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"

    let parts: Vec<&str> = input.split('|').collect();
    let signals = parts[0];

    let mut input_signals = Vec::<HashSet::<char>>::new();
    for signal in signals.split_ascii_whitespace() {
        let mut signal_pattern = HashSet::new();
        for letter in signal.chars() {
            signal_pattern.insert(letter);
        }
        input_signals.push(signal_pattern);
    }

    let digits = parts[1];
    let mut output_digits = Vec::<HashSet::<char>>::new();
    for digit_pattern in digits.split_ascii_whitespace() {
        let mut output = HashSet::new();
        for digit in digit_pattern.chars() {
            output.insert(digit);
        }
        output_digits.push(output);
    }

    (input_signals, output_digits)
}

fn determine_input_numbers(input: &[HashSet<char>]) -> Vec<HashSet<char>> {
    let mut numbers: Vec<HashSet<char>> = iter::repeat_with(HashSet::<char>::new).take(10).collect();

    for some_num in input {
        match some_num.len() {
            2 => numbers[1] = some_num.to_owned(),
            3 => numbers[7] = some_num.to_owned(),
            4 => numbers[4] = some_num.to_owned(),
            7 => numbers[8] = some_num.to_owned(),
            _ => ()
        };
    }

    for some_num in input {
        if is_zero(&numbers[1], &numbers[4], some_num) {
            numbers[0] = some_num.to_owned();
        } else if is_two(&numbers[4], some_num) {
            numbers[2] = some_num.to_owned();
        } else if is_three(&numbers[1], some_num) {
            numbers[3] = some_num.to_owned();
        } else if is_five(&numbers[1], &numbers[4], some_num) {
            numbers[5] = some_num.to_owned();
        } else if is_six(&numbers[1], some_num) {
            numbers[6] = some_num.to_owned();
        } else if is_nine(&numbers[4], some_num) {
            numbers[9] = some_num.to_owned();
        }
    }

    numbers
}

fn determine_output_number(input: &[HashSet<char>], output_digits: &[HashSet<char>]) -> usize {
    let mut number = String::new();

    for digit in output_digits {
        let index = input.iter().position(|input_num| input_num  == digit).unwrap();
        number += &index.to_string();

    }

    number.parse::<usize>().unwrap()
}

fn is_zero(one: &HashSet<char>, four: &HashSet<char>, some_num: &HashSet<char>) -> bool {
    some_num.len() == 6 && one.intersection(some_num).count() == one.len() && four.intersection(some_num).count() == 3
}

fn is_two(four: &HashSet<char>, some_num: &HashSet<char>) -> bool {
    some_num.len() == 5 && four.intersection(some_num).count() == 2
}

fn is_three(one: &HashSet<char>, some_num: &HashSet<char>) -> bool {
    some_num.len() == 5 && one.intersection(some_num).count() == one.len()
}

fn is_five(one: &HashSet<char>, four: &HashSet<char>, some_num: &HashSet<char>) -> bool {
    some_num.len() == 5 && one.intersection(some_num).count() == 1 && four.intersection(some_num).count() == 3
}

fn is_six(one: &HashSet<char>, some_num: &HashSet<char>) -> bool {
    some_num.len() == 6 && one.intersection(some_num).count() == 1
}

fn is_nine(four: &HashSet<char>, some_num: &HashSet<char>) -> bool {
    some_num.len() == 6 && four.intersection(some_num).count() == four.len()
}

#[cfg(test)]
mod day8_tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_parse_input_signals_and_output_digits() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let (input, output) = parse_input_signals_and_output_digits(input);

        assert_eq!(10, input.len());

        assert_eq!(2, input[0].len());
        assert!(input[0].contains(&'b'));
        assert!(input[0].contains(&'e'));

        assert_eq!(7, input[1].len());
        assert!(input[1].contains(&'a'));
        assert!(input[1].contains(&'b'));
        assert!(input[1].contains(&'c'));
        assert!(input[1].contains(&'d'));
        assert!(input[1].contains(&'e'));
        assert!(input[1].contains(&'f'));
        assert!(input[1].contains(&'g'));

        assert_eq!(4, output.len());

        assert_eq!(7, output[0].len());
        assert!(output[0].contains(&'a'));
        assert!(output[0].contains(&'b'));
        assert!(output[0].contains(&'c'));
        assert!(output[0].contains(&'d'));
        assert!(output[0].contains(&'e'));
        assert!(output[0].contains(&'f'));
        assert!(output[0].contains(&'g'));
    }

    #[test]
    fn test_sample_part_1() {
        let input_file = File::open("./data/sample8.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let mut count = 0;
        for line in lines {
            let (_, output) = parse_input_signals_and_output_digits(&line);
            for digit in output {
                // 1 has 2 segments
                // 4 has 4 segments
                // 7 has 3 segments
                // 8 has 7 segments
                if [2, 4, 3, 7].contains(&digit.len()) {
                    count += 1;
                }
            }
        }

        assert_eq!(26, count);
    }

    #[test]
    fn test_determine_input_numbers() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let (input, _) = parse_input_signals_and_output_digits(input);

        let input_numbers = determine_input_numbers(&input);

        assert_eq!("abcdeg",  input_numbers[0].iter().sorted().collect::<String>());
        assert_eq!("ab",      input_numbers[1].iter().sorted().collect::<String>());
        assert_eq!("acdfg",   input_numbers[2].iter().sorted().collect::<String>());
        assert_eq!("abcdf",   input_numbers[3].iter().sorted().collect::<String>());
        assert_eq!("abef",    input_numbers[4].iter().sorted().collect::<String>());
        assert_eq!("bcdef",   input_numbers[5].iter().sorted().collect::<String>());
        assert_eq!("bcdefg",  input_numbers[6].iter().sorted().collect::<String>());
        assert_eq!("abd",     input_numbers[7].iter().sorted().collect::<String>());
        assert_eq!("abcdefg", input_numbers[8].iter().sorted().collect::<String>());
        assert_eq!("abcdef",  input_numbers[9].iter().sorted().collect::<String>());
    }

    #[test]
    fn test_determine_output_number() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let (input, output) = parse_input_signals_and_output_digits(input);

        let input_numbers = determine_input_numbers(&input);
        let output_number = determine_output_number(&input_numbers, &output);

        assert_eq!(5353,  output_number);
    }

    #[test]
    fn test_sample_part_2() {
        let input_file = File::open("./data/sample8.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let mut sum: usize = 0;
        for line in lines {
            let (input_digits, output_digits) = parse_input_signals_and_output_digits(&line);
            let input = determine_input_numbers(&input_digits);
            let output_number = determine_output_number(&input, &output_digits);

            sum += output_number;
        }

        assert_eq!(61229, sum);
    }
}