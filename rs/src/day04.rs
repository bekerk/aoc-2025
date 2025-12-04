// --- Day 4: Printing Department ---
// https://adventofcode.com/2025/day/4

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
}
