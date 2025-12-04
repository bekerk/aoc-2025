// --- Day 4: Printing Department ---
// https://adventofcode.com/2025/day/4

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
];

pub fn get_safe_positions(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let max_x = grid[0].len();
    let max_y = grid.len();
    let mut safe_positions: Vec<(usize, usize)> = Vec::new();

    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y][x] != '@' {
                continue;
            }
            let mut surrounding_rolls_count = 0;
            for (dx, dy) in NEIGHBOURS {
                if let (Some(nx), Some(ny)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    if nx < max_x && ny < max_y && grid[ny][nx] == '@' {
                        surrounding_rolls_count += 1;
                    }
                }
            }
            if surrounding_rolls_count < 4 {
                safe_positions.push((x, y));
            }
        }
    }

    safe_positions
}

pub fn count_total_rolls_that_can_be_removed(grid: &[Vec<char>]) -> usize {
    let mut grid = grid.to_vec();
    let mut total_rolls = 0;

    loop {
        let safe_positions = get_safe_positions(&grid);

        if safe_positions.is_empty() {
            break;
        }

        total_rolls += safe_positions.len();

        for (x, y) in safe_positions {
            grid[y][x] = '.';
        }
    }

    total_rolls
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<Vec<char>> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }

    #[test]
    fn test_iterate_over_grid() {
        let grid = file_to_vec("../input/day04.txt");
        assert_eq!(super::get_safe_positions(&grid).len(), 1540);
    }

    #[test]
    fn test_count_total_rolls_that_can_be_removed() {
        let grid = file_to_vec("../input/day04.txt");
        assert_eq!(super::count_total_rolls_that_can_be_removed(&grid), 8972);
    }
}
