// --- Day 3: Lobby ---
// https://adventofcode.com/2025/day/3

pub fn maximum_possible_joltages(
    series: &Vec<Vec<i64>>,
    battery_func: fn(&Vec<i64>) -> i64,
) -> i64 {
    series.iter().map(|batteries| battery_func(batteries)).sum()
}

pub fn maximum_possible_joltage(batteries: &Vec<i64>) -> i64 {
    let len = batteries.len();
    let mut x = 0;
    let mut x_pos = 0;
    let mut y = 0;

    for i in 0..len - 1 {
        if batteries[i] > x {
            x = batteries[i];
            x_pos = i;
        }
    }

    for i in x_pos + 1..len {
        if batteries[i] > y {
            y = batteries[i];
        }
    }

    x * 10 + y
}

use std::cmp::Reverse;

pub fn maximum_possible_joltage_v2(batteries: &Vec<i64>) -> i64 {
    let n = batteries.len();
    let mut result = 0;
    let mut start = 0;

    for i in 0..12 {
        let remaining = 12 - i - 1;
        let end = n - remaining;

        let (max_pos, max_val) = batteries[start..end]
            .iter()
            .enumerate()
            .max_by_key(|(idx, x)| (**x, Reverse(*idx)))
            .unwrap();

        result = result * 10 + max_val;

        start = start + max_pos + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<Vec<i64>> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>()
    }

    #[test]
    fn test_maximum_possible_joltage() {
        assert_eq!(
            super::maximum_possible_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            98
        );
        assert_eq!(
            super::maximum_possible_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            89
        );
        assert_eq!(
            super::maximum_possible_joltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            78
        );
        assert_eq!(
            super::maximum_possible_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 1, 2, 1, 1, 1]),
            92
        );
    }

    #[test]
    fn test_maximum_possible_joltage_v2() {
        assert_eq!(
            super::maximum_possible_joltage_v2(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            987654321111
        );
        assert_eq!(
            super::maximum_possible_joltage_v2(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            811111111119
        );
        assert_eq!(
            super::maximum_possible_joltage_v2(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            434234234278
        );
        assert_eq!(
            super::maximum_possible_joltage_v2(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            888911112111
        );
    }

    #[test]
    fn test_solve_part_1() {
        let input = file_to_vec("../input/day03.txt");
        let joltage = super::maximum_possible_joltages(&input, super::maximum_possible_joltage);
        assert_eq!(joltage, 17443);
    }

    #[test]
    fn test_solve_part_2() {
        let input = file_to_vec("../input/day03.txt");
        let joltage = super::maximum_possible_joltages(&input, super::maximum_possible_joltage_v2);
        assert_eq!(joltage, 172167155440541);
    }
}
