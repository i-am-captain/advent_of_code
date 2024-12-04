use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let input = input::load_file("2024", "01");

    let result = process_1(sample_input);
    assert_eq!(result, 11);

    let result = process_1(&input);
    assert_eq!(result, 1722302);

    let result = process_2(sample_input);
    assert_eq!(result, 31);

    let result = process_2(&input);
    assert_eq!(result, 20373490);
}

fn process_1(input: &str) -> i64 {
    let parsed_input: Vec<Vec<&str>> = input
        .split("\n")
        .map(|line| line.split(" ").collect())
        .collect();

    let mut left: Vec<i64> = parsed_input
        .iter()
        .filter_map(|row| row.first())
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut right: Vec<i64> = parsed_input
        .iter()
        .filter_map(|row| row.last())
        .filter_map(|s| s.parse().ok())
        .collect();
    left.sort();
    right.sort();

    let mut count = 0;
    for i in 0..left.len() {
        count += (left[i] - right[i]).abs();
    }
    count
}

fn process_2(input: &str) -> i64 {
    let parsed_input: Vec<Vec<&str>> = input
        .split("\n")
        .map(|line| line.split(" ").collect())
        .collect();

    let mut left: Vec<i64> = parsed_input
        .iter()
        .filter_map(|row| row.first())
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut right: Vec<i64> = parsed_input
        .iter()
        .filter_map(|row| row.last())
        .filter_map(|s| s.parse().ok())
        .collect();
    left.sort();
    right.sort();

    let mut count = 0;
    let mut last_right_index = 0;
    for lv in left.iter() {
        let mut times = 0;
        let mut current_right_index = 0;
        'rl: for (ri, rv) in right[last_right_index..].iter().enumerate() {
            if lv == rv {
                times += 1;
            } else if times > 0 {
                current_right_index = ri;
                // since the lists are sorted, if we are past equality in the right list, we can break out, to save some search time
                break 'rl;
            }
        }
        if current_right_index > times {
            // since the lists are sorted, we can start searching in the right list from the latest known position of the currently checked value. Simply subtracting times may overshoot a bit but that does not matter
            last_right_index = current_right_index - times;
        }

        let times = times as i64;
        count += (lv * times).abs();
    }

    count
}
