// --- Day 7: Laboratories ---
// https://adventofcode.com/2025/day/7

pub fn taychom_splitter(
    grid: &mut Vec<Vec<char>>,
    visited: &mut Vec<(usize, usize)>,
    x_pos: usize,
    y_pos: usize,
) -> usize {
    if y_pos >= grid.len() || x_pos >= grid[y_pos].len() || visited.contains(&(x_pos, y_pos)) {
        return 0;
    }

    if grid[y_pos][x_pos] == '^' {
        let mut total = 1;

        visited.push((x_pos, y_pos));
        total += taychom_splitter(grid, visited, x_pos - 1, y_pos);
        total += taychom_splitter(grid, visited, x_pos + 1, y_pos);

        total
    } else {
        taychom_splitter(grid, visited, x_pos, y_pos + 1)
    }
}

pub fn taychom_splitter_dfs_memo(
    grid: &mut Vec<Vec<char>>,
    memo: &mut Vec<Vec<Option<usize>>>,
    x_pos: usize,
    y_pos: usize,
) -> usize {
    if y_pos >= grid.len() || x_pos >= grid[y_pos].len() {
        return 1;
    }

    if let Some(total) = memo[y_pos][x_pos] {
        return total;
    }

    let mut total = 0;

    if grid[y_pos][x_pos] == '^' {
        total += taychom_splitter_dfs_memo(grid, memo, x_pos - 1, y_pos);
        total += taychom_splitter_dfs_memo(grid, memo, x_pos + 1, y_pos);
    } else {
        total += taychom_splitter_dfs_memo(grid, memo, x_pos, y_pos + 1);
    }

    memo[y_pos][x_pos] = Some(total);

    total
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
    fn test_taychom_splitter() {
        let mut input = file_to_vec("../input/day07.txt");
        let mut visited: Vec<(usize, usize)> = Vec::new();

        let start_x_pos = input[0].iter().position(|&c| c == 'S').unwrap();
        let start_y_pos = 1;

        assert_eq!(
            super::taychom_splitter(&mut input, &mut visited, start_x_pos, start_y_pos),
            1609
        );
    }

    #[test]
    fn test_taychom_splitter_dfs() {
        let mut input = file_to_vec("../input/day07.txt");
        let mut memo: Vec<Vec<Option<usize>>> = vec![vec![None; input[0].len()]; input.len()];

        let start_x_pos = input[0].iter().position(|&c| c == 'S').unwrap();
        let start_y_pos = 1;

        assert_eq!(
            super::taychom_splitter_dfs_memo(&mut input, &mut memo, start_x_pos, start_y_pos),
            12472142047197
        );
    }
}
