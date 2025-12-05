use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io;

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    // println!("=== Part 1 ===");
    //
    // fn part1<R: io::BufRead>(reader: R) -> Result<usize> {
    //     // TODO: Solve Part 1 of the puzzle
    //     let mut grid = Vec::<Vec<char>>::new();
    //
    //     for line in reader.lines() {
    //         let line = line?; // io::Result<String> → String
    //         let trimmed = line.trim();
    //
    //         if trimmed.is_empty() {
    //             continue; // ignore blank lines
    //         }
    //
    //         grid.push(trimmed.chars().collect());
    //     }
    //
    //     let mut grid2 = grid.clone();
    //
    //     let mut count = 0;
    //     for i in 0..grid.len() {
    //         for j in 0..grid[i].len() {
    //             let mut neighbors = 0;
    //             if grid[i][j] != '@' {
    //                 continue
    //             }
    //             if i > 0 && grid[i - 1][j] == '@' {
    //                 neighbors += 1
    //             }
    //             if i > 0 && j > 0 && grid[i - 1][j - 1] == '@' {
    //                 neighbors += 1
    //             }
    //             if i > 0 && j < grid[i].len() - 1 && grid[i - 1][j + 1] == '@' {
    //                 neighbors += 1
    //             }
    //             if i < grid.len() - 1 && grid[i + 1][j] == '@' {
    //                 neighbors += 1
    //             }
    //             if i < grid.len() - 1 && j > 0 && grid[i + 1][j - 1] == '@' {
    //                 neighbors += 1
    //             }
    //             if i < grid.len() - 1 && j < grid[i].len() - 1 && grid[i + 1][j + 1] == '@' {
    //                 neighbors += 1
    //             }
    //             if j < grid[i].len() - 1 && grid[i][j + 1] == '@' {
    //                 neighbors += 1
    //             }
    //             if j > 0 && grid[i][j - 1] == '@' {
    //                 neighbors += 1
    //             }
    //             println!("{}", neighbors);
    //             if neighbors < 4 {
    //                 count += 1;
    //                 grid2[i][j] = 'X';
    //             }
    //         }
    //     }
    //     for line in grid2 {
    //         for char in line {
    //             print!("{}", char)
    //         }
    //         println!()
    //     }
    //     Ok(count)
    // }
    //
    // // TODO: Set the expected answer for the test input
    // assert_eq!(13, part1(io::BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = io::BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: io::BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 2 of the puzzle
        let mut grid = Vec::<Vec<char>>::new();

        for line in reader.lines() {
            let line = line?; // io::Result<String> → String
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue; // ignore blank lines
            }

            grid.push(trimmed.chars().collect());
        }

        let mut grid2 = grid.clone();

        let mut removed = 1; // Will be reset at the loop start
        let mut removed_total = 0;

        while removed > 0 {
            removed = 0;
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    let mut neighbors = 0;
                    if grid[i][j] != '@' {
                        continue
                    }
                    if i > 0 && grid[i - 1][j] == '@' {
                        neighbors += 1
                    }
                    if i > 0 && j > 0 && grid[i - 1][j - 1] == '@' {
                        neighbors += 1
                    }
                    if i > 0 && j < grid[i].len() - 1 && grid[i - 1][j + 1] == '@' {
                        neighbors += 1
                    }
                    if i < grid.len() - 1 && grid[i + 1][j] == '@' {
                        neighbors += 1
                    }
                    if i < grid.len() - 1 && j > 0 && grid[i + 1][j - 1] == '@' {
                        neighbors += 1
                    }
                    if i < grid.len() - 1 && j < grid[i].len() - 1 && grid[i + 1][j + 1] == '@' {
                        neighbors += 1
                    }
                    if j < grid[i].len() - 1 && grid[i][j + 1] == '@' {
                        neighbors += 1
                    }
                    if j > 0 && grid[i][j - 1] == '@' {
                        neighbors += 1
                    }
                    // println!("{}", neighbors);
                    if neighbors < 4 {
                        removed += 1;
                        removed_total += 1;
                        grid2[i][j] = 'X';
                    }
                }
            }


            // Print grid2
            for line in &grid2 {
                for &char in line {
                    print!("{}", char)
                }
                println!()
            }
            println!();
            println!();
            println!();


            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid2[i][j] == 'X' {
                        grid[i][j] = '.';
                        grid2[i][j] = '.';
                    }
                }
            }

        }

        Ok(removed_total)
    }

    // TODO: Set the expected answer for the 2nd part test input
    assert_eq!(43, part2(io::BufReader::new(TEST.as_bytes()))?);

    let input_file = io::BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
