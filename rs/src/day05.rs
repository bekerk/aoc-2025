// --- Day 5: Cafeteria ---
// https://adventofcode.com/2025/day/5

pub fn count_fresh_ingredients(ranges: &[Vec<usize>], values: &[usize]) -> usize {
    values
        .iter()
        .filter(|&value| {
            ranges
                .iter()
                .any(|range| *value >= range[0] && *value <= range[1])
        })
        .count()
}

pub fn count_fresh_ids_in_ranges(ranges: &[Vec<usize>]) -> usize {
    let mut ranges = ranges.to_vec();
    ranges.sort_by_key(|range| range[0]);

    let mut merged = Vec::new();
    let (mut current_start, mut current_end) = (ranges[0][0], ranges[0][1]);

    for range in &ranges[1..] {
        let (start, end) = (range[0], range[1]);
        if start <= current_end {
            current_end = current_end.max(end);
        } else {
            merged.push((current_start, current_end));
            (current_start, current_end) = (start, end);
        }
    }

    merged.push((current_start, current_end));

    merged.iter().map(|(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn split_ranges(ranges: &str) -> Vec<Vec<usize>> {
        ranges
            .split("\n")
            .map(|range| {
                range
                    .split("-")
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>()
    }

    fn split_values(values: &str) -> Vec<usize> {
        values
            .split("\n")
            .map(|value| value.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    }

    fn file_to_vec(file: &str) -> (Vec<Vec<usize>>, Vec<usize>) {
        let file_content = fs::read_to_string(file).unwrap();
        let (ranges, values) = file_content.split_once("\n\n").unwrap();

        (split_ranges(ranges), split_values(values))
    }

    #[test]
    fn test_count_fresh_ingredients() {
        let (ranges, values) = file_to_vec("../input/day05.txt");
        assert_eq!(super::count_fresh_ingredients(&ranges, &values), 505);
    }

    #[test]
    fn test_count_fresh_ids_in_ranges() {
        let (ranges, _) = file_to_vec("../input/day05.txt");
        assert_eq!(super::count_fresh_ids_in_ranges(&ranges), 344423158480189);
    }
}
