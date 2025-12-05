// --- Day 2: Gift Shop ---
// https://adventofcode.com/2025/day/2

use fancy_regex::Regex;
use lazy_static::lazy_static;

pub fn find_invalid_ids_in_ranges(
    ranges: &[(usize, usize)],
    filter_impl: fn(usize) -> bool,
) -> Vec<usize> {
    ranges
        .iter()
        .flat_map(|range| find_invalid_ids_in_range(*range, filter_impl))
        .collect::<Vec<usize>>()
}

pub fn find_invalid_ids_in_range(
    range: (usize, usize),
    filter_impl: fn(usize) -> bool,
) -> Vec<usize> {
    (range.0..=range.1)
        .filter(|id| filter_impl(*id))
        .collect::<Vec<usize>>()
}

pub fn is_invalid_id(id: usize) -> bool {
    // well... we could also just work on strings there and avoid passing usize around
    let len = id.ilog10() + 1;

    if len.is_multiple_of(2) {
        return id / 10_usize.pow(len / 2) == id % 10_usize.pow(len / 2);
    }

    return false;
}

pub fn is_invalid_id_v2(id: usize) -> bool {
    let str = id.to_string();
    let len = str.len();

    for i in 1..=len / 2 {
        if str[..i].repeat(len / i) == str {
            return true;
        }
    }

    false
}

lazy_static! {
    pub static ref ID_PATTERN_REGEX: Regex = Regex::new(r"^(.+)\1+$").unwrap();
}

pub fn is_invalid_id_v2_with_regex(id: usize) -> bool {
    ID_PATTERN_REGEX.is_match(&id.to_string()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<(usize, usize)> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split(',')
            .map(|s| {
                s.split('-')
                    .map(|part| part.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .map(|v| (v[0], v[1]))
            .collect::<Vec<(usize, usize)>>()
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
    fn test_find_invalid_ids_in_ranges() {
        let input = file_to_vec("../input/day02.txt");
        let invalid_ids = super::find_invalid_ids_in_ranges(&input, super::is_invalid_id);
        let sum = invalid_ids.iter().sum::<usize>();

        assert_eq!(sum, 15873079081);
    }

    #[test]
    fn test_find_invalid_ids_in_ranges_v2() {
        let input = file_to_vec("../input/day02.txt");
        let invalid_ids = super::find_invalid_ids_in_ranges(&input, super::is_invalid_id_v2);
        let sum = invalid_ids.iter().sum::<usize>();

        assert_eq!(sum, 22617871034);
    }

    #[test]
    fn test_solve_part_2_with_regex() {
        let input = file_to_vec("../input/day02.txt");
        let invalid_ids =
            super::find_invalid_ids_in_ranges(&input, super::is_invalid_id_v2_with_regex);
        let sum = invalid_ids.iter().sum::<usize>();

        assert_eq!(sum, 22617871034);
    }
}
