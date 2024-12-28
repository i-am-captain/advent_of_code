use std::collections::HashSet;

use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    let input = input::load_file("2024", "10");

    let result = process_1(sample_input);
    assert_eq!(result, 36);

    let result = process_1(&input);
    assert_eq!(result, 611);

    let result = process_2(sample_input);
    assert_eq!(result, 81);

    let result = process_2(&input);
    assert_eq!(result, 1380);
}

fn process_1(input: &str) -> i64 {
    let map: Vec<Vec<char>> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, ch)| {
                    if *ch == '0' {
                        let step_lists = find_step_lists(x, y, &map, '0');
                        // part 1 wants only the number of unique 9s, that can be reached from each 0
                        let set: HashSet<Position> = step_lists
                            .iter()
                            .filter_map(|list| list.last())
                            .cloned()
                            .collect();
                        // println!("{:?}", step_lists);
                        return set.len() as i64;
                    }
                    0
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}
fn process_2(input: &str) -> i64 {
    let map: Vec<Vec<char>> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, ch)| {
                    if *ch == '0' {
                        let step_lists = find_step_lists(x, y, &map, '0');
                        // as expected, the second part wants the number of unique paths from each 0 to each 9.
                        // Which we already have, by calculating the whole tree recursively
                        // and flattened into a list of unique list of steps (= paths)
                        return step_lists.len() as i64;
                    }
                    0
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
    ch: char,
}

fn find_step_forward(
    old_char: char,
    new_x: usize,
    new_y: usize,
    map: &[Vec<char>],
) -> Vec<Vec<Position>> {
    let new_char = map[new_y][new_x];

    let next_char: char = old_char
        .to_digit(10)
        .map(|d| d + 1)
        .and_then(|d| char::from_digit(d, 10))
        .unwrap_or('E');

    if next_char != new_char {
        // cannot move in this direction, stop recursion, this path will be dropped and not added to the final list of step lists
        return Vec::new();
    }

    if new_char == '9' {
        // end reached, collapse recursion to a list of steps to this position.
        // This will cause the lists of steps to be built from 9 down to the beginning 0
        return vec![vec![Position {
            x: new_x,
            y: new_y,
            ch: new_char,
        }]];
    }

    find_step_lists(new_x, new_y, map, new_char)
}

fn find_step_lists(
    old_x: usize,
    old_y: usize,
    map: &[Vec<char>],
    old_char: char,
) -> Vec<Vec<Position>> {
    let mut next_moves: Vec<Vec<Position>> = Vec::new();
    // up
    if old_y > 0 {
        let mut step_lists = find_step_forward(old_char, old_x, old_y - 1, map);
        for mut list in step_lists {
            if !list.is_empty() {
                // only continue with the path, if something is included, i.e. we reached '9' at some point
                next_moves.push(list);
            }
        }
    }
    // down
    if old_y < map.len() - 1 {
        let mut step_lists = find_step_forward(old_char, old_x, old_y + 1, map);
        for mut list in step_lists {
            if !list.is_empty() {
                next_moves.push(list);
            }
        }
    }
    // left
    if old_x > 0 {
        let mut step_lists = find_step_forward(old_char, old_x - 1, old_y, map);
        for mut list in step_lists {
            if !list.is_empty() {
                next_moves.push(list);
            }
        }
    }
    // right
    if old_x < map[0].len() - 1 {
        let mut step_lists = find_step_forward(old_char, old_x + 1, old_y, map);
        for mut list in step_lists {
            if !list.is_empty() {
                next_moves.push(list);
            }
        }
    }

    for mut list in next_moves.iter_mut() {
        // For all paths, that were identified to reach 9, add the current char, to extend the path.
        if !list.is_empty() {
            list.insert(
                0,
                Position {
                    x: old_x,
                    y: old_y,
                    ch: old_char,
                },
            );
        }
    }

    next_moves
}
