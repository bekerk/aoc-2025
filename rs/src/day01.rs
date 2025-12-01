pub fn get_password(input: &[&str]) -> (i32, i32) {
    input.iter().fold((50, 0), |(dial, acc), instruction| {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let steps = instruction[1..].parse::<i32>().unwrap();
        let delta = sign * steps;
        let dial = (dial + delta).rem_euclid(100);

        (dial, acc + (dial == 0) as i32)
    })
}

pub fn get_password_v2(input: &[&str]) -> (i32, i32) {
    input.iter().fold((50, 0), |(dial, acc), instruction| {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let steps = instruction[1..].parse::<i32>().unwrap();
        let delta = sign * steps;
        let times: i32;

        if delta >= 0 {
            times = (dial + delta) / 100;
        } else {
            let rev_dial = (100 - dial) % 100;
            times = (rev_dial - delta) / 100;
        }

        let dial = (dial + delta).rem_euclid(100);

        (dial, acc + times)
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = include_str!("../../input/day01.txt")
            .split_whitespace()
            .collect::<Vec<&str>>();
        let password = super::get_password(&input);
        assert_eq!(password, (99, 995));
    }

    #[test]
    fn test_solve_part_2() {
        let input = include_str!("../../input/day01.txt")
            .split_whitespace()
            .collect::<Vec<&str>>();
        let password = super::get_password_v2(&input);
        assert_eq!(password, (99, 5847));
    }
}
