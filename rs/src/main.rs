fn part_1_solution(input: &[&str]) -> (i32, i32) {
    input.iter().fold((50, 0), |(dial, acc), instruction| {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let steps = instruction[1..].parse::<i32>().unwrap();
        let delta = sign * steps;
        let dial = (dial + delta).rem_euclid(100);

        (dial, if dial == 0 { acc + 1 } else { acc })
    })
}

fn part_2_solution(input: &[&str]) -> (i32, i32) {
    input.iter().fold((50, 0), |(dial, acc), instruction| {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let steps = instruction[1..].parse::<i32>().unwrap();
        let delta = sign * steps;
        let dial = (dial + delta).rem_euclid(100);

        (dial, acc)
    })
}

fn main() {
    let input = include_str!("../../input/day01.txt")
        .lines()
        .collect::<Vec<&str>>();
    let part_1_result = part_1_solution(&input);
    let part_2_result = part_2_solution(&input);

    println!("Part 1 result: {:?}", part_1_result);
    println!("Part 2 result: {:?}", part_2_result);
}

#[test]
fn test_day01() {
    assert_eq!(1, 1);
}
