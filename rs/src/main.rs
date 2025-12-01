fn get_password(input: &[&str]) -> (i32, i32) {
    input.iter().fold((50, 0), |(dial, acc), instruction| {
        let sign = if instruction.starts_with('L') { -1 } else { 1 };
        let steps = instruction[1..].parse::<i32>().unwrap();
        let delta = sign * steps;
        let dial = (dial + delta).rem_euclid(100);

        (dial, acc + (dial == 0) as i32)
    })
}

fn get_password_v2(input: &[&str]) -> (i32, i32) {
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

fn main() {
    let input = include_str!("../../input/day01.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let password = get_password(&input);
    let password_v2 = get_password_v2(&input);

    println!("Password (Part 1): {:?}", password);
    println!("Password (Part 2): {:?}", password_v2);
}

#[test]
fn test_day01() {
    let example_input: &[&str] = &[
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];
    let example_password = get_password(example_input);
    let example_password_v2 = get_password_v2(example_input);

    assert_eq!(example_password, (32, 3));
    assert_eq!(example_password_v2, (32, 6));
}
