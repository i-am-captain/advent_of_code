use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let input = input::load_file("2024", "04");

    let result = process_1(sample_input);
    assert_eq!(result, 18);

    let result = process_1(&input);
    assert_eq!(result, 2547);

    let result = process_2(sample_input);
    assert_eq!(result, 9);

    let result = process_2(&input);
    assert_eq!(result, 1939);
}

fn process_1(input: &str) -> i64 {
    // create matrix
    let matrix: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            // first look for Xs
            if *c == 'X' {
                // search right
                count += search(&matrix, x, y, 1, 0);
                // search right down
                count += search(&matrix, x, y, 1, 1);
                // search down
                count += search(&matrix, x, y, 0, 1);
                // search left down
                count += search(&matrix, x, y, -1, 1);
                // search left
                count += search(&matrix, x, y, -1, 0);
                // search left up
                count += search(&matrix, x, y, -1, -1);
                // search up
                count += search(&matrix, x, y, 0, -1);
                // search right up
                count += search(&matrix, x, y, 1, -1);
            }
        }
    }

    count
}

fn search(matrix: &[Vec<char>], x: usize, y: usize, x_direction: i64, y_direction: i64) -> i64 {
    let search_value: Vec<char> = "XMAS".chars().collect();

    // match all enumerated chars of the search_value against the matrix,
    // given the search index and direction added to the starting x and y positions.
    // Using iterators only for extra style.
    let found = search_value.iter().enumerate().all(|(index, search_char)| {
        let x_value = x as i64 + index as i64 * x_direction;
        let y_value = y as i64 + index as i64 * y_direction;

        if x_value < 0 || y_value < 0 {
            return false;
        }

        let x_value = x_value as usize;
        let y_value = y_value as usize;

        matrix
            .get(y_value)
            .and_then(|line| line.get(x_value))
            .map(|c| c == search_char)
            .unwrap_or(false)
    });

    if found {
        return 1;
    }
    0
}

fn process_2(input: &str) -> i64 {
    // create matrix
    let matrix: Vec<Vec<char>> = input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            // cannot underflow usize
            let x_neg = x as i64 - 1;
            let y_neg = y as i64 - 1;
            if x_neg < 0 || y_neg < 0 {
                continue;
            }

            // first look for Xs
            if *c == 'A' {
                let left_up = *matrix
                    .get(y - 1)
                    .and_then(|line| line.get(x - 1))
                    .unwrap_or(&'F');

                let right_up = *matrix
                    .get(y - 1)
                    .and_then(|line| line.get(x + 1))
                    .unwrap_or(&'F');

                let right_down = *matrix
                    .get(y + 1)
                    .and_then(|line| line.get(x + 1))
                    .unwrap_or(&'F');

                let left_down = *matrix
                    .get(y + 1)
                    .and_then(|line| line.get(x - 1))
                    .unwrap_or(&'F');

                let found1 =
                    left_up == 'M' && right_down == 'S' || left_up == 'S' && right_down == 'M';

                let found2 =
                    right_up == 'M' && left_down == 'S' || right_up == 'S' && left_down == 'M';

                if found1 && found2 {
                    count += 1;
                }
            }
        }
    }
    count
}
