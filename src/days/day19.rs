use super::DayRunner;

pub struct Day19;

impl DayRunner for Day19 {
    fn run(&self, input: &Vec<u8>) {
        let part1_result = self.part1(input);
        let part2_result = self.part2(input);
        
        println!("Day 19");
        println!("  Part 1: {}", part1_result);
        println!("  Part 2: {}", part2_result);
    }
}

impl Day19 {
    fn part1(&self, _input: &Vec<u8>) -> String {
        "TODO".to_string()
    }

    fn part2(&self, _input: &Vec<u8>) -> String {
        "TODO".to_string()
    }
}
