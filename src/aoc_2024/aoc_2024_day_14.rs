use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    let input = input::load_file("2024", "14");

    let result = process_1(sample_input, 11, 7);
    assert_eq!(result, 12);

    let result = process_1(&input, 101, 103);
    assert_eq!(result, 230435667);

    let result = process_2(&input, 101, 103);
    assert_eq!(result, 0);
}

fn process_1(input: &str, width: i64, height: i64) -> i64 {
    let mut robots = parse_robots(input);

    robots.par_iter_mut().for_each(|robot| {
        for i in 0..100 {
            robot.step(width, height);
        }
    });

    let (sum_1, sum_2, sum_3, sum_4) = calulate_quadrants(&robots, width, height);

    sum_1 * sum_2 * sum_3 * sum_4
}

fn process_2(input: &str, width: i64, height: i64) -> i64 {
    let mut robots = parse_robots(input);

    for i in 0..10000 {
        robots.iter_mut().for_each(|robot| {
            robot.step(width, height);
        });

        let (sum_1, sum_2, sum_3, sum_4) = calulate_quadrants(&robots, width, height);
        // tree should create some sort of cluster. Search manually. Luckily 1/2 turns out to be a good limit
        let len = robots.len() as i64 / 2;
        if sum_1 >= len || sum_2 >= len || sum_3 >= len || sum_4 >= len {
            println!("Robots: {}", i + 1);
            print_robots(&robots, width, height);
            println!("-------");
        }
    }
    0
}

fn calulate_quadrants(robots: &[Robot], width: i64, height: i64) -> (i64, i64, i64, i64) {
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let mut sum_3 = 0;
    let mut sum_4 = 0;

    robots.iter().for_each(|robot| {
        let mid_x = width / 2;
        let mid_y = height / 2;
        if robot.x < mid_x && robot.y < mid_y {
            sum_1 += 1;
        }
        if robot.x > mid_x && robot.y < mid_y {
            sum_2 += 1;
        }
        if robot.x < mid_x && robot.y > mid_y {
            sum_3 += 1;
        }
        if robot.x > mid_x && robot.y > mid_y {
            sum_4 += 1;
        }
    });
    (sum_1, sum_2, sum_3, sum_4)
}

fn print_robots(robots: &[Robot], width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            let opt = robots.iter().find(|robot| robot.x == x && robot.y == y);
            match opt {
                Some(_) => print!("X"),

                None => print!(" "),
            }
        }
        println!();
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let mut robots: Vec<Robot> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mid = line.find(" v=").unwrap();
            let ps: Vec<i64> = line[2..mid]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let vs: Vec<i64> = line[mid + 3..]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            Robot {
                x: ps[0],
                y: ps[1],
                vx: vs[0],
                vy: vs[1],
            }
        })
        .collect();
    robots
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}
impl Robot {
    fn step(&mut self, width: i64, height: i64) {
        self.x = modulo(self.x + self.vx, width);
        self.y = modulo(self.y + self.vy, height);
    }
}

fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}
