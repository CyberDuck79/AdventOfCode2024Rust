use super::DayRunner;

pub struct Day3;

const MUL_PREFIX: &'static [u8; 4] = b"mul(";
const DO_INST: &'static [u8; 4] = b"do()";
const DONT_INST: &'static [u8; 7] = b"don't()";

impl DayRunner for Day3 {
    fn run(&self, input: &Vec<u8>) {
        let part1_result = self.part1(input);
        let part2_result = self.part2(input);
        
        println!("Day 3");
        println!("  Part 1: {}", part1_result);
        println!("  Part 2: {}", part2_result);
    }
}

impl Day3 {
    fn parse_mul(input: &Vec<u8>, start_idx: usize) -> Option<(i32, i32)> {
        let remaining = &input[start_idx + 4..]; // Skip "mul("

        let (first_num, comma_idx) = remaining
            .iter()
            .take_while(|&b| b.is_ascii_digit())
            .fold((0, 0), |(acc, count), &b| (acc * 10 + (b - b'0') as i32, count + 1));
        
        // Expecting a comma
        if comma_idx >= remaining.len() || remaining[comma_idx] != b',' {
            return None;
        }

        let (second_num, closing_idx) = remaining[comma_idx + 1..]
            .iter()
            .take_while(|&b| b.is_ascii_digit())
            .fold((0, comma_idx + 1), |(acc, count), &b| (acc * 10 + (b - b'0') as i32, count + 1));

        if closing_idx >= remaining.len() || remaining[closing_idx] != b')' {
            return None;
        }

        Some((first_num, second_num))
    }

    fn part1(&self, _input: &Vec<u8>) -> String {
        _input.windows(MUL_PREFIX.len())
            .enumerate()
            .filter(|(_, window)| *window == MUL_PREFIX)
            .filter_map(|(i, _)| Day3::parse_mul(_input, i))
            .map(|(a, b)| a * b)
            .sum::<i32>()
            .to_string()
    }

    fn is_active(do_indexes: &Vec<usize>, dont_indexes: &Vec<usize>, idx: usize) -> bool {
        let last_do = do_indexes.iter().filter(|&&i| i < idx).last();
        let last_dont = dont_indexes.iter().filter(|&&i| i < idx).last();

        match (last_do, last_dont) {
            (Some(&do_i), Some(&dont_i)) => do_i > dont_i,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }


    fn part2(&self, _input: &Vec<u8>) -> String {
        let do_indexes = _input
            .windows(DO_INST.len())
            .enumerate()
            .filter_map(|(i, window)| if window == DO_INST { Some(i) } else { None })
            .collect::<Vec<usize>>();
        let dont_indexes = _input
            .windows(DONT_INST.len())
            .enumerate()
            .filter_map(|(i, window)| if window == DONT_INST { Some(i) } else { None })
            .collect::<Vec<usize>>();
        _input.windows(MUL_PREFIX.len())
            .enumerate()
            .filter(|(_, window)| *window == MUL_PREFIX)
            .filter(|(i, _)| Day3::is_active(&do_indexes, &dont_indexes, *i))
            .filter_map(|(i, _)| Day3::parse_mul(&_input, i))
            .map(|(a, b)| a * b)
            .sum::<i32>()
            .to_string()
    }
}
