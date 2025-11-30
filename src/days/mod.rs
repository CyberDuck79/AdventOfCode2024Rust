pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub trait DayRunner {
    fn run(&self, input: &Vec<u8>);
}

pub fn run_day(day: u32, input: &Vec<u8>) -> Result<(), String> {
    match day {
        1 => { day1::Day1.run(input); Ok(()) },
        2 => { day2::Day2.run(input); Ok(()) },
        3 => { day3::Day3.run(input); Ok(()) },
        4 => { day4::Day4.run(input); Ok(()) },
        5 => { day5::Day5.run(input); Ok(()) },
        6 => { day6::Day6.run(input); Ok(()) },
        7 => { day7::Day7.run(input); Ok(()) },
        8 => { day8::Day8.run(input); Ok(()) },
        9 => { day9::Day9.run(input); Ok(()) },
        10 => { day10::Day10.run(input); Ok(()) },
        11 => { day11::Day11.run(input); Ok(()) },
        12 => { day12::Day12.run(input); Ok(()) },
        13 => { day13::Day13.run(input); Ok(()) },
        14 => { day14::Day14.run(input); Ok(()) },
        15 => { day15::Day15.run(input); Ok(()) },
        16 => { day16::Day16.run(input); Ok(()) },
        17 => { day17::Day17.run(input); Ok(()) },
        18 => { day18::Day18.run(input); Ok(()) },
        19 => { day19::Day19.run(input); Ok(()) },
        20 => { day20::Day20.run(input); Ok(()) },
        21 => { day21::Day21.run(input); Ok(()) },
        22 => { day22::Day22.run(input); Ok(()) },
        23 => { day23::Day23.run(input); Ok(()) },
        24 => { day24::Day24.run(input); Ok(()) },
        25 => { day25::Day25.run(input); Ok(()) },
        _ => Err(format!("Day {} not implemented", day)),
    }
}
