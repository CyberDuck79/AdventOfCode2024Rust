use super::DayRunner;
use std::collections::{HashMap, HashSet};

pub struct Day5;

fn bytes_to_int(bytes: &[u8]) -> i32 {
    bytes.iter().fold(0, |acc, &b| {
        if b.is_ascii_digit() {
            acc * 10 + (b - b'0') as i32
        } else {
            acc
        }
    })
}


impl DayRunner for Day5 {
    fn run(&self, input: &Vec<u8>) {
        let part1_result = self.part1(input);
        let part2_result = self.part2(input);
        
        println!("Day 5");
        println!("  Part 1: {}", part1_result);
        println!("  Part 2: {}", part2_result);
    }
}

fn input_parsing(_input: &Vec<u8>) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let ordering_rules: HashMap<i32, HashSet<i32>> = _input
        .split(|&b| b == b'\n')
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let x = bytes_to_int(&line[..2]);
            let y = bytes_to_int(&line[3..]);
            (x, y)
        })
        .fold(HashMap::new(), |mut map, (x, y)| {
            map.entry(x).or_insert_with(HashSet::new).insert(y);
            map
        });
    let updates = _input.split(|&b| b == b'\n')
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(|&b| b == b',')
                .map(|number| bytes_to_int(number))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    (ordering_rules, updates)
}

impl Day5 {
    fn validate_update(update_pages: &Vec<i32>, ordering_rules: &HashMap<i32, HashSet<i32>>) -> Option<i32> {
        for i in 0..(update_pages.len() as i32 - 1) {
            for j in i+1..(update_pages.len() as i32) {
                let page_i = update_pages[i as usize];
                let page_j = update_pages[j as usize];
                if !ordering_rules.get(&page_i).map_or(false, |set| set.contains(&page_j)) {
                    return None;
                }
            }
        }
        Some(update_pages[update_pages.len() / 2])
    }

    fn part1(&self, _input: &Vec<u8>) -> String {
        let (ordering_rules, updates) = input_parsing(_input);
        updates.iter()
            .filter_map(|update_pages| Day5::validate_update(update_pages, &ordering_rules))
            .sum::<i32>().to_string()
    }

    fn fix_invalid_update(update_pages: &Vec<i32>, ordering_rules: &HashMap<i32, HashSet<i32>>) -> Option<i32> {
        let mut update_pages = update_pages.clone();
        let mut invalid = false;
        for i in (1..update_pages.len()).rev() {
            for j in (0..i).rev() {
                let page_i = update_pages[i as usize];
                let page_j = update_pages[j as usize];
                if !ordering_rules.get(&page_j).map_or(false, |set| set.contains(&page_i)) {
                    // Swap pages
                    update_pages.swap(i as usize, j as usize);
                    invalid = true;
                }
            }
        }
        if invalid {
            Some(update_pages[update_pages.len() / 2])
        } else {
            None
        }
    }

    fn part2(&self, _input: &Vec<u8>) -> String {
        let (ordering_rules, updates) = input_parsing(_input);
        updates.iter()
            .filter_map(|update_pages| Day5::fix_invalid_update(update_pages, &ordering_rules))
            .sum::<i32>().to_string()
    }
}
