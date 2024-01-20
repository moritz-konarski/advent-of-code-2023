use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PART1_FILE: &str = "part1.txt";
const PART2_FILE: &str = "part2.txt";
const WORKING: char = '.';
const BROKEN: char = '#';
const UNKNOWN: char = '?';

fn main() {
    let usage = "Incorrect arguements!\nUsage: day-12 p<n>";
    if let Some(part) = env::args().nth(1) {
        match part.as_str() {
            "p1" => {
                println!("Reading `{PART1_FILE}`");
                println!("Sum is {}", part1(PART1_FILE));
            }
            "p2" => {
                println!("Reading `{PART2_FILE}`");
                println!("Sum is {}", part2(PART2_FILE));
            }
            _ => eprintln!("{usage}"),
        }
    } else {
        eprintln!("{usage}");
    }
}

fn process_line(springs: Vec<&str>, nums: Vec<usize>) -> usize {
    let mut section_permutations: Vec<usize> = vec![];
    let mut spring_index = 0;
    let mut nums_index = 0;

    while let Some(spring) = springs.get(spring_index) {
        let unknown_count = spring.chars().filter(|c| *c == UNKNOWN).count();
        let broken_count = spring.chars().filter(|c| *c == BROKEN).count();

        if broken_count == nums[nums_index] {
            println!(
                "complete broken fit {spring:?} with {:?} at {:?}",
                nums[nums_index], nums_index
            );
            section_permutations.push(1);
        } else if unknown_count == nums[nums_index] {
            println!(
                "complete unknown fit {spring:?} with {:?} at {:?}",
                nums[nums_index], nums_index
            );
            section_permutations.push(1);
        } else if unknown_count == nums[nums_index] + 1 {
            println!(
                "+1 fit {spring:?} with {:?} at {:?}",
                nums[nums_index], nums_index
            );
            section_permutations.push(nums[nums_index] + 1);
        } else if unknown_count + broken_count == nums[nums_index] {
            println!(
                "broken + unknown fit {spring:?} with {:?} at {:?}",
                nums[nums_index], nums_index
            );
            section_permutations.push(1);
        }

        spring_index += 1;
        nums_index += 1;
        continue;

        let new_limit = nums_index + 2;
        if let Some(num_slice) = nums.get(nums_index..new_limit) {
            if unknown_count == num_slice.iter().sum::<usize>() + num_slice.len() - 1 {
                println!(
                    "multi-fit {spring:?} with {:?} at {:?}",
                    num_slice,
                    nums_index..new_limit
                );
                section_permutations.push(nums[nums_index]);

                spring_index += 1;
                nums_index += 2;
                continue;
            }
        }

        spring_index += 1;
        nums_index += 1;

        if nums_index >= nums.len() {
            break;
        }
    }

    println!("Done at s:{:?}, n:{:?} left", spring_index, nums_index);

    // let broken_counts: Vec<usize> = springs
    //     .iter()
    //     .map(|s| s.chars().filter(|c| *c == BROKEN).count())
    //     .collect();

    // if broken_counts.len() != springs.len() {
    //     return (springs.to_vec(), nums.to_vec());
    // }

    // let mut n_offset = 0;
    // let mut remove_indices = vec![];
    // for count in broken_counts {
    //     if let Some(index) = nums.iter().skip(n_offset).position(|n| *n == count) {
    //         n_offset = index + n_offset;
    //         remove_indices.push(n_offset);
    //         n_offset += 1;
    //     }
    // }

    // println!("{remove_indices:?}");

    // for (i, index) in remove_indices.iter().enumerate() {
    //     springs.remove(*index - i);
    //     nums.remove(*index - i);
    // }

    // let (bc_offset, n_offset) = broken_counts
    //     .iter()
    //     .enumerate()
    //     .find_map(|(i, bc)| {
    //         if let Some(pos) = nums.iter().position(|n| n == bc) {
    //             Some((i, pos))
    //         } else {
    //             None
    //         }
    //     })
    //     .unwrap();

    // println!("{bc_offset:?} {n_offset:?}");
    // for (index, count) in broken_counts[bc_offset..].iter().enumerate().rev() {
    //     if *count == 0 {
    //         continue;
    //     }

    //     if index >= nums.len() {
    //         break;
    //     }

    //     println!("{index:?} {:?}", broken_counts[index]);

    //     if broken_counts[index] == nums[n_offset..][index] {
    //         springs.remove(index + n_offset);
    //         nums.remove(index + n_offset);
    //     }
    // }

    // (springs, nums)
    let p = section_permutations.iter().product();
    println!("result: {p:?}");
    p
}

fn part1(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    file.lines().fold(0, |sum, line| {
        let line = line.unwrap();
        println!("\n{line}");
        let (springs, nums) = line.split_once(' ').unwrap();
        let springs: Vec<&str> = springs.split(WORKING).filter(|s| !s.is_empty()).collect();
        println!("{springs:?}");
        let nums: Vec<usize> = nums.split(',').map(|n| n.parse().unwrap()).collect();
        println!("{nums:?}");

        sum + process_line(springs, nums)
    })
}

fn part2(filename: &str) -> usize {
    let file = File::open(filename).expect("Should be able to read the file");
    let file = BufReader::new(file);

    0
}

#[test]
fn part1_example() {
    assert_eq!(21, part1("test1.txt"));
}

// #[test]
// fn part1_puzzle() {
//     assert_eq!(250058342, part1(PART1_FILE));
// }

// #[test]
// fn part2_example() {
//     assert_eq!(5905, part2("test2.txt"));
// }

// #[test]
// fn part2_puzzle() {
//     assert_eq!(250506580, part2(PART2_FILE));
// }
