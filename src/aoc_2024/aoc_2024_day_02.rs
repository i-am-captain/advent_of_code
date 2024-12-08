use std::cmp::Ordering;

use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    let input = input::load_file("2024", "02");

    let result = process_1(sample_input);
    assert_eq!(result, 2);

    let result = process_1(&input);
    assert_eq!(result, 670);

    let result = process_2(sample_input);
    assert_eq!(result, 4);

    let result = process_2(&input);
    assert_eq!(result, 700);
}

fn process_1(input: &str) -> usize {
    let parsed_input: Vec<Vec<i64>> = input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .flat_map(|column| column.parse().ok())
                .collect()
        })
        .filter(|num_line: &Vec<i64>| !num_line.is_empty())
        .collect();

    parsed_input
        .iter()
        .filter(|line| {
            let mut sign;

            let first = line[0];
            let second = line[1];

            let diff = second - first;
            match diff.cmp(&0) {
                Ordering::Less => sign = -1,
                // If first two elements already don't change, we are immediately invalid.
                Ordering::Equal => return false,
                Ordering::Greater => sign = 1,
            }

            let mut last_num = first;
            for num in line[1..].iter() {
                let diff = sign * (num - last_num);
                if !(1..=3).contains(&diff) {
                    return false;
                }
                last_num = *num;
            }
            true
        })
        .count()
}

fn process_2(input: &str) -> usize {
    let parsed_input: Vec<Vec<i64>> = input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .flat_map(|column| column.parse().ok())
                .collect()
        })
        .filter(|num_line: &Vec<i64>| !num_line.is_empty())
        .collect();

    parsed_input
        .iter()
        .filter(|line| {
            // default case
            let (mut valid, mut index) = is_line_valid(line);

            if !valid {
                // check simply removing current index
                let mut vec_rem_index = line.to_vec();
                vec_rem_index.remove(index);
                valid = is_line_valid(&&vec_rem_index).0;
            }
            if !valid && index > 0 {
                // check left edge cases
                let mut vec_rem_index = line.to_vec();
                vec_rem_index.remove(index - 1);
                valid = is_line_valid(&&vec_rem_index).0;
            }
            if !valid && index < line.len() {
                // check right edge cases
                let mut vec_rem_index = line.to_vec();
                vec_rem_index.remove(index + 1);
                valid = is_line_valid(&&vec_rem_index).0;
            }
            valid
        })
        .count()
}

fn is_line_valid(line: &&Vec<i64>) -> (bool, usize) {
    let mut last_num = line[0];
    let mut sign = get_sign(line[1], line[0]);
    let mut start_index = 1;

    for (index, num) in line[start_index..].iter().enumerate() {
        let diff = sign * (num - last_num);
        if !(1..=3).contains(&diff) {
            return (false, index);
        }
        last_num = *num;
    }
    (true, 0)
}

fn get_sign(second: i64, first: i64) -> i64 {
    let diff = second - first;
    match diff.cmp(&0) {
        Ordering::Less => -1,
        // If first two elements already don't change, we are immediately invalid. But 0 works as well for the calculation later and make filtering easier.
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}
