// --- Day 6: Trash Compactor ---
// https://adventofcode.com/2025/day/6

use std::iter::zip;

pub fn flip_grid<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let num_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    for col in 0..num_cols {
        let mut stack = Vec::new();
        for item in grid.iter() {
            if col < item.len() {
                stack.push(item[col].clone());
            }
        }
        result.push(stack);
    }

    result
}

pub fn calculate_value(values: &[Vec<usize>], ops: &[String]) -> usize {
    zip(values, ops)
        .map(|(values, op)| match op.as_str() {
            "*" => values.iter().product(),
            "+" => values.iter().sum(),
            _ => 0,
        })
        .sum()
}

pub fn group_by_op(work_sheet: &[Vec<char>]) -> Vec<(char, Vec<Vec<char>>)> {
    let flipped = flip_grid(work_sheet);
    let mut grouped: Vec<(char, Vec<Vec<char>>)> = Vec::new();
    let mut current_group: Vec<Vec<char>> = Vec::new();
    let mut current_op: char = ' ';

    for mut row in flipped {
        let is_empty = row.iter().all(|&c| c == ' ');
        let sign = row.iter().find(|&&c| c == '*' || c == '+').copied();
        row.retain(|&c| c != ' ');

        if let Some(sign) = sign {
            current_op = sign;
            row.retain(|&c| c != current_op);
        }

        if is_empty {
            if !current_group.is_empty() {
                grouped.push((current_op, current_group));
                current_group = Vec::new();
            }
        } else {
            current_group.push(row);
        }
    }

    if !current_group.is_empty() {
        grouped.push((current_op, current_group));
    }

    grouped
}

pub fn calculate_work_sheet(work_sheet: &[Vec<String>]) -> usize {
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

pub fn calculate_work_sheet_with_most_significant_digits(work_sheet: &[Vec<char>]) -> usize {
    let grouped = group_by_op(work_sheet);

    let values = grouped
        .iter()
        .map(|(_, group)| {
            group
                .iter()
                .map(|row| {
                    row.iter()
                        .fold(0, |acc, x| acc * 10 + x.to_digit(10).unwrap_or(0) as usize)
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let ops = grouped
        .iter()
        .map(|(op, _)| op.to_string())
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

    fn file_to_vec_with_whitespace(file: &str) -> Vec<Vec<char>> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }

    #[test]
    fn test_calculate_work_sheet() {
        let input = file_to_vec("../input/day06.txt");
        assert_eq!(super::calculate_work_sheet(&input), 6957525317641);
    }

    #[test]
    fn test_calculate_work_sheet_with_most_significant_digits() {
        let input = file_to_vec_with_whitespace("../input/day06.txt");
        assert_eq!(
            super::calculate_work_sheet_with_most_significant_digits(&input),
            13215665360076
        );
    }
}
