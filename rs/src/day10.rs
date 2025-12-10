// --- Day 10: Factory ---
// https://adventofcode.com/2025/day/10

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Diagram {
    state: HashMap<usize, bool>,
    target_state: HashMap<usize, bool>,
}

impl Diagram {
    fn new(target_state: HashMap<usize, bool>) -> Self {
        let state = target_state
            .clone()
            .iter()
            .map(|(k, _v)| (*k, false))
            .collect();
        Self {
            state,
            target_state,
        }
    }

    fn apply_button_press(&mut self, button_press: &[&Vec<usize>]) {
        button_press.iter().for_each(|button_press| {
            button_press.iter().for_each(|button| {
                let is_on = self.state.get(&button).unwrap();
                self.state.insert(*button, !is_on);
            });
        });
    }

    fn target_state_reached(&self) -> bool {
        self.state == self.target_state
    }
}

pub fn calculate_fewest_button_presses(
    input: &[(HashMap<usize, bool>, Vec<Vec<usize>>, Vec<usize>)],
) -> usize {
    input
        .iter()
        .filter_map(|(indicator_lights_map, button_presses, _joltages)| {
            (1..button_presses.len()).find(|&i| {
                button_presses.iter().combinations(i).any(|combination| {
                    let mut diagram = Diagram::new(indicator_lights_map.clone());
                    diagram.apply_button_press(&combination);
                    diagram.target_state_reached()
                })
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<(HashMap<usize, bool>, Vec<Vec<usize>>, Vec<usize>)> {
        fn parse_indicator_lights_diagram(s: &str) -> HashMap<usize, bool> {
            s.trim_matches(['[', ']'])
                .chars()
                .enumerate()
                .map(|(i, c)| (i, c == '#'))
                .collect()
        }

        fn parse_button_presses(s: &[&str]) -> Vec<Vec<usize>> {
            s.iter()
                .map(|s| {
                    s.trim_matches(['(', ')'])
                        .split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect()
                })
                .collect()
        }

        fn parse_joltages(s: &str) -> Vec<usize> {
            s.trim_matches(['{', '}'])
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect()
        }

        fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let light_diagram_map = parse_indicator_lights_diagram(parts.next().unwrap());
                let mut parts: Vec<_> = parts.collect();
                let joltages = parse_joltages(parts.pop().unwrap());
                let button_presses = parse_button_presses(&parts);

                (light_diagram_map, button_presses, joltages)
            })
            .collect()
    }

    #[test]
    fn test_parse_input() {
        let input = file_to_vec("../input/day10.txt");
        assert_eq!(super::calculate_fewest_button_presses(&input), 417);
    }
}
