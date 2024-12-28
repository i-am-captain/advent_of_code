use std::{collections::HashMap, fmt::Display};

use crate::{input, utils::maps::Map};

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let input = input::load_file("2024", "08");

    let result = process_1(sample_input);
    assert_eq!(result, 14);

    let result = process_1(&input);
    assert_eq!(result, 252);

    let result = process_2(sample_input);
    assert_eq!(result, 34);

    let result = process_2(&input);
    assert_eq!(result, 0);
}

#[derive(Debug, Clone, PartialEq)]
struct Element {
    x: i64,
    y: i64,
    ch: char,
}

fn process_1(input: &str) -> i64 {
    let fields: Vec<Vec<Element>> = input
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| Element {
                    x: x as i64,
                    y: y as i64,
                    ch,
                })
                .collect()
        })
        .filter(|v: &Vec<Element>| !v.is_empty())
        .collect();

    let y_limit = fields.len() as i64;
    let x_limit = fields[0].len() as i64;

    let mut ch_map: HashMap<char, Vec<&Element>> = HashMap::new();

    let mut updated_fields = fields.clone();

    fields.iter().for_each(|line| {
        line.iter().for_each(|element| {
            if element.ch != '.' {
                ch_map.entry(element.ch).or_default().push(element);
            }
        })
    });
    // fields.iter().for_each(|line| {
    //     line.iter().for_each(|element| print!("{}", element.ch));
    //     println!();
    // });

    let mut cross_elements: Vec<Element> = Vec::new();

    ch_map.iter().for_each(|(ch, elements)| {
        (0..elements.len()).for_each(|i| {
            let current_element = elements[i];
            ((i + 1)..elements.len()).for_each(|j| {
                let mirror_element = elements[j];
                let delta_x = mirror_element.x - current_element.x;
                let delta_y = mirror_element.y - current_element.y;
                let new_x_1 = current_element.x - delta_x;
                let new_y_1 = current_element.y - delta_y;
                let new_x_2 = mirror_element.x + delta_x;
                let new_y_2 = mirror_element.y + delta_y;
                if new_x_1 >= 0 && new_x_1 < x_limit && new_y_1 >= 0 && new_y_1 < y_limit {
                    let new_element = Element {
                        x: new_x_1,
                        y: new_y_1,
                        ch: '#',
                    };
                    if !cross_elements.contains(&new_element) {
                        cross_elements.push(new_element.clone());
                        updated_fields[new_y_1 as usize][new_x_1 as usize] = new_element.clone();
                    }
                }
                if new_x_2 >= 0 && new_x_2 < x_limit && new_y_2 >= 0 && new_y_2 < y_limit {
                    let new_element = Element {
                        x: new_x_2,
                        y: new_y_2,
                        ch: '#',
                    };
                    if !cross_elements.contains(&new_element) {
                        cross_elements.push(new_element.clone());
                        updated_fields[new_y_2 as usize][new_x_2 as usize] = new_element.clone();
                    }
                }
            });
        });
    });
    // updated_fields.iter().for_each(|line| {
    //     line.iter().for_each(|element| print!("{}", element.ch));
    //     println!();
    // });

    cross_elements.len() as i64
}

fn process_2(input: &str) -> i64 {
    let fields: Vec<Vec<Element>> = input
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| Element {
                    x: x as i64,
                    y: y as i64,
                    ch,
                })
                .collect()
        })
        .filter(|v: &Vec<Element>| !v.is_empty())
        .collect();

    let y_limit = fields.len() as i64;
    let x_limit = fields[0].len() as i64;

    let mut ch_map: HashMap<char, Vec<&Element>> = HashMap::new();

    let mut updated_fields = fields.clone();

    fields.iter().for_each(|line| {
        line.iter().for_each(|element| {
            if element.ch != '.' {
                ch_map.entry(element.ch).or_default().push(element);
            }
        })
    });
    fields.iter().for_each(|line| {
        line.iter().for_each(|element| print!("{}", element.ch));
        println!();
    });

    let mut cross_elements: Vec<Element> = Vec::new();

    ch_map.iter().for_each(|(ch, elements)| {
        (0..elements.len()).for_each(|i| {
            let current_element = elements[i];
            ((i + 1)..elements.len()).for_each(|j| {
                let mirror_element = elements[j];
                let delta_x = mirror_element.x - current_element.x;
                let delta_y = mirror_element.y - current_element.y;

                let mut new_x = current_element.x;

                let new_element = Element {
                    x: new_x,
                    y: current_element.y,
                    ch: '#',
                };
                if !cross_elements.contains(&new_element) {
                    cross_elements.push(new_element.clone());
                    updated_fields[current_element.y as usize][new_x as usize] =
                        new_element.clone();
                }

                while new_x >= 0 && new_x < x_limit - 1 {
                    new_x += 1;
                    let gradient_dividend = (new_x - current_element.x) * delta_y;

                    if gradient_dividend % delta_x != 0 {
                        // not on a direct line, because gradient is no integer division
                        continue;
                    }

                    let new_y = current_element.y + gradient_dividend / delta_x;

                    if new_y < 0 || new_y >= y_limit {
                        // y out of bounds
                        continue;
                    }

                    let new_element = Element {
                        x: new_x,
                        y: new_y,
                        ch: '#',
                    };
                    if !cross_elements.contains(&new_element) {
                        cross_elements.push(new_element.clone());
                        updated_fields[new_y as usize][new_x as usize] = new_element.clone();
                    }
                }
                let mut new_x = current_element.x;

                while new_x > 0 && new_x < x_limit {
                    new_x -= 1;
                    let gradient_dividend = (new_x - current_element.x) * delta_y;

                    if gradient_dividend % delta_x != 0 {
                        // not on a direct line, because gradient is no integer division
                        continue;
                    }

                    let new_y = current_element.y + gradient_dividend / delta_x;

                    if new_y < 0 || new_y >= y_limit {
                        // y out of bounds
                        continue;
                    }

                    let new_element = Element {
                        x: new_x,
                        y: new_y,
                        ch: '#',
                    };
                    if !cross_elements.contains(&new_element) {
                        cross_elements.push(new_element.clone());
                        updated_fields[new_y as usize][new_x as usize] = new_element.clone();
                    }
                }
            });
        });
    });
    updated_fields.iter().for_each(|line| {
        line.iter().for_each(|element| print!("{}", element.ch));
        println!();
    });

    cross_elements.len() as i64
}
