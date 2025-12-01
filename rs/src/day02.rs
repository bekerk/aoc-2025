// --- Day 2: ???---
// https://adventofcode.com/2025/day/2

pub fn hello() -> String {
    String::from("day 2")
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hello() {
        assert_eq!(super::hello(), "day 2");
    }
}
