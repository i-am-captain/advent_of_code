use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let input = input::load_file("2024", "06");

    let result = process_1(sample_input);
    assert_eq!(result, 41);

    let result = process_1(&input);
    assert_eq!(result, 5030);

    let result = process_2(sample_input);
    assert_eq!(result, 6);

    let result = process_2(&input);
    assert_eq!(result, 1928);
}

#[derive(Debug, Clone)]
struct Map {
    fields: Vec<Vec<char>>,
}
#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i64,
    y: i64,
    direction: Direction,
}
#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Map {
    fn parse(input: &str) -> Map {
        let fields: Vec<Vec<char>> = input
            .split("\n")
            .map(|line| line.chars().collect())
            .collect();
        Map { fields }
    }

    fn get_guard(&self) -> Position {
        for (y, line) in self.fields.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if field == &'^' {
                    return Position {
                        x: x as i64,
                        y: y as i64,
                        direction: Direction::Up,
                    };
                }
            }
        }

        Position::new(0, 0, Direction::Up)
    }

    fn is_free(&self, x: i64, y: i64) -> bool {
        if self.is_out_of_bounds(x, y) {
            // oob is allowed and expected
            return true;
        }

        let x = x as usize;
        let y = y as usize;
        self.fields
            .get(y)
            .and_then(|line| line.get(x))
            .map(|c| c != &'#')
            .unwrap_or(false)
    }

    fn is_out_of_bounds(&self, x: i64, y: i64) -> bool {
        if y < 0
            || y >= self.fields.len() as i64
            || x < 0
            || x >= self.fields[y as usize].len() as i64
        {
            // out of bounds is allowed and expected
            return true;
        }
        false
    }

    fn clear_guard(&mut self, position: &Position) {
        let x = position.x;
        let y = position.y;

        if self.is_out_of_bounds(x, y) {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(line) = self.fields.get_mut(y) {
            line[x] = 'X';
        };
    }

    fn set_guard(&mut self, position: &Position) {
        let x = position.x;
        let y = position.y;

        if self.is_out_of_bounds(x, y) {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(line) = self.fields.get_mut(y) {
            line[x] = '^';
        };
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for line in self.fields.iter() {
            for c in line.iter() {
                s.push(*c);
            }
            s.push('\n');
        }
        s.push('\n');
        f.write_str(s.as_str())
    }
}

impl Position {
    fn new(x: i64, y: i64, direction: Direction) -> Position {
        Position { x, y, direction }
    }

    fn move_step(&mut self, map: &mut Map) {
        let (mut new_x, mut new_y) = self.coord_in_direction();

        while !map.is_free(new_x, new_y) {
            self.turn_right();
            (new_x, new_y) = self.coord_in_direction();
        }

        map.clear_guard(self);

        self.x = new_x;
        self.y = new_y;

        map.set_guard(self);
    }

    fn coord_in_direction(&self) -> (i64, i64) {
        let mut new_x = self.x;
        let mut new_y = self.y;

        match self.direction {
            Direction::Left => new_x = self.x - 1,
            Direction::Right => new_x = self.x + 1,
            Direction::Up => new_y = self.y - 1,
            Direction::Down => new_y = self.y + 1,
        }
        (new_x, new_y)
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
        }
    }

    fn is_out(&self, map: &Map) -> bool {
        map.is_out_of_bounds(self.x, self.y)
    }
}

fn process_1(input: &str) -> i64 {
    let mut map = Map::parse(input);
    let mut guard = map.get_guard();

    while !guard.is_out(&map) {
        guard.move_step(&mut map);
    }

    map.fields
        .iter()
        .map(|line| line.iter().filter(|c| c == &&'X').count() as i64)
        .sum()
}

fn process_2(input: &str) -> i64 {
    let mut map = Map::parse(input);
    let mut original_guard = map.get_guard();

    let mut loop_count = 0;

    let loop_count: usize = map
        .fields
        .par_iter()
        .enumerate()
        .map(|(y, line)| {
            let sum: usize = line
                .par_iter()
                .enumerate()
                .map(|(x, c)| check_map(&map, &original_guard, y, x))
                .sum();
            sum
        })
        .sum();

    println!();
    loop_count as i64
}

fn check_map(map: &Map, original_guard: &Position, y: usize, x: usize) -> usize {
    // clone map and guard to allow parallel cehcking
    let mut map = map.clone();
    let mut guard = original_guard.clone();

    let old_c = map.fields[y][x];

    if old_c == '^' {
        return 0;
    }

    map.fields[y][x] = '#';

    let mut loop_detected = false;
    // a bit more complex history map than a simple vec.
    let mut history: HashMap<usize, HashMap<usize, Vec<Position>>> = HashMap::new();
    while !guard.is_out(&map) {
        let guard_x = guard.x as usize;
        let guard_y = guard.y as usize;

        history
            .entry(guard_y)
            .or_default()
            .entry(guard_x)
            .or_default()
            .push(guard.clone());

        guard.move_step(&mut map);

        let guard_x = guard.x as usize;
        let guard_y = guard.y as usize;

        if history
            .get(&guard_y)
            .and_then(|map| map.get(&guard_x))
            .map(|vec| vec.contains(&guard))
            .unwrap_or(false)
        {
            // loop detected
            loop_detected = true;
            break;
        }
    }

    map.fields[y][x] = old_c;

    if loop_detected {
        return 1;
    }
    0
}
