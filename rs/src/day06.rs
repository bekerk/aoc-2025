// --- Day 6: Trash Compactor ---
// https://adventofcode.com/2025/day/6

use std::iter::zip;

pub fn flip_grid<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let num_rows = grid.len();
    let num_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    for col in 0..num_cols {
        let mut stack = Vec::new();
        for row in 0..num_rows {
            if col < grid[row].len() {
                stack.push(grid[row][col].clone());
            }
        }
        result.push(stack);
    }

    result
}

pub fn calculate_value(values: &Vec<Vec<usize>>, ops: &[String]) -> usize {
    zip(values, ops)
        .map(|(values, op)| match op.as_str() {
            "*" => {
                let result = values.iter().fold(1, |acc, x| acc * x);
                println!("for values {:?} * x: {:?}", values, result);
                result
            }
            "+" => {
                let result = values.iter().sum::<usize>();
                println!("for values {:?} + x: {:?}", values, result);
                result
            }
            _ => 0,
        })
        .sum()
}

pub fn split_into_digits(value: usize) -> Vec<usize> {
    value
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}

pub fn calculate_work_sheet(work_sheet: &Vec<Vec<String>>) -> usize {
    let flipped_work_sheet = flip_grid(work_sheet);

    let values = flipped_work_sheet
        .iter()
        .map(|row| {
            row.iter()
                .take(row.len() - 1)
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let ops = flipped_work_sheet
        .iter()
        .map(|row| row.last().unwrap().clone())
        .collect::<Vec<String>>();

    calculate_value(&values, &ops)
}

pub fn calculate_work_sheet_with_most_significant_digits(work_sheet: &Vec<Vec<String>>) -> usize {
    let flipped = flip_grid(work_sheet);

    let values: Vec<Vec<usize>> = flipped
        .iter()
        .map(|row| {
            let nums: Vec<usize> = row[..row.len() - 1]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect();

            let digits: Vec<Vec<_>> = nums
                .iter()
                .map(|&v| split_into_digits(v).into_iter().rev().collect())
                .collect();

            flip_grid(&digits)
                .iter()
                .map(|d| d.iter().fold(0, |acc, x| acc * 10 + x))
                .collect()
        })
        .collect();

    let ops = flipped
        .iter()
        .map(|row| row.last().unwrap().clone())
        .collect::<Vec<String>>();

    calculate_value(&values, &ops)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<Vec<String>> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
    }

    #[test]
    fn test_calculate_work_sheet() {
        let input = file_to_vec("../input/day06.txt");
        assert_eq!(super::calculate_work_sheet(&input), 4277556);
    }

    #[test]
    fn test_calculate_work_sheet_with_most_significant_digits() {
        let input = file_to_vec("../input/day06.txt");
        assert_eq!(
            super::calculate_work_sheet_with_most_significant_digits(&input),
            3263827
        );
    }
}
