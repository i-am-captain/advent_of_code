use std::{cmp::Ordering, collections::HashMap};

use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let input = input::load_file("2024", "05");

    let result = process_1(sample_input);
    assert_eq!(result, 143);

    let result = process_1(&input);
    assert_eq!(result, 7024);

    let result = process_2(sample_input);
    assert_eq!(result, 123);

    let result = process_2(&input);
    assert_eq!(result, 4151);
}

fn process_1(input: &str) -> i64 {
    let split: Vec<&str> = input.split("\n\n").collect();

    let rules = *split.first().unwrap_or(&"");
    let prints = *split.last().unwrap_or(&"");

    // not sure if ruleset always creates a linear sortable line of values, so use more flexible value mapping.

    // key must come before value
    let mut before_map: HashMap<&str, Vec<&str>> = HashMap::new();
    // key must come afer value
    let mut after_map: HashMap<&str, Vec<&str>> = HashMap::new();

    rules.split("\n").for_each(|line| {
        let split: Vec<&str> = line.split("|").collect();

        let before = *split.first().unwrap_or(&"");
        let after = *split.last().unwrap_or(&"");

        before_map.entry(before).or_default().push(after);
        after_map.entry(after).or_default().push(before);
    });

    let sum: i64 = prints
        .split("\n")
        .map(|line| {
            let pages: Vec<&str> = line.split(",").collect();

            let valid = pages.windows(2).all(|window| {
                let before = window[0];
                let after = window[1];
                // here could be && or || depending on the implicit rules.
                // If the rules are purely linear and every rule has it's opposite also listed, use &&
                // If rules can be circular or might not be fully complete, use ||
                // If rules contradict, were focked either way.
                // -> Tested both, same result. So rules are linear and complete. So use && just because.
                before_map
                    .get(before)
                    .map(|values| values.contains(&after))
                    .unwrap_or(false)
                    && after_map
                        .get(after)
                        .map(|values| values.contains(&before))
                        .unwrap_or(false)
            });

            if valid {
                return pages
                    .get(pages.len() / 2)
                    .and_then(|page| page.parse().ok())
                    .unwrap_or(0);
            }
            0
        })
        .sum();

    sum
}

fn process_2(input: &str) -> i64 {
    let split: Vec<&str> = input.split("\n\n").collect();

    let rules = *split.first().unwrap_or(&"");
    let prints = *split.last().unwrap_or(&"");

    // not sure if ruleset always creates a linear sortable line of values, so use more flexible value mapping.

    // key must come before value
    let mut before_map: HashMap<&str, Vec<&str>> = HashMap::new();
    // key must come afer value
    let mut after_map: HashMap<&str, Vec<&str>> = HashMap::new();

    rules.split("\n").for_each(|line| {
        let split: Vec<&str> = line.split("|").collect();

        let before = *split.first().unwrap_or(&"");
        let after = *split.last().unwrap_or(&"");

        before_map.entry(before).or_default().push(after);
        after_map.entry(after).or_default().push(before);
    });

    let sum: i64 = prints
        .split("\n")
        .map(|line| {
            let pages: Vec<&str> = line.split(",").collect();

            let valid = pages.windows(2).all(|window| {
                let before = window[0];
                let after = window[1];
                // here could be && or || depending on the implicit rules.
                // If the rules are purely linear and every rule has it's opposite also listed, use &&
                // If rules can be circular or might not be fully complete, use ||
                // If rules contradict, were focked either way.
                // -> Tested both, same result. So rules are linear and complete. So use && just because.
                before_map
                    .get(before)
                    .map(|values| values.contains(&after))
                    .unwrap_or(false)
                    && after_map
                        .get(after)
                        .map(|values| values.contains(&before))
                        .unwrap_or(false)
            });

            if !valid {
                // reorder here

                let mut sorted = pages.to_vec();
                sorted.sort_by(|left, right| {
                    if left == right {
                        return Ordering::Equal;
                    }
                    // again, if rules are complete, we can use &&. left is &self, so if it fits, it is lesser. Otherwise greater.
                    if before_map
                        .get(left)
                        .map(|values| values.contains(right))
                        .unwrap_or(false)
                        || after_map
                            .get(right)
                            .map(|values| values.contains(left))
                            .unwrap_or(false)
                    {
                        return Ordering::Less;
                    }
                    Ordering::Greater
                });

                return sorted
                    .get(sorted.len() / 2)
                    .and_then(|page| page.parse().ok())
                    .unwrap_or(0);
            }
            0
        })
        .sum();

    sum
}
