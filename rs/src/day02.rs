// --- Day 2: Gift Shop ---
// https://adventofcode.com/2025/day/2

pub fn find_invalid_ids_in_ranges(
    ranges: Vec<(i64, i64)>,
    filter_impl: fn(i64) -> bool,
) -> Vec<i64> {
    ranges
        .iter()
        .flat_map(|range| find_invalid_ids_in_range(*range, filter_impl))
        .collect::<Vec<i64>>()
}

pub fn find_invalid_ids_in_range(range: (i64, i64), filter_impl: fn(i64) -> bool) -> Vec<i64> {
    (range.0..=range.1)
        .filter(|id| filter_impl(*id))
        .collect::<Vec<i64>>()
}

pub fn is_invalid_id(id: i64) -> bool {
    let len = id.ilog10() + 1;
    let left: i64;
    let right: i64;

    if len.is_multiple_of(2) {
        left = id / 10_i64.pow(len / 2);
        right = id % 10_i64.pow(len / 2);

        return left == right;
    }

    false
}

pub fn is_invalid_id_v2(id: i64) -> bool {
    let str = id.to_string();
    let len = str.len();

    for i in 1..=len / 2 {
        if str[..i].repeat(len / i) == str {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<(i64, i64)> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split(',')
            .map(|s| {
                s.split('-')
                    .map(|part| part.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .map(|v| (v[0], v[1]))
            .collect::<Vec<(i64, i64)>>()
    }

    #[test]
    fn test_find_invalid_id_in_range() {
        assert_eq!(
            super::find_invalid_ids_in_range((11, 22), super::is_invalid_id),
            vec![11, 22]
        );
        assert_eq!(
            super::find_invalid_ids_in_range((99, 115), super::is_invalid_id),
            vec![99]
        );
        assert_eq!(
            super::find_invalid_ids_in_range((1188511880, 1188511890), super::is_invalid_id),
            vec![1188511885]
        );
        assert_eq!(
            super::find_invalid_ids_in_range((1698522, 1698528), super::is_invalid_id),
            vec![]
        );
    }

    #[test]
    fn test_find_invalid_id_v2_in_range() {
        assert_eq!(
            super::find_invalid_ids_in_range((11, 22), super::is_invalid_id_v2),
            vec![11, 22]
        );
        assert_eq!(
            super::find_invalid_ids_in_range((99, 115), super::is_invalid_id_v2),
            vec![99, 111]
        );
        assert_eq!(
            super::find_invalid_ids_in_range((1188511880, 1188511890), super::is_invalid_id_v2),
            vec![1188511885]
        );
        assert_eq!(
            super::find_invalid_ids_in_range((1698522, 1698528), super::is_invalid_id_v2),
            vec![]
        );
    }

    #[test]
    fn test_solve_part_1() {
        let input = file_to_vec("../input/day02.txt");
        let invalid_ids = super::find_invalid_ids_in_ranges(input, super::is_invalid_id);
        let sum = invalid_ids.iter().sum::<i64>();

        assert_eq!(sum, 15873079081);
    }

    #[test]
    fn test_solve_part_2() {
        let input = file_to_vec("../input/day02.txt");
        let invalid_ids = super::find_invalid_ids_in_ranges(input, super::is_invalid_id_v2);
        let sum = invalid_ids.iter().sum::<i64>();

        assert_eq!(sum, 22617871034);
    }
}
