use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    let input = input::load_file("2024", "13");

    let result = process(sample_input, 0);
    assert_eq!(result, 480);

    let result = process(&input, 0);
    assert_eq!(result, 37901);

    let result = process(&input, 10000000000000);
    assert_eq!(result, 77407675412647);
}

fn process(input: &str, p_correction: i64) -> i64 {
    let sum: i64 = input
        .split("\n\n")
        .map(|block| {
            let lines: Vec<&str> = block.split("\n").collect();
            if lines.len() < 3 {
                return 0;
            }
            let axy: Vec<i64> = lines[0]
                .split("Button A: X+")
                .nth(1)
                .unwrap()
                .split(", Y+")
                .map(|s| s.parse().unwrap())
                .collect();
            let ax = axy[0];
            let ay = axy[1];
            let bxy: Vec<i64> = lines[1]
                .split("Button B: X+")
                .nth(1)
                .unwrap()
                .split(", Y+")
                .map(|s| s.parse().unwrap())
                .collect();
            let bx = bxy[0];
            let by = bxy[1];
            let pxy: Vec<i64> = lines[2]
                .split("Prize: X=")
                .nth(1)
                .unwrap()
                .split(", Y=")
                .map(|s| s.parse().unwrap())
                .collect();
            let px = pxy[0] + p_correction;
            let py = pxy[1] + p_correction;
            // a * ax + b * bx = px
            // a * ay + b * by = py
            // a and b are unknowns
            // a = (px - b * bx) / ax
            // ((px - b * bx) / ax) * ay + b * by = py
            // ((px - b * bx) / ax) + b * by / ay = py / ay
            // (px - b * bx) + b * by * ax / ay = py * ax / ay
            // px - b * bx + b * by * ax / ay = py * ax / ay
            // px + b * ((by * ax / ay) - bx) = py * ax / ay
            // b  = ((py * ax / ay) - px) / ((by * ax / ay) - bx)
            // b  = ((py * ax / ay) - px) / ((by * ax - bx * ay) / ay)
            // b  = ((py * ax / ay) - px) * ay / (by * ax - bx * ay)
            // b  = (py * ax - px * ay) / (by * ax - bx * ay)
            // If not cleanly integer divisible, solution does not work. It must be whole steps.
            // -> (py * ax - px * ay) % (by * ax - bx * ay) == 0
            // -> (px - b * bx) % ax == 0
            if (py * ax - px * ay) % (by * ax - bx * ay) != 0 {
                return 0;
            }
            let b = (py * ax - px * ay) / (by * ax - bx * ay);
            if (px - b * bx) % ax != 0 {
                return 0;
            }
            let a = (px - b * bx) / ax;
            a * 3 + b
        })
        .sum();
    sum
}
