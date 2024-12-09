use std::env;
use std::fs;

#[derive(Copy, Clone, Debug, PartialEq)]
enum BlockType {
    Empty,
    File(u64),
}

#[derive(Debug)]
struct Block {
    file: BlockType,
    length: u64,
}

fn part1(input: &str) {
    let mut file = true;
    let mut blocks = Vec::new();
    let mut id = 0;
    for c in input.trim().chars() {
        let digit = c.to_digit(10).unwrap();
        blocks.push(Block {
            file: if file {
                BlockType::File(id)
            } else {
                BlockType::Empty
            },
            length: digit as u64,
        });
        if file {
            id += 1;
        }
        file = !file;
    }

    let mut front_pointer = 0;

    while front_pointer < blocks.len() {
        if blocks[front_pointer].file != BlockType::Empty {
            front_pointer += 1;
            continue;
        }
        let last_pointer = blocks.len() - 1;
        match blocks[last_pointer].file {
            BlockType::Empty => {
                blocks.pop();
            }
            BlockType::File(id) => {
                let empty_block_length = blocks[front_pointer].length;
                if blocks[last_pointer].length > empty_block_length {
                    blocks[front_pointer].file = BlockType::File(id);
                    blocks[last_pointer].length -= blocks[front_pointer].length;
                } else if blocks[last_pointer].length < empty_block_length {
                    let rem = empty_block_length - blocks[last_pointer].length;
                    blocks[front_pointer].length = rem;
                    blocks.insert(
                        front_pointer,
                        Block {
                            file: BlockType::File(id),
                            length: blocks[last_pointer].length,
                        },
                    );
                    blocks.pop();
                } else if blocks[last_pointer].length == empty_block_length {
                    blocks[front_pointer].file = BlockType::File(id);
                    blocks.pop();
                }
            }
        }
    }

    let mut ans = 0;
    let mut i = 0;
    for block in blocks {
        match block.file {
            BlockType::File(id) => {
                for _j in 0..block.length {
                    ans += i * id;
                    i += 1;
                }
            }
            BlockType::Empty => {
                break;
            }
        }
    }
    println!("ans {ans}");
}

fn move_block(id: u64, blocks: &mut Vec<Block>) {
    let mut ind = 0;
    for i in (0..blocks.len()).rev() {
        match blocks[i].file {
            BlockType::File(idx) => {
                if id == idx {
                    ind = i;
                    break;
                }
            }
            BlockType::Empty => {}
        }
    }
    let length = blocks[ind].length;
    for i in 0..ind {
        match blocks[i].file {
            BlockType::Empty => {
                if blocks[i].length >= length {
                    let rem = blocks[i].length - length;
                    blocks[i].length = rem;
                    blocks[ind].file = BlockType::Empty;
                    blocks.insert(
                        i,
                        Block {
                            file: BlockType::File(id),
                            length,
                        },
                    );
                    break;
                }
            }
            BlockType::File(_) => {}
        }
    }
}

fn part2(input: &str) {
    let mut file = true;
    let mut blocks = Vec::new();
    let mut id = 0;
    for c in input.trim().chars() {
        let digit = c.to_digit(10).unwrap();
        blocks.push(Block {
            file: if file {
                BlockType::File(id)
            } else {
                BlockType::Empty
            },
            length: digit as u64,
        });
        if file {
            id += 1;
        }
        file = !file;
    }
    for i in (0..id).rev() {
        move_block(i, &mut blocks);
    }

    let mut ans = 0;
    let mut i = 0;
    for block in blocks {
        match block.file {
            BlockType::File(id) => {
                for _j in 0..block.length {
                    ans += i * id;
                    i += 1;
                }
            }
            BlockType::Empty => {
                i += block.length;
            }
        }
    }
    println!("ans {ans}");
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    part1(&input);
    part2(&input);
}
