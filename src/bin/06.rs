use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let lines: Vec<String> = reader.lines().flatten().collect();

        let method_line = lines.last().unwrap();
        let methods: Vec<char> = method_line
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect();

        let mut problems: Vec<Vec<i64>> = vec![vec![]; methods.len()];

        for line in &lines[..lines.len() - 1] {
            for (j, item) in line.split_whitespace().enumerate() {
                problems[j].push(item.parse::<i64>()?);
            }
        }

        // reduce column-wise
        let mut sum = 0;
        for j in 0..methods.len() {
            let method = methods[j];
            let val = problems[j]
                .iter()
                .copied()
                .reduce(|a, b| match method {
                    '+' => a + b,
                    '*' => a * b,
                    _ => unreachable!(),
                })
                .unwrap();
            sum += val;
        }

        Ok(sum as usize)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 2 of the puzzle
        let lines: Vec<String> = reader.lines().flatten().collect();
        let last_line = lines.last().unwrap();
        let mut methods: Vec<char> = Vec::new();
        let mut columns: Vec<usize> = Vec::new();

        // Iterate each character
        for (i, ch) in last_line.chars().enumerate() {
            if ch == '+' || ch == '*' {
                methods.push(ch);
                columns.push(i);
                println!("method={} column={}", ch, i);
            }
        }

        let mut max_len = 0;
        for line in &lines {
            if line.len() > max_len {
                max_len = line.len()
            }
        }

        let mut sum = 0;
        for problem_index in 0..methods.len() {
            let from_column = columns[problem_index];
            let method = methods[problem_index];
            let to_column = if problem_index == methods.len() - 1 {
                max_len - 1
            } else {
                columns[problem_index + 1] - 2
            };
            if from_column > to_column {
                panic!(
                    "problem_index = {}, from_column = {}, to_column = {}",
                    problem_index, from_column, to_column
                )
            };
            let mut numbers: Vec<usize> = Vec::new();
            for column in from_column..=to_column {
                let mut number = 0;
                let mut number_started = false;
                for line in lines[0..lines.len() - 1].iter() {
                    let digit = line.chars().nth(column).unwrap_or(' ');
                    if digit == ' ' {
                        if number_started {
                            break;
                        } else {
                            continue;
                        }
                    }
                    number = number * 10 + digit.to_string().parse::<usize>().unwrap();
                }
                numbers.push(number);
            }
            let val = numbers
                .iter()
                .copied()
                .reduce(|a, b| match method {
                    '+' => a + b,
                    '*' => a * b,
                    _ => unreachable!(),
                })
                .unwrap();
            println!("val = {val}");
            sum += val;
        }

        println!("Methods: {:?}", methods);
        println!("Columns: {:?}", columns);

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
