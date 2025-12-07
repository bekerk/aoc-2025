// --- Day 8: ??? ---
// https://adventofcode.com/2025/day/8

pub fn hello_world() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    #[allow(unused)]
    fn file_to_vec(file: &str) -> Vec<Vec<char>> {
        fs::read_to_string(file)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }

    #[test]
    fn test_hello_world() {
        assert_eq!(super::hello_world(), "Hello, World!");
    }
}
