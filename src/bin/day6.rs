use std::{collections::VecDeque, fs::File, io::Read};

struct Fish {
    // index into collection is age of fish
    // i.e. ages[2] = 42 => 42 fish have age of 2
    ages: VecDeque<u64>,
}

impl Fish {
    fn new(ages: &[u8]) -> Self {
        // length of 9 to account for ages 0 -> 8
        let mut fish_ages = [0_u64; 9];
        for age in ages {
            assert!((*age as usize) < fish_ages.len());
            fish_ages[*age as usize] += 1;
        }

        Self {
            ages: VecDeque::from(fish_ages),
        }
    }

    fn count(&self) -> u64 {
        self.ages.iter().sum()
    }

    fn simulate_day(&mut self) {
        let num_reproducing_fish = self.ages.pop_front().unwrap();

        self.ages[6] += num_reproducing_fish;
        self.ages.push_back(num_reproducing_fish);
    }

    fn simulate_days(&mut self, num_days: usize) {
        for _ in 0..num_days {
            self.simulate_day();
        }
    }
}

fn main() {
    let mut input_file = File::open("./data/day6.txt").unwrap();
    let mut input = String::new();
    let _ = input_file.read_to_string(&mut input);

    let fish_ages = parse_fish_ages(&input);

    let mut fish = Fish::new(&fish_ages);
    let num_days = 80;
    fish.simulate_days(num_days);
    println!("After {} days, there are {} fish", num_days, fish.count());

    let mut fish = Fish::new(&fish_ages);
    let num_days = 256;
    fish.simulate_days(num_days);
    println!("After {} days, there are {} fish", num_days, fish.count());
}

fn parse_fish_ages(ages: &str) -> Vec<u8> {
    // parse 3,4,3,1,2
    ages.split(',')
        .map(|age| age.parse::<u8>().unwrap())
        .collect()
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn test_parse_fish_ages() {
        let ages = "3,4,3,1,2";
        let ages = parse_fish_ages(ages);

        assert_eq!(vec![3, 4, 3, 1, 2], ages);
    }

    mod fish_tests {
        use super::*;

        #[test]
        fn test_simulate_day() {
            let mut fish = Fish::new(&[3_u8, 4, 3, 1, 2]);

            fish.simulate_day();
            assert_eq!(5, fish.count());

            fish.simulate_day();
            assert_eq!(6, fish.count());

            fish.simulate_day();
            assert_eq!(7, fish.count());

            fish.simulate_day();
            assert_eq!(9, fish.count());
        }

        #[test]
        fn test_simulate_days() {
            let mut fish = Fish::new(&[3_u8, 4, 3, 1, 2]);
            fish.simulate_days(18);
            assert_eq!(26, fish.count());

            let mut fish = Fish::new(&[3_u8, 4, 3, 1, 2]);
            fish.simulate_days(80);
            assert_eq!(5934, fish.count());

            let mut fish = Fish::new(&[3_u8, 4, 3, 1, 2]);
            fish.simulate_days(256);
            assert_eq!(26984457539, fish.count());
        }
    }
}
