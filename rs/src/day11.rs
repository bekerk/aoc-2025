// --- Day 11: Reactor ---
// https://adventofcode.com/2025/day/11

use std::collections::HashMap;

pub fn dfs_paths_count(map: &HashMap<usize, Vec<usize>>, node: usize, end: usize) -> usize {
    if node == end {
        return 1;
    }

    let mut count = 0;

    if let Some(neighbors) = map.get(&node) {
        for neighbor in neighbors {
            count += dfs_paths_count(map, *neighbor, end);
        }
    }

    count
}

pub fn dfs_memoized_with_conditions(
    map: &HashMap<usize, Vec<usize>>,
    memo: &mut HashMap<(usize, bool, bool), usize>,
    node: usize,
    end: usize,
    mut has_fft: bool,
    mut has_dac: bool,
    fft_id: usize,
    dac_id: usize,
) -> usize {
    if node == fft_id {
        has_fft = true;
    }
    if node == dac_id {
        has_dac = true;
    }

    let key = (node, has_fft, has_dac);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if node == end {
        let result = if has_fft && has_dac { 1 } else { 0 };
        memo.insert(key, result);
        return result;
    }

    let mut total = 0;
    if let Some(neighbors) = map.get(&node) {
        for &n in neighbors {
            total +=
                dfs_memoized_with_conditions(map, memo, n, end, has_fft, has_dac, fft_id, dac_id);
        }
    }

    memo.insert(key, total);

    total
}

// credit goes to https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2025/day11.rs
pub fn to_index(s: &str) -> usize {
    s.bytes()
        .take(3)
        .fold(0, |acc, b| 26 * acc + usize::from(b - b'a'))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    fn file_to_hash_map(file: &str) -> HashMap<usize, Vec<usize>> {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .for_each(|s| {
                let parts = s.split(":").collect::<Vec<&str>>();
                let key = super::to_index(parts[0].trim());
                let values = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| super::to_index(s))
                    .collect::<Vec<usize>>();

                map.insert(key, values);
            });

        map
    }

    #[test]
    fn test_dfs_paths_count() {
        let input = file_to_hash_map("../input/day11.txt");
        let length = super::dfs_paths_count(&input, super::to_index("you"), super::to_index("out"));
        assert_eq!(length, 786);
    }

    #[test]
    fn test_dfs_memoized_with_conditions() {
        let input = file_to_hash_map("../input/day11.txt");
        let length = super::dfs_memoized_with_conditions(
            &input,
            &mut HashMap::new(),
            super::to_index("svr"),
            super::to_index("out"),
            false,
            false,
            super::to_index("fft"),
            super::to_index("dac"),
        );
        assert_eq!(length, 495845045016588);
    }
}
