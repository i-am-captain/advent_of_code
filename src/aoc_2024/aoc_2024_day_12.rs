use colored::{ColoredString, Colorize};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::collections::HashMap;

use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let input = input::load_file("2024", "12");

    let sample_input = "AAAA
BBCD
BBCC
EEEC";

    let result = process_1(sample_input);
    assert_eq!(result, 140);

    let result = process_2(sample_input);
    assert_eq!(result, 80);

    let sample_input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    let result = process_1(sample_input);
    assert_eq!(result, 772);

    let result = process_2(sample_input);
    assert_eq!(result, 436);

    let sample_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    let result = process_1(sample_input);
    assert_eq!(result, 1930);

    let result = process_1(&input);
    assert_eq!(result, 1400386);
    return;

    // let result = process_2(sample_input);
    // assert_eq!(result, 0);

    // let result = process_2(&input);
    // assert_eq!(result, 0);
}

fn process_1(input: &str) -> usize {
    let map = parse_map(input);

    // Regions are created, now try calculating perimeter

    let sum: usize = map
        .iter()
        .map(|(key, regions)| {
            regions
                .iter()
                .map(|region| region.calulate_border().len() * region.points.len())
                .sum::<usize>()
        })
        .sum();

    // print_regions(&map);

    sum
}

fn process_2(input: &str) -> i64 {
    0
}

fn parse_map(input: &str) -> HashMap<char, Vec<Region>> {
    // Recursively searching for neighbors (DFS) would probably be faster and simpler, but i want to try something different here. Declare each element as region and then merge regions of same kind.
    let mut map: HashMap<char, Vec<Region>> = HashMap::new();

    // Parse all elements into separate regions.
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                let region = Region::new(ch, Point::new_usize(x, y, ch));
                map.entry(ch).or_default().push(region);
            });
        });

    map.iter_mut().for_each(|(key, mut regions)| {
        // repeat merge as long as there is something to merge. If latest iteration performed no merge, finish.
        // iterate from last element, to remove like from a stack. Each element that is removed, is added to the end of the vec at the end. This reorders the whole vec, but since we go backwards, it is fine.
        let mut i = regions.len();
        while i > 0 {
            // subtract first, to be able to stop at 0, so also start at len() instead of len() -1
            i -= 1;

            // need to remove the region, to make borrow checker happy. Maybe not the best solution.
            let mut region = regions.remove(i);
            // check removed region against all remaining ones
            let mut j = regions.len();
            let mut something_merged = false;
            while j > 0 {
                // same as for i
                j -= 1;
                let mut other_region = regions.remove(j);
                if region.can_merge(&other_region) {
                    // merge, do not add back to stack
                    region.merge(other_region);
                    something_merged = true;
                } else {
                    // cannot merge, add back to stack
                    regions.push(other_region);
                }
            }
            // add the previously removed region back. It might have multiple regions merged.
            regions.push(region);

            if something_merged {
                // something was merged, go for another loop around, by resetting i to the full length
                // because both the i iteration and the j iteration reorder the vec, we cannot simply just get the "next" i value.
                // i can only count completey to 0, if nothing is merged anymore
                i = regions.len();
            }
        }
    });
    map
}

fn print_regions(map: &HashMap<char, Vec<Region>>) {
    // print all regions for debuging

    // always get the same colors
    let mut random: StdRng = StdRng::seed_from_u64(42);

    // first allign them in a indexed map again
    let mut hash_map: HashMap<i64, HashMap<i64, ColoredString>> = HashMap::new();
    map.iter().for_each(|(key, regions)| {
        regions.iter().for_each(|region| {
            println!(
                "Region: {} border: {} area: {}",
                region.ch,
                region.calulate_border().len(),
                region.points.len()
            );

            let red = (random.next_u64() % 255) as u8;
            let green = (random.next_u64() % 255) as u8;
            let blue = (random.next_u64() % 255) as u8;
            let region_string = region.ch.to_string().custom_color((red, green, blue));
            region.points.iter().for_each(|point| {
                hash_map
                    .entry(point.y)
                    .or_default()
                    .entry(point.x)
                    .insert_entry(region_string.clone());
            })
        })
    });

    // sort indices and print with color
    let mut ys: Vec<&i64> = hash_map.keys().collect();
    ys.sort();
    for y in ys {
        let line = hash_map.get(y).unwrap();
        let mut xs: Vec<&i64> = line.keys().collect();
        xs.sort();
        for x in xs {
            let s = line.get(x).unwrap();
            print!("{}", s);
        }
        println!()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
    ch: char,
}
impl Point {
    /// Is the other point next to this point in North, East, South or West direction
    fn is_neighbour_4(&self, other: &Point) -> bool {
        (self.y == other.y && self.x.abs_diff(other.x) == 1)
            || (self.x == other.x && self.y.abs_diff(other.y) == 1)
    }

    fn new_usize(x: usize, y: usize, ch: char) -> Self {
        Self::new(x as i64, y as i64, ch)
    }

    fn new(x: i64, y: i64, ch: char) -> Self {
        Self { x, y, ch }
    }

    /// Returns all up to 8 neighboring points. x and y can be negative for edge cases.
    /// ch is copied over
    fn create_neighbors_8(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for dy in -1..2 {
            for dx in -1..2 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                let point = Point::new(self.x + dx, self.y + dy, self.ch);
                points.push(point);
            }
        }
        points
    }

    /// Returns neighbors in North, East, South and West direction. x and y can be negative for edge cases.
    /// ch is copied over
    fn create_neighbors_4(&self) -> Vec<Point> {
        let mut points = vec![
            Point::new(self.x + 1, self.y, self.ch),
            Point::new(self.x - 1, self.y, self.ch),
            Point::new(self.x, self.y + 1, self.ch),
            Point::new(self.x, self.y - 1, self.ch),
        ];
        points
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Region {
    ch: char,
    points: Vec<Point>,
}

impl Region {
    fn can_merge(&self, other: &Region) -> bool {
        if self.ch == other.ch {
            return self.points.iter().any(|point| {
                other
                    .points
                    .iter()
                    .any(|other_point| point.is_neighbour_4(other_point))
            });
        }
        false
    }

    /// Merge all points from the other region into this region.
    /// Other region is dropped afterwards.
    fn merge(&mut self, mut other: Region) {
        self.points.append(&mut other.points);
    }

    fn new(ch: char, point: Point) -> Self {
        let points = vec![point];
        Self { ch, points }
    }

    fn calulate_border(&self) -> Vec<Point> {
        let mut border: Vec<Point> = Vec::new();
        self.points.iter().for_each(|point| {
            let mut neighbors = point.create_neighbors_4();
            for mut neighbor in neighbors {
                if !self.points.contains(&neighbor) {
                    neighbor.ch = '#';
                    // borders can be duplicated, because every edge has one right next to it, even if they are on the same point
                    border.push(neighbor);
                }
            }
        });
        border
    }

    fn calculate_perimeter_length(&self) {
        let border = self.calulate_border();
    }
}
