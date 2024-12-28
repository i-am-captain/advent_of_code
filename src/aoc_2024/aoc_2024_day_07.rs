use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let input = input::load_file("2024", "07");

    let result = process_1(sample_input);
    assert_eq!(result, 3749);

    let result = process_1(&input);
    assert_eq!(result, 945512582195);

    let result = process_2(sample_input);
    assert_eq!(result, 11387);

    let result = process_2(&input);
    assert_eq!(result, 271691107779347);
}

#[test]
pub fn test_operator_generator() {
    // 5 values, two operators
    let mut operator_cg = OperatorCombinationGenerator::new(vec![0; 5], 2);

    let count = operator_cg.inspect(|ops| println!("{:?}", ops)).count();
    assert_eq!(count, 2_usize.pow(5));

    // 4 values, 3 operators
    let mut operator_cg = OperatorCombinationGenerator::new(vec![0; 4], 3);

    let count = operator_cg.inspect(|ops| println!("{:?}", ops)).count();
    assert_eq!(count, 3_usize.pow(4));
}

fn process_1(input: &str) -> i64 {
    let result: i64 = input
        .split("\n")
        .map(|line| {
            let line_split: Vec<&str> = line.split_whitespace().collect();

            if line_split.len() < 2 {
                return 0;
            }

            let expected_result: i64 = line_split
                .first()
                .and_then(|s| s[0..s.len() - 1].parse().ok())
                .unwrap_or(0);

            let values: Vec<i64> = line_split[1..]
                .iter()
                .flat_map(|s| s.parse().ok())
                .collect();

            let operators = vec!["+", "*"];

            let found_combinations =
                count_operator_combinations(values, operators, expected_result);

            if found_combinations > 0 {
                return expected_result;
            }
            0
        })
        .sum();
    result
}

fn process_2(input: &str) -> i64 {
    let result: i64 = input
        .split("\n")
        .map(|line| {
            let line_split: Vec<&str> = line.split_whitespace().collect();

            if line_split.len() < 2 {
                return 0;
            }

            let expected_result: i64 = line_split
                .first()
                .and_then(|s| s[0..s.len() - 1].parse().ok())
                .unwrap_or(0);

            let values: Vec<i64> = line_split[1..]
                .iter()
                .flat_map(|s| s.parse().ok())
                .collect();

            let operators = vec!["+", "*", "||"];

            let found_combinations =
                count_operator_combinations(values, operators, expected_result);

            if found_combinations > 0 {
                return expected_result;
            }
            0
        })
        .sum();
    result
}

struct OperatorCombinationGenerator {
    operator_selection: Vec<usize>,
    operator_count: usize,
    current_position: usize,
}
impl OperatorCombinationGenerator {
    fn new(operator_selection: Vec<usize>, operator_count: usize) -> OperatorCombinationGenerator {
        OperatorCombinationGenerator {
            operator_selection,
            operator_count,
            current_position: 0,
        }
    }
}
impl Iterator for OperatorCombinationGenerator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.current_position == 0 {
            // Special case for first element, return initial value without incresing anything, to also get [0, 0, 0, ...].
            self.current_position += 1;
            return Some(self.operator_selection.clone());
        }

        let base = self.operator_count;
        let mut add_next = 1;
        self.operator_selection
            .iter_mut()
            .enumerate()
            .for_each(|(i, sel_value)| {
                *sel_value += add_next;
                add_next = 0;
                if *sel_value >= base {
                    add_next = 1;
                    *sel_value = 0;
                }
            });
        if add_next > 0 {
            // We overlowed the operator selection vector, so we checked every possible combination. Stop searching.
            self.current_position += 1;
            return None;
        }
        // Don't care abot complex lending iterators or stuff, just clone the vector, it isn't that big.
        self.current_position += 1;
        Some(self.operator_selection.clone())
    }
}

fn count_operator_combinations(
    values: Vec<i64>,
    operators: Vec<&str>,
    expected_result: i64,
) -> i32 {
    let mut operator_cg = OperatorCombinationGenerator::new(vec![0; values.len()], operators.len());

    let mut search = true;
    let found_combinations = operator_cg
        .map(|operator_selection| {
            // check if current combination of operator selectors leads to ecpected result.
            let mut sum = 0;
            values.iter().enumerate().for_each(|(i, value)| {
                let op_index = operator_selection[i];
                let op = operators[op_index];

                // we could just use the op_index and delete the operators vec, but this way it is easier to follow.
                match op {
                    "+" => sum += value,
                    "*" => sum *= value,
                    // 12 || 345 = 12345, shift left value in base 10 to the left and add right value
                    "||" => sum = sum * 10_i64.pow(value.to_string().len() as u32) + value,
                    _ => (),
                }
            });

            if sum == expected_result {
                // count all combinations that lead to expected result
                return 1;
            }
            0
        })
        .sum();
    found_combinations
}
