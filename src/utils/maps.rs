use std::{collections::HashMap, fmt::Display};

static DEFAULT_CHUNK_SIZE: i64 = 20;

/// Overly complicated chunk based map implementation. Just because it's fun and to try it out.
#[derive(Debug, Clone, PartialEq)]
pub struct Map<T: Default + Display> {
    /// HashMap of HashMaps is probably not the fastest way to do that, but good enough for now.
    chunks: HashMap<i64, HashMap<i64, Chunk<T>>>,
    /// Use same type as for x and y for easier calculation.
    chunk_size: i64,
}
impl<T: Default + Display> Map<T> {
    fn new(chunk_size: i64) -> Map<T> {
        Map {
            chunks: HashMap::new(),
            chunk_size,
        }
    }

    fn create_chunk_mut(&mut self, x: i64, y: i64) -> &mut Chunk<T> {
        let chunk_x = x / self.chunk_size;
        let chunk_y = y / self.chunk_size;

        self.chunks
            .entry(chunk_y)
            .or_default()
            .entry(chunk_x)
            .or_insert(Chunk::new(chunk_x, chunk_y, self.chunk_size))
    }

    fn get_position_mut(&mut self, x: i64, y: i64) -> &mut Position<T> {
        // copy the variable, before self is mutably borrowed in create_chunk
        let chunk_size = self.chunk_size;
        let chunk = self.create_chunk_mut(x, y);

        let x_in_chunk = (x % chunk_size) as usize;
        let y_in_chunk = (y % chunk_size) as usize;
        chunk.get_position_mut(x_in_chunk, y_in_chunk)
    }

    pub fn parse(input: &str) -> Map<char> {
        Self::parse_cs(input, DEFAULT_CHUNK_SIZE)
    }

    fn parse_cs(input: &str, chunk_size: i64) -> Map<char> {
        let mut map: Map<char> = Map::new(DEFAULT_CHUNK_SIZE);

        input.split("\n").enumerate().for_each(|(y, line)| {
            let y: i64 = y as i64;
            line.chars().enumerate().for_each(|(x, ch)| {
                let x = x as i64;
                let position = map.get_position_mut(x, y);
                position.content = ch;
            });
        });
        map
    }

    fn is_out_of_bounds(&self, x: i64, y: i64) -> bool {
        false
    }
}

impl<T: Default + Display> Display for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        self.chunks.iter().for_each(|(y, row)| {
            row.iter().for_each(|(x, chunk)| {
                s.push_str("Chunk y: ");
                s.push_str(y.to_string().as_str());
                s.push_str(" x: ");
                s.push_str(x.to_string().as_str());
                s.push('\n');
                chunk.fields.iter().for_each(|line| {
                    line.iter().for_each(|pos| {
                        s.push_str(pos.content.to_string().as_str());
                    });
                    s.push('\n');
                });
            });
            s.push('\n');
        });
        s.push('\n');
        f.write_str(s.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Chunk<T: Default + Display> {
    chunk_x: i64,
    chunk_y: i64,
    fields: Vec<Vec<Position<T>>>,
}
impl<T: Default + Display> Chunk<T> {
    fn new(chunk_x: i64, chunk_y: i64, chunk_size: i64) -> Chunk<T> {
        let chunk_size_usize = chunk_size as usize;

        let mut y_vec: Vec<Vec<Position<T>>> = Vec::with_capacity(chunk_size_usize);
        (0..chunk_size_usize).for_each(|y_in_chunk| {
            y_vec.insert(y_in_chunk, Vec::with_capacity(chunk_size_usize));
            (0..chunk_size_usize).for_each(|x_in_chunk| {
                let x_pos = chunk_x * chunk_size + x_in_chunk as i64;
                let y_pos = chunk_y * chunk_size + y_in_chunk as i64;
                y_vec[y_in_chunk].insert(
                    x_in_chunk,
                    Position::new(x_pos, y_pos, x_in_chunk, y_in_chunk, T::default()),
                );
            });
        });

        Chunk {
            chunk_x,
            chunk_y,
            fields: y_vec,
        }
    }

    fn get_position_mut(&mut self, x: usize, y: usize) -> &mut Position<T> {
        &mut self.fields[y][x]
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Position<T: Default + Display> {
    x: i64,
    y: i64,
    x_in_chunk: usize,
    y_in_chunk: usize,
    content: T,
}
impl<T: Default + Display> Position<T> {
    fn new(x: i64, y: i64, x_in_chunk: usize, y_in_chunk: usize, content: T) -> Position<T> {
        Position {
            x,
            y,
            x_in_chunk,
            y_in_chunk,
            content,
        }
    }

    fn is_out(&self, map: &Map<T>) -> bool {
        map.is_out_of_bounds(self.x, self.y)
    }
}
