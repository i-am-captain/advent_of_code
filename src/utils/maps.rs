use std::fmt::Display;

#[derive(Debug, Clone)]
struct Map {
    fields: Vec<Vec<Position>>,
}
impl Map {
    fn parse(input: &str) -> Map {
        let fields: Vec<Vec<Position>> = input
            .split("\n")
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| Position { x, y, ch })
                    .collect()
            })
            .collect();
        Map { fields }
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
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for line in self.fields.iter() {
            for c in line.iter() {
                s.push(c.ch);
            }
            s.push('\n');
        }
        s.push('\n');
        f.write_str(s.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i64,
    y: i64,
    ch: char,
}
impl Position {
    fn new(x: i64, y: i64, ch: char) -> Position {
        Position { x, y, ch }
    }

    fn is_out(&self, map: &Map) -> bool {
        map.is_out_of_bounds(self.x, self.y)
    }
}
