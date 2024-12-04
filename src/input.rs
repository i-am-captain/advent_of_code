use std::fs;

/// Load input from private repo cloned next to this repo.
/// Format: ../aoc_input/2024/day_01.txt
pub fn load_file(year: &str, day: &str) -> String {
    let mut relative_path = "../aoc_input/".to_owned();
    relative_path = relative_path + year + "/day_" + day + ".txt";

    fs::read_to_string(relative_path).expect("Cannot read file.")
}
