use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let lines: Vec<String> = reader.lines().flatten().collect();

        let start_line = lines.first().unwrap();
        let start_pos: usize = start_line
            .chars()
            .position(|c| c == 'S')
            .unwrap() as usize;
        let mut beams: HashSet<usize> = HashSet::new();
        beams.insert(start_pos);
        let mut count = 0;
        for line in &lines[1..] {
            let mut new_beams: HashSet<usize> = HashSet::new();
            for (i, ch) in line.chars().enumerate() {
                if beams.contains(&i) {
                    if ch == '^' {
                        count += 1;
                        new_beams.insert(i - 1);
                        new_beams.insert(i + 1);
                    } else {
                        new_beams.insert(i);
                    }
                }
            }
            beams = new_beams;
        }

        Ok(count)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 2 of the puzzle
        let lines: Vec<String> = reader.lines().flatten().collect();

        let start_line = lines.first().unwrap();
        let start_pos: usize = start_line.chars().position(|c| c == 'S').unwrap() as usize;
        let mut beams: HashMap<usize, usize> = HashMap::new();
        beams.insert(start_pos as usize, 1);
        for line in &lines[1..] {
            let mut new_beams: HashMap<usize, usize> = HashMap::new();
            for (i, ch) in line.chars().enumerate() {
                if let Some(&beam_count) = beams.get(&i) {
                    if ch == '^' {
                        *new_beams.entry(i - 1).or_insert(0) += beam_count;
                        *new_beams.entry(i + 1).or_insert(0) += beam_count;
                    } else {
                        *new_beams.entry(i).or_insert(0) += beam_count;
                    }
                }
            }
            beams = new_beams;
        }

        Ok(beams.values().sum())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
