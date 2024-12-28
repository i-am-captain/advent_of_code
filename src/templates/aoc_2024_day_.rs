use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "";

    let input = input::load_file("2024", "01");

    let result = process_1(&sample_input);
    assert_eq!(result, 0);

    let result = process_1(&input);
    assert_eq!(result, 0);

    let result = process_2(&sample_input);
    assert_eq!(result, 0);

    let result = process_2(&input);
    assert_eq!(result, 0);
}

fn process_1(input: &str) -> i64 {
    0
}

fn process_2(input: &str) -> i64 {
    0
}
