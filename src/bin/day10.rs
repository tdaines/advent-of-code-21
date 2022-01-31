use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file = File::open("./data/day10.txt").unwrap();
    let reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let results: Vec<Result<String, char>> = lines.iter().map(|line| parse_line(line)).collect();

    let score = get_syntax_error_score(&results);
    println!("Total syntax error score is {}", score);

    let mut autocomplete_scores = get_autocomplete_scores(&results);
    autocomplete_scores.sort_unstable();

    let middle_score = autocomplete_scores[autocomplete_scores.len() / 2];
    println!("Autocomplete middle score is {}", middle_score);
}

fn get_syntax_error_score(results: &[Result<String, char>]) -> usize {
    let mut score: usize = 0;

    for result in results {
        if result.is_err() {
            match result.as_ref().err().unwrap() {
                ')' => score += 3,
                ']' => score += 57,
                '}' => score += 1197,
                '>' => score += 25137,
                _ => (),
            }
        }
    }

    score
}

fn get_autocomplete_scores(results: &[Result<String, char>]) -> Vec<usize> {
    let mut scores = Vec::with_capacity(results.len());

    for result in results {
        if result.is_ok() {
            let autocomplete = result.as_ref().unwrap();
            let mut score: usize = 0;
            for token in autocomplete.chars() {
                score *= 5;
                match token {
                    ')' => score += 1,
                    ']' => score += 2,
                    '}' => score += 3,
                    '>' => score += 4,
                    _ => (),
                }
            }

            scores.push(score);
        }
    }

    scores
}

fn parse_line(line: &str) -> Result<String, char> {
    // [({(<(())[]>[[{[]{<()<>>
    let mut token_stack = VecDeque::new();

    for token in line.chars() {
        match token {
            '[' | '(' | '{' | '<' => {
                token_stack.push_back(token);
            }
            ']' => {
                let top = token_stack.pop_back().unwrap();
                if top != '[' {
                    return Err(']');
                }
            }
            ')' => {
                let top = token_stack.pop_back().unwrap();
                if top != '(' {
                    return Err(')');
                }
            }
            '}' => {
                let top = token_stack.pop_back().unwrap();
                if top != '{' {
                    return Err('}');
                }
            }
            '>' => {
                let top = token_stack.pop_back().unwrap();
                if top != '<' {
                    return Err('>');
                }
            }
            _ => (),
        }
    }

    // Line is not corrupted, just incomplete
    let mut autocomplete: String = String::new();

    while !token_stack.is_empty() {
        let token = token_stack.pop_back().unwrap();
        match token {
            '[' => autocomplete += "]",
            '(' => autocomplete += ")",
            '{' => autocomplete += "}",
            '<' => autocomplete += ">",
            _ => (),
        }
    }

    Ok(autocomplete)
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn test_parse_line_corrupted() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        let result = parse_line(line);
        assert_eq!('}', result.err().unwrap());

        let line = "[[<[([]))<([[{}[[()]]]";
        let result = parse_line(line);
        assert_eq!(')', result.err().unwrap());

        let line = "[{[{({}]{}}([{[{{{}}([]";
        let result = parse_line(line);
        assert_eq!(']', result.err().unwrap());

        let line = "[<(<(<(<{}))><([]([]()";
        let result = parse_line(line);
        assert_eq!(')', result.err().unwrap());

        let line = "<{([([[(<>()){}]>(<<{{";
        let result = parse_line(line);
        assert_eq!('>', result.err().unwrap());
    }

    #[test]
    fn test_parse_line_valid() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        let result = parse_line(line);
        assert!(result.is_ok());
        assert_eq!("}}]])})]", result.unwrap());

        let line = "[(()[<>])]({[<{<<[]>>(";
        let result = parse_line(line);
        assert!(result.is_ok());
        assert_eq!(")}>]})", result.unwrap());
    }

    #[test]
    fn test_get_syntax_error_score() {
        let input_file = File::open("./data/sample10.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let results: Vec<Result<String, char>> =
            lines.iter().map(|line| parse_line(line)).collect();

        let score = get_syntax_error_score(&results);
        assert_eq!(26397, score);
    }

    #[test]
    fn test_get_autocomplete_scores() {
        let input_file = File::open("./data/sample10.txt").unwrap();
        let reader = BufReader::new(input_file);
        let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        let results: Vec<Result<String, char>> =
            lines.iter().map(|line| parse_line(line)).collect();

        let autocomplete_scores = get_autocomplete_scores(&results);

        assert_eq!(5, autocomplete_scores.len());
        assert_eq!(288957, autocomplete_scores[0]);
        assert_eq!(5566, autocomplete_scores[1]);
        assert_eq!(1480781, autocomplete_scores[2]);
        assert_eq!(995444, autocomplete_scores[3]);
        assert_eq!(294, autocomplete_scores[4]);
    }
}
