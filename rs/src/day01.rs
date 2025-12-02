// --- Day 1: Secret Entrance ---
// https://adventofcode.com/2025/day/1

pub fn get_password(input: &[String]) -> (i32, i32) {
    input.iter().fold((50, 0), |(dial, acc), instruction| {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let steps = instruction[1..].parse::<i32>().unwrap();
        let delta = sign * steps;
        // https://en.wikipedia.org/wiki/Euclidean_division
        let dial = (dial + delta).rem_euclid(100);

        (dial, acc + (dial == 0) as i32)
    })
}

pub fn get_password_v2(input: &[String]) -> (i32, i32) {
    input.iter().fold((50, 0), |(dial, acc), instruction| {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let steps = instruction[1..].parse::<i32>().unwrap();
        let delta = sign * steps;

        let times = if delta >= 0 {
            (dial + delta) / 100
        } else {
            let rev_dial = (100 - dial) % 100;
            (rev_dial - delta) / 100
        };

        let dial = (dial + delta).rem_euclid(100);

        (dial, acc + times)
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<String> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_solve_part_1() {
        let input = file_to_vec("../input/day01.txt");
        let password = super::get_password(&input);
        assert_eq!(password, (99, 995));
    }

    #[test]
    fn test_solve_part_2() {
        let input = file_to_vec("../input/day01.txt");
        let password = super::get_password_v2(&input);
        assert_eq!(password, (99, 5847));
    }
}
