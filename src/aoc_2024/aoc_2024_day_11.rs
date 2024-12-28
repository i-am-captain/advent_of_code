use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "125 17";

    let input = input::load_file("2024", "11");

    let result = process_1(sample_input);
    assert_eq!(result, 55312);

    let result = process_1(&input);
    assert_eq!(result, 186424);

    let result = process_1_by_recursion(&input);
    assert_eq!(result, 186424);

    let result = process_2(&input);
    // This is the number of elements in the potential array.
    // It is not possible to calculate it by expanding an array like in the first method.
    assert_eq!(result, 219838428124832);
}

fn process_1(input: &str) -> usize {
    let mut data: Vec<String> = input
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect();
    for i in 0..25 {
        data = iterate_once(&data);
    }
    data.len()
}

fn process_1_by_recursion(input: &str) -> u64 {
    let mut data: Vec<String> = input
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect();

    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let sum: u64 = data
        .iter()
        .map(|s| calculate_number_recursively(s.parse().unwrap_or(0), 25, &mut HashMap::new()))
        .sum();
    sum
}

fn process_2(input: &str) -> u64 {
    let mut data: Vec<String> = input
        .split_ascii_whitespace()
        .map(|s| s.to_owned())
        .collect();

    let mut cache_hits = 0;
    let mut cache_misses = 0;
    let sum: u64 = data
        .iter()
        .map(|s| calculate_number_recursively(s.parse().unwrap_or(0), 75, &mut HashMap::new()))
        .sum();
    sum
}

fn calculate_number_recursively(
    number: u64,
    steps: u64,
    cache: &mut HashMap<u64, HashMap<u64, u64>>,
) -> u64 {
    if steps == 0 {
        return 1;
    }
    // Use the cache to stop any recursion tree, that has already been done.
    // value * 2024 will happen quite often for the same value, especially in later steps,
    // when the number of elements grows by being split continously.
    // So the cache improves runtime significantly.
    if cache.contains_key(&steps) {
        let steps_cache = cache.get(&steps).unwrap();
        if steps_cache.contains_key(&number) {
            return *steps_cache.get(&number).unwrap();
        }
    }
    let sum = match number {
        0 => calculate_number_recursively(1, steps - 1, cache),
        n if n.to_string().len() % 2 == 0 => {
            let (left, right) = split_number(n);
            calculate_number_recursively(left, steps - 1, cache)
                + calculate_number_recursively(right, steps - 1, cache)
        }
        n => calculate_number_recursively(n * 2024, steps - 1, cache),
    };
    cache.entry(steps).or_default().insert(number, sum);
    sum
}

fn split_number(number: u64) -> (u64, u64) {
    let s = number.to_string();
    let left: u64 = s[0..s.len() / 2].parse().unwrap_or(0);
    let right: u64 = s[s.len() / 2..s.len()].parse().unwrap_or(0);
    (left, right)
}

fn iterate_once(data: &[String]) -> Vec<String> {
    let data: Vec<String> = data
        .par_iter()
        .filter(|s| !s.is_empty())
        .flat_map(|s| {
            if s == "0" {
                return vec!["1".to_owned()];
            }
            if s.len() % 2 == 0 {
                let len = s.len();
                let half_len = len / 2;
                let left = s[0..half_len].to_owned();
                let right = &s[half_len..len];
                // remove leading 0s. E.g. 012 -> 12
                let right = right
                    .parse::<u64>()
                    .map(|n| n.to_string())
                    .unwrap_or("0".to_owned());
                return vec![left, right];
            }
            let new_s = s
                .parse::<u64>()
                .map(|n| n * 2024)
                .map(|n| n.to_string())
                .unwrap_or("".to_owned());

            vec![new_s]
        })
        .collect();
    data
}
