use crate::input;

#[test]
pub fn test_all() {
    run();
}

pub fn run() {
    let sample_input = "2333133121414131402";

    let input = input::load_file("2024", "09");

    let result = process_1(sample_input);
    assert_eq!(result, 1928);

    let result = process_1(&input);
    assert_eq!(result, 6461289671426);

    let result = process_2(sample_input);
    assert_eq!(result, 2858);

    let result = process_2(&input);
    assert_eq!(result, 6488291456470);
}

#[derive(Debug, Clone, PartialEq)]
struct File {
    id: u64,
}

fn process_1(input: &str) -> u64 {
    let mut blocks: Vec<Option<File>> = input
        .chars()
        .enumerate()
        .flat_map(|(index, ch)| {
            // 0 and every even index defines a block, every uneven index defines empty space.
            let is_block = index % 2 == 0;
            // The index of the block in the "compressed" array is the id.
            let id: Option<u64> = if is_block {
                Some(index as u64 / 2)
            } else {
                None
            };

            // The character digit of the block or the free space defines the size.
            let count: u32 = ch.to_digit(10).unwrap_or(0);
            let mut blocks: Vec<Option<File>> = Vec::new();
            for i in 0..count {
                blocks.push(id.map(|id| File { id }));
            }
            blocks
        })
        .collect();

    let mut left_index = 0;
    let mut right_index = blocks.len() - 1;
    while left_index < right_index {
        let left_block = &blocks[left_index];
        let right_block = &blocks[right_index];

        if left_block.is_some() {
            left_index += 1;
            continue;
        }
        if right_block.is_none() {
            right_index -= 1;
            continue;
        }
        blocks[left_index] = Some(File {
            id: right_block.as_ref().unwrap().id,
        });
        blocks[right_index] = None;
    }

    blocks
        .iter()
        .enumerate()
        .map(|(i, block_opt)| (i as u64) * block_opt.as_ref().map(|file| file.id).unwrap_or(0))
        .sum::<u64>()
}

#[derive(Debug, Clone, PartialEq)]
struct Block {
    start: u64,
    size: u64,
    id: u64,
    is_used: bool,
}

fn process_2(input: &str) -> u64 {
    let mut last_block_end = 0;

    let mut blocks: Vec<Block> = input
        .chars()
        .enumerate()
        .filter_map(|(index, ch)| {
            // 0 and every even index defines a block, every uneven index defines empty space.
            let is_used = index % 2 == 0;
            // The index of the block in the "compressed" array is the id.
            let id: u64 = if is_used { index as u64 / 2 } else { 0 };

            // The character digit of the block or the free space defines the size.
            let size: u64 = ch.to_digit(10).unwrap_or(0) as u64;
            if size == 0 {
                // Blocks without size are no blocks
                return None;
            }
            let start = last_block_end;
            last_block_end += size;
            Some(Block {
                start,
                size,
                id,
                is_used,
            })
        })
        .collect();

    let mut right_index = blocks.len() - 1;
    'a: while right_index > 0 {
        let mut right_block = blocks[right_index].clone();

        if right_block.is_used {
            let mut left_index = 0;
            // only move up to right index
            while left_index < right_index {
                let mut left_block = blocks[left_index].clone();

                if !left_block.is_used && left_block.size >= right_block.size {
                    blocks[right_index].is_used = false;
                    blocks[right_index].id = 0;

                    right_block.start = left_block.start;
                    left_block.size -= right_block.size;
                    left_block.start = right_block.start + right_block.size;

                    if left_block.size > 0 {
                        // still some free space left, keep original free space block
                        blocks.insert(left_index, right_block);
                        blocks[left_index + 1] = left_block;
                        // we insert a new block, so go one to the right again
                        right_index += 1;
                    } else {
                        // drop previous free space block
                        blocks[left_index] = right_block;
                    }
                    // some position has been found, continue with next block
                    continue 'a;
                }
                left_index += 1;
            }
        }
        right_index -= 1;
    }

    // print_blocks(&blocks);

    blocks
        .iter()
        .filter(|block| block.is_used)
        .map(|block| {
            // iterate over each "field" of the block
            (0..block.size)
                // multiply id by the total index of the field
                .map(|i| block.id * (block.start + i))
                // sum all up
                .sum::<u64>()
        })
        // sum all blocks as well
        .sum::<u64>()
}

fn print_blocks(blocks: &[Block]) {
    blocks.iter().for_each(|block| {
        (0..block.size).for_each(|i| {
            if block.is_used {
                print!("{:?}", block.id);
            } else {
                print!(".");
            }
        });
        print!("|");
    });

    println!();
}
