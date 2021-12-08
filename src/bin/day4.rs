use std::{fs::File, io::Read, ops::Index, str::Lines};

#[derive(Copy, Clone)]
struct BingoNumber {
    value: u32,
    marked: bool,
}

impl BingoNumber {
    fn new(value: u32) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

struct BingoBoard {
    // 5x5 grid of bingo numbers
    board: [[BingoNumber; 5]; 5],
}

impl BingoBoard {
    fn from_lines(numbers: &mut Lines) -> Self {
        let mut board = [[BingoNumber::new(0); 5]; 5];
        for row in &mut board {
            let row_str = numbers.next().unwrap();
            let values: Vec<u32> = row_str
                .split_ascii_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            for (j, value) in values.iter().enumerate() {
                row[j] = BingoNumber::new(*value);
            }
        }

        Self { board }
    }

    fn mark(&mut self, value: u32) {
        for row in &mut self.board {
            for num in row {
                if num.value == value {
                    num.marked = true;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        // Check rows
        for row in &self.board {
            if row.iter().all(|num| num.marked) {
                return true;
            }
        }

        // Check columns
        for col in 0..self.board.len() {
            if (0..self.board[col].len())
                .map(|row| &self.board[row][col])
                .all(|num| num.marked)
            {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0u32;

        for row in &self.board {
            let row_sum: u32 = row
                .iter()
                .filter(|&num| !num.marked)
                .map(|&num| num.value)
                .sum();
            sum += row_sum;
        }

        sum
    }
}

impl Index<usize> for BingoBoard {
    type Output = [BingoNumber; 5];

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("./data/day4.txt")?;

    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines();
    let draw = parse_bingo_draw(lines.next().unwrap());

    let mut boards = Vec::new();

    while let Some(_) = lines.next() {
        let board = BingoBoard::from_lines(&mut lines);
        boards.push(board);
    }

    for number in draw {
        for board in &mut boards {
            board.mark(number);
            if board.has_bingo() {
                let sum_unmarked = board.sum_unmarked();
                println!(
                    "Sum: {}, Number: {}, SxN: {}",
                    sum_unmarked,
                    number,
                    sum_unmarked * number
                );
                return Ok(());
            }
        }
    }

    Ok(())
}

fn parse_bingo_draw(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    mod test_bingo_board {
        use super::*;

        #[test]
        fn test_from_lines() {
            fn row_to_string(row: &[BingoNumber; 5]) -> String {
                row.iter()
                    .map(|&num| num.value.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            }

            let lines = "22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19";

            let board = BingoBoard::from_lines(&mut lines.lines());

            assert_eq!(row_to_string(&board[0]), "22 13 17 11 0");
            assert!(board[0].iter().all(|&num| num.marked == false));

            assert_eq!(row_to_string(&board[1]), "8 2 23 4 24");
            assert!(board[1].iter().all(|&num| num.marked == false));

            assert_eq!(row_to_string(&board[2]), "21 9 14 16 7");
            assert!(board[2].iter().all(|&num| num.marked == false));

            assert_eq!(row_to_string(&board[3]), "6 10 3 18 5");
            assert!(board[3].iter().all(|&num| num.marked == false));

            assert_eq!(row_to_string(&board[4]), "1 12 20 15 19");
            assert!(board[4].iter().all(|&num| num.marked == false));
        }

        #[test]
        fn test_mark() {
            let lines = "22 13 17 11  0
            8  2 23  4 24
           21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19";

            let mut board = BingoBoard::from_lines(&mut lines.lines());
            board.mark(77);
            board.mark(23);
            board.mark(5);

            assert!(board[1][2].marked);
            assert!(board[1][4].marked == false);
            assert!(board[3][4].marked);
        }

        #[test]
        fn test_sum_unmarked() {
            let lines = "22 13 17 11  0
            8  2 23  4 24
           21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19";

            let mut board = BingoBoard::from_lines(&mut lines.lines());
            board.mark(22);
            board.mark(13);
            board.mark(17);
            board.mark(23);
            board.mark(24);
            board.mark(21);
            board.mark(9);
            board.mark(14);
            board.mark(16);
            board.mark(10);
            board.mark(3);
            board.mark(18);
            board.mark(12);
            board.mark(20);
            board.mark(15);
            board.mark(19);

            assert_eq!(board.sum_unmarked(), 44);
        }

        #[test]
        fn test_has_bingo_row() {
            let lines = "22 13 17 11  0
            8  2 23  4 24
           21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19";

            let mut board = BingoBoard::from_lines(&mut lines.lines());

            assert!(!board.has_bingo());
            board.mark(1);
            assert!(!board.has_bingo());
            board.mark(12);
            assert!(!board.has_bingo());
            board.mark(20);
            assert!(!board.has_bingo());
            board.mark(15);
            assert!(!board.has_bingo());
            board.mark(19);
            assert!(board.has_bingo());
        }

        #[test]
        fn test_has_bingo_col() {
            let lines = "22 13 17 11  0
            8  2 23  4 24
           21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19";

            let mut board = BingoBoard::from_lines(&mut lines.lines());

            assert!(!board.has_bingo());
            board.mark(0);
            assert!(!board.has_bingo());
            board.mark(24);
            assert!(!board.has_bingo());
            board.mark(7);
            assert!(!board.has_bingo());
            board.mark(5);
            assert!(!board.has_bingo());
            board.mark(19);
            assert!(board.has_bingo());
        }
    }

    mod test_parse_bingo_draw {
        use super::*;

        #[test]
        fn test_sample() {
            let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7"
                .to_owned();

            let draw = parse_bingo_draw(input.lines().next().unwrap());
            let string_list: String = draw
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");

            assert_eq!(
                string_list,
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
            );
        }
    }
}
