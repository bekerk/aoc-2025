// --- Day 12: ??? ---
// https://adventofcode.com/2025/day/12
// FINAL DAY!!!!

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    fn parse_region(line: &str) -> ((usize, usize), Vec<usize>) {
        let mut parts = line.split_whitespace();
        let grid_str = parts.next().unwrap().trim_end_matches(':');
        let (w, h) = grid_str.split_once('x').unwrap();
        let dimensions = (w.parse().unwrap(), h.parse().unwrap());
        let quantities = parts.map(|s| s.parse().unwrap()).collect();
        (dimensions, quantities)
    }

    fn parse_shape(block: &str) -> Vec<Vec<char>> {
        block
            .lines()
            .skip(1)
            .map(|line| line.chars().collect())
            .collect()
    }

    fn file_to_vec(file: &str) -> (Vec<((usize, usize), Vec<usize>)>, Vec<Vec<Vec<char>>>) {
        let content = fs::read_to_string(file).unwrap();
        let sections: Vec<&str> = content.trim().split("\n\n").collect();

        let (regions_section, shape_sections) = sections.split_last().unwrap();

        let regions = regions_section.lines().map(parse_region).collect();
        let shapes = shape_sections.iter().map(|s| parse_shape(s)).collect();

        (regions, shapes)
    }

    #[test]
    fn hello_world() {
        let (regions, shapes) = file_to_vec("../input/day12.txt");

        println!("Regions: {:?}", regions);
        println!("Shapes: {:?}", shapes);
    }
}
