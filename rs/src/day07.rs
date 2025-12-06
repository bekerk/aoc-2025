// --- Day 7: ??? ---
// https://adventofcode.com/2025/day/7

pub fn hello_world() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hello_world() {
        assert_eq!(super::hello_world(), "Hello, world!");
    }
}
