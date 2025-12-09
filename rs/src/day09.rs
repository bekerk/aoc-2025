use itertools::Itertools;

pub fn get_rectangle_area(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let width = p1.0.abs_diff(p2.0) + 1;
    let height = p1.1.abs_diff(p2.1) + 1;

    width * height
}

pub fn get_points_of_rectangle(p1: (usize, usize), p2: (usize, usize)) -> Vec<(usize, usize)> {
    let p3 = (p1.0, p2.1);
    let p4 = (p2.0, p1.1);

    vec![p1, p2, p3, p4]
}

pub fn get_max_rectangle_area(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .tuple_combinations::<(&_, &_)>()
        .map(|(a, b)| get_rectangle_area(*a, *b))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<(usize, usize)> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .filter_map(|s| {
                let mut chars = s.split(',');
                let x = chars.next().unwrap().parse::<usize>().ok()?;
                let y = chars.next().unwrap().parse::<usize>().ok()?;
                Some((x, y))
            })
            .collect::<Vec<(usize, usize)>>()
    }

    #[test]
    fn get_max_rectangle_area_test() {
        let input = file_to_vec("../input/day09.txt");
        assert_eq!(super::get_max_rectangle_area(&input), 4763509452);
    }
}
