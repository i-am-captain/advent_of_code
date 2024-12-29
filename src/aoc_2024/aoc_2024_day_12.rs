use std::collections::HashMap;

use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    let input = input::load_file("2024", "12");

    let result = process_1(sample_input);
    assert_eq!(result, 1930);

    let result = process_1(&input);
    assert_eq!(result, 0);
    return;
    let result = process_2(sample_input);
    assert_eq!(result, 0);

    let result = process_2(&input);
    assert_eq!(result, 0);
}

fn process_1(input: &str) -> i64 {
    // Recursively searching for neighbors (DFS) would probably be faster and simpler, but i want to try something different here. Declare each element as region and then merge regions of same kind.

    let mut map: HashMap<char, Vec<Region>> = HashMap::new();

    // Parse all elements into separate regions.
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                let region = Region::new(ch, Point::new(x, y, ch));
                map.entry(ch).or_default().push(region);
            });
        });

    map.iter_mut().for_each(|(key, mut regions)| {
        // repeat merge as long as there is something to merge. If latest iteration performed no merge, skip.
        let mut has_merged = true;
        while has_merged {
            has_merged = false;
            // iterate from last element, to remove like from a stack. That should be faster due to not moving elements to other indices after remove.
            let mut i = regions.len();
            while i > 0 {
                // subtract first, to be able to stop at 0. Start at len() instead of len() -1
                i -= 1;

                // need to remove the region, to make borrow checker happy. Maybe not the best solution.
                let mut region = regions.remove(i);
                let mut j = regions.len();
                while j > 0 {
                    // same as for i
                    j -= 1;
                    let mut other_region = regions.remove(j);
                    if region.can_merge(&other_region) {
                        // merge, do not add back to stack
                        region.merge(other_region);
                        // since one more element was removed, move i one more to the left.
                        i = i.saturating_sub(1);
                        // something was merged, go for another loop around.
                        has_merged = true;
                    } else {
                        // cannot merge, add back to stack
                        regions.push(other_region);
                    }
                }
                // add the previously removed region back. It might have multiple regions merged.
                regions.push(region);
            }
        }
    });

    // println!("{:?}", map);
    // Regions are created, now try calculating perimeter

    map.iter().for_each(|(key, regions)| {});

    0
}

fn process_2(input: &str) -> i64 {
    0
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
    ch: char,
}
impl Point {
    /// Is the other point next to this point in North, East, South or Westt direction
    fn is_neighbour_4(&self, other: &Point) -> bool {
        (self.y == other.y && self.x.abs_diff(other.x) == 1)
            || (self.x == other.x && self.y.abs_diff(other.y) == 1)
    }

    fn new(x: usize, y: usize, ch: char) -> Self {
        Self { x, y, ch }
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
}
