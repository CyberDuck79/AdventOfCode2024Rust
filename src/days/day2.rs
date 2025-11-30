use super::DayRunner;

fn bytes_to_int(bytes: &[u8]) -> i32 {
    bytes.iter().fold(0, |acc, &b| {
        if b.is_ascii_digit() {
            acc * 10 + (b - b'0') as i32
        } else {
            acc
        }
    })
}

pub struct Day2;

impl DayRunner for Day2 {
    fn run(&self, input: &Vec<u8>) {
        let part1_result = self.part1(input);
        let part2_result = self.part2(input);
        
        println!("Day 2");
        println!("  Part 1: {}", part1_result);
        println!("  Part 2: {}", part2_result);
    }
}

impl Day2 {
    fn report_is_safe(levels: &Vec<i32>) -> bool {
        let start_signum = (levels[0] - levels[1]).signum();
        levels.windows(2).all(|window| {
            let diff = window[0] - window[1];
            diff != 0 && diff.signum() == start_signum && diff.abs() <= 3
        })
    }

    fn part1(&self, _input: &Vec<u8>) -> String {
        _input.split(|&b| b == b'\n')
              .map(|line| line.split(|&b| b == b' '))
              .map(|parts| parts.map(|part| bytes_to_int(part)).collect::<Vec<i32>>())
              .filter(|levels| Day2::report_is_safe(levels))
              .count().to_string()
    }

    fn tolerant_report_check(levels: &Vec<i32>) -> bool {
        Day2::report_is_safe(levels) || 
        (0..levels.len()).any(|i| {
            let variant: Vec<i32> = levels.iter().enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, &val)| val)
                .collect();
            Day2::report_is_safe(&variant)
        })
    }

    fn part2(&self, _input: &Vec<u8>) -> String {
        _input.split(|&b| b == b'\n')
              .map(|line| line.split(|&b| b == b' '))
              .map(|parts| parts.map(|part| bytes_to_int(part)).collect::<Vec<i32>>())
              .filter(|levels| Day2::tolerant_report_check(levels))
              .count().to_string()
    }
}
