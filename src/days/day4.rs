use super::DayRunner;

pub struct Day4;


const XMAS: &[u8; 4] = b"XMAS";

impl DayRunner for Day4 {
    fn run(&self, input: &Vec<u8>) {
        let part1_result = self.part1(input);
        let part2_result = self.part2(input);
        
        println!("Day 4");
        println!("  Part 1: {}", part1_result);
        println!("  Part 2: {}", part2_result);
    }
}

impl Day4 {
    fn part1(&self, input: &Vec<u8>) -> String {
        let grid: Vec<&[u8]> = input.split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .collect();
        
        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;
        
        // All 8 directions: (row_delta, col_delta)
        let directions: [(i32, i32); 8] = [
            (0, 1),   // right →
            (0, -1),  // left ←
            (1, 0),   // down ↓
            (-1, 0),  // up ↑
            (1, 1),   // diagonal ↘
            (-1, -1), // diagonal ↖
            (1, -1),  // diagonal ↙
            (-1, 1),  // diagonal ↗
        ];
        
        for row in 0..rows {
            for col in 0..cols {
                for &(dr, dc) in &directions {
                    if Day4::check_xmas(&grid, row, col, dr, dc, rows, cols) {
                        count += 1;
                    }
                }
            }
        }
        
        count.to_string()
    }

    fn check_xmas(grid: &Vec<&[u8]>, row: usize, col: usize, dr: i32, dc: i32, rows: usize, cols: usize) -> bool {
        for i in 0..4 {
            let new_row = row as i32 + dr * i;
            let new_col = col as i32 + dc * i;
            
            // Bounds check
            if new_row < 0 || new_row >= rows as i32 || new_col < 0 || new_col >= cols as i32 {
                return false;
            }
            
            if grid[new_row as usize][new_col as usize] != XMAS[i as usize] {
                return false;
            }
        }
        true
    }

    fn part2(&self, _input: &Vec<u8>) -> String {
        let grid: Vec<&[u8]> = _input.split(|&b| b == b'\n')
            .filter(|line| !line.is_empty())
            .collect();

        let cols = grid[0].len();

        let corner_pairs = vec![
            ((0, 0),(2, 2)),((0, 2),(2, 0))
        ];

        let mut found_shapes = 0;
        for row_window in grid.windows(3) {
            if row_window.len() < 3 {
                continue;
            }
            for col in 0..cols - 2 {
                // filter X-MAS shape -> always have A at center
                if row_window[1][col + 1] == b'A' {
                    // always have two adjacent M and two adjacent S corners
                    if corner_pairs.iter().all(|&((r1, c1), (r2, c2))| {
                        let corner1 = row_window[r1][col + c1];
                        let corner2 = row_window[r2][col + c2];
                        (corner1 == b'M' && corner2 == b'S') ||
                        (corner1 == b'S' && corner2 == b'M')
                    }) {
                        found_shapes += 1;
                    }
                }
            }
        }
        

        found_shapes.to_string()
    }
}
