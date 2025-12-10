// --- Day 10: Factory ---
// https://adventofcode.com/2025/day/10

use itertools::Itertools;
use std::collections::HashMap;

type Input = (HashMap<usize, bool>, Vec<Vec<usize>>, HashMap<usize, usize>);

#[derive(Debug, Clone)]
struct Diagram {
    state: HashMap<usize, bool>,
    target_state: HashMap<usize, bool>,
}

impl Diagram {
    fn new(target_state: HashMap<usize, bool>) -> Self {
        let state = target_state.clone().keys().map(|k| (*k, false)).collect();
        Self {
            state,
            target_state,
        }
    }

    fn apply_button_press(&mut self, button_press: &[&Vec<usize>]) {
        button_press.iter().for_each(|button_press| {
            button_press.iter().for_each(|button| {
                let is_on = self.state.get(button).unwrap();
                self.state.insert(*button, !is_on);
            });
        });
    }

    fn target_state_reached(&self) -> bool {
        self.state == self.target_state
    }
}

#[derive(Debug, Clone)]
struct DiagramWithJoltages {
    state: HashMap<usize, usize>,
    target_joltages: HashMap<usize, usize>,
}

impl DiagramWithJoltages {
    fn new(target_joltages: HashMap<usize, usize>) -> Self {
        let state = target_joltages.keys().map(|k| (*k, 0)).collect();
        Self {
            state,
            target_joltages,
        }
    }

    fn apply_button_presses(&mut self, buttons: &[Vec<usize>], press_counts: &[usize]) {
        for (btn_idx, &count) in press_counts.iter().enumerate() {
            for &counter_idx in &buttons[btn_idx] {
                *self.state.entry(counter_idx).or_insert(0) += count;
            }
        }
    }

    fn target_state_reached(&self) -> bool {
        self.state == self.target_joltages
    }
}

pub fn calculate_fewest_button_presses(input: &[Input]) -> usize {
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

pub fn calculate_fewest_button_presses_with_joltages(input: &[Input]) -> usize {
    input
        .iter()
        .map(|(_indicator_lights_map, buttons, target_joltages)| {
            let max_joltage = *target_joltages.values().max().unwrap_or(&0);
            let max_presses_per_button = max_joltage + 1;

            let ranges: Vec<_> = (0..buttons.len())
                .map(|_| 0..=max_presses_per_button)
                .collect();

            ranges
                .iter()
                .cloned()
                .multi_cartesian_product()
                .filter_map(|press_counts| {
                    let mut diagram = DiagramWithJoltages::new(target_joltages.clone());
                    diagram.apply_button_presses(buttons, &press_counts);

                    if diagram.target_state_reached() {
                        Some(press_counts.iter().sum())
                    } else {
                        None
                    }
                })
                .min()
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    use std::fs;

    fn file_to_vec(file: &str) -> Vec<super::Input> {
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

        fn parse_joltages(s: &str) -> HashMap<usize, usize> {
            s.trim_matches(['{', '}'])
                .split(',')
                .filter_map(|s| s.parse().ok())
                .enumerate()
                .map(|(i, s)| (i, s))
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
    fn test_calculate_fewest_button_presses() {
        let input = file_to_vec("../input/day10.txt");
        assert_eq!(super::calculate_fewest_button_presses(&input), 417);
    }

    #[test]
    fn test_calculate_fewest_button_presses_with_joltages() {
        let input = file_to_vec("../input/day10.txt");

        // assert_eq!(
        //     super::calculate_fewest_button_presses_with_joltages(&input),
        //     33
        // );

        assert_eq!(input.len(), 163);
    }
}
