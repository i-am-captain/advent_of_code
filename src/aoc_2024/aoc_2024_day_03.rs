use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let input = input::load_file("2024", "03");

    let result = process_1(sample_input);
    assert_eq!(result, 161);

    let result = process_1(&input);
    assert_eq!(result, 192767529);

    let sample_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let result = process_2(sample_input);
    assert_eq!(result, 48);

    let result = process_2(&input);
    assert_eq!(result, 104083373);
}

fn process_1(input: &str) -> i64 {
    let mut state = 0;

    let mut number: i64 = 0;

    let mut mult1 = 0;

    let mut result = 0;

    for c in input.chars() {
        match c {
            'm' if state == 0 => state += 1,
            'u' if state == 1 => state += 1,
            'l' if state == 2 => state += 1,
            '(' if state == 3 => state += 1,
            '0'..='9' if state == 4 => {
                number = (number * 10) + c.to_digit(10).unwrap_or(0) as i64;
            }
            ',' if state == 4 => {
                mult1 = number;
                number = 0;
                state += 1;
            }
            '0'..='9' if state == 5 => {
                number = (number * 10) + c.to_digit(10).unwrap_or(0) as i64;
            }
            ')' if state == 5 => {
                result += mult1 * number;

                state = 0;
                number = 0;
                mult1 = 0;
            }
            _ => {
                state = 0;
                number = 0;
                mult1 = 0;
            }
        }
    }

    result
}

fn process_2(input: &str) -> i64 {
    let mut state = 0;

    let mut number: i64 = 0;

    let mut mult1 = 0;

    let mut result = 0;

    let mut enabled = true;

    for c in input.chars() {
        match c {
            'm' => state = 1,
            'u' if state == 1 => state += 1,
            'l' if state == 2 => state += 1,
            '(' if state == 3 => state += 1,
            '0'..='9' if state == 4 => {
                number = (number * 10) + c.to_digit(10).unwrap_or(0) as i64;
            }
            ',' if state == 4 => {
                mult1 = number;
                number = 0;
                state += 1;
            }
            '0'..='9' if state == 5 => {
                number = (number * 10) + c.to_digit(10).unwrap_or(0) as i64;
            }
            ')' if state == 5 => {
                if enabled {
                    result += mult1 * number;
                }

                state = 0;
                number = 0;
                mult1 = 0;
            }

            // There are no overlapping elements in mul(x,y) and do() / don't(). So we can simply offset the state, to not have to implement a complex lexer.
            'd' => state = 100,
            'o' if state == 100 => state = 101,
            '(' if state == 101 => state = 102,
            ')' if state == 102 => {
                enabled = true;
                state = 0;
            }

            'n' if state == 101 => state = 110,
            '\'' if state == 110 => state = 111,
            't' if state == 111 => {
                enabled = false;
                state = 0;
            }
            _ => {
                state = 0;
                number = 0;
                mult1 = 0;
            }
        }
    }

    result
}
