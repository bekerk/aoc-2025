// --- Day 3: ??? ---
// https://adventofcode.com/2025/day/3

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<String> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_hello_world() {
        let input = file_to_vec("../input/day03.txt");
        assert_eq!(input, vec!["Hello, World!"]);
    }
}
