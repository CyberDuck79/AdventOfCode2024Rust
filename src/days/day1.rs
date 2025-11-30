use std::collections::HashMap;

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

pub struct Day1;

impl DayRunner for Day1 {
    fn run(&self, input: &Vec<u8>) {
        let part1_result = self.part1(input);
        let part2_result = self.part2(input);
        
        println!("Day 1");
        println!("  Part 1: {}", part1_result);
        println!("  Part 2: {}", part2_result);
    }
}

impl Day1 {
    fn part1(&self, _input: &Vec<u8>) -> String {
		let mut left_list = Vec::new();
		let mut right_list = Vec::new();
        _input.chunks(14)
			.for_each(|line| {
				left_list.push(bytes_to_int(&line[0..5]));
				right_list.push(bytes_to_int(&line[8..13]));
			});
		// Sort once at the end
        left_list.sort();
        right_list.sort();
		
		// Calculate distance
        let distance: i32 = left_list.iter()
            .zip(right_list.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        
        distance.to_string()
    }

    fn part2(&self, _input: &Vec<u8>) -> String {
        let mut left_list = Vec::new();
		let mut right_list = Vec::new();
        _input.chunks(14)
			.for_each(|line| {
				left_list.push(bytes_to_int(&line[0..5]));
				right_list.push(bytes_to_int(&line[8..13]));
			});

		// Build frequency map for right_list
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for &num in &right_list {
            *counts.entry(num).or_insert(0) += 1;
        }

		// Calculate similarity score
        left_list.iter()
            .map(|&x| x * counts.get(&x).copied().unwrap_or(0))
            .sum::<i32>()
            .to_string()
    }
}
