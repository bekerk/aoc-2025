// --- Day 3: Lobby ---
// https://adventofcode.com/2025/day/3

pub fn maximum_possible_joltages(
    series: &[Vec<usize>],
    battery_func: fn(&[usize]) -> usize,
) -> usize {
    series.iter().map(|batteries| battery_func(batteries)).sum()
}

pub fn maximum_possible_joltage(batteries: &[usize]) -> usize {
    let len = batteries.len();
    let (mut x, mut x_pos, mut y) = (0, 0, 0);

    for (i, &battery) in batteries.iter().enumerate().take(len - 1) {
        if battery > x {
            x = battery;
            x_pos = i;
        }
    }

    for &battery in batteries.iter().skip(x_pos + 1) {
        if battery > y {
            y = battery;
        }
    }

    x * 10 + y
}

use std::cmp::Reverse;

pub fn maximum_possible_joltage_v2(batteries: &[usize]) -> usize {
    let n = batteries.len();
    let (mut result, mut start) = (0, 0);

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

    fn file_to_vec(file: &str) -> Vec<Vec<usize>> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>()
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
    fn test_maximum_possible_joltages() {
        let input = file_to_vec("../input/day03.txt");
        let joltage = super::maximum_possible_joltages(&input, super::maximum_possible_joltage);
        assert_eq!(joltage, 17443);
    }

    #[test]
    fn test_maximum_possible_joltages_v2() {
        let input = file_to_vec("../input/day03.txt");
        let joltage = super::maximum_possible_joltages(&input, super::maximum_possible_joltage_v2);
        assert_eq!(joltage, 172167155440541);
    }
}
