use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;
use regex::Regex;

const DAY: &str = "10"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"; // TODO: Add the test input

struct Machine {
    target_mask: usize,
    buttons_masks: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>
}

fn parse_line(line: String) -> Machine {
    let re_target = Regex::new(r"\[([.#]+)\]").unwrap();
    let target_captures = re_target.captures(&line).unwrap();
    let target_inside = &target_captures[1];
    let mut target_mask = 0usize;
    for (i, ch) in target_inside.chars().enumerate() {
        if ch == '#' {
            target_mask |= 1 << i;
        }
    }

    let re_button = Regex::new(r"\(([\d,]+)\)").unwrap();
    let mut buttons_masks = Vec::new();
    let mut buttons = Vec::new();

    for caps in re_button.captures_iter(&line) {
        let inside = &caps[1];
        let indices: Vec<usize> = inside
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let mut mask = 0usize;
        for idx in &indices {
            mask |= 1 << idx;
        }

        buttons_masks.push(mask);
        buttons.push(indices);
    }

    let re_joltages = Regex::new(r"\{([\d,]+)\}").unwrap();
    let joltages_captures = re_joltages.captures(&line).unwrap();
    let joltages_inside = &joltages_captures[1];
    let joltages: Vec<usize> = joltages_inside
        .split(',')
        .map(|part| part.parse::<usize>().unwrap())
        .collect();

    Machine { target_mask, buttons_masks, buttons, joltages }
}

fn get_steps_1(machine: Machine) -> usize {
    let mut fewest_steps = usize::MAX;
    for buttons_masks_mask in 0..(1 << machine.buttons_masks.len()) {
        let mut final_mask = 0usize;
        for (button_index, &buttons_mask) in machine.buttons_masks.iter().enumerate() {
            if buttons_masks_mask & (1 << button_index) != 0 {
                final_mask = final_mask ^ buttons_mask
            }
        }
        if final_mask == machine.target_mask {
            let steps = format!("{buttons_masks_mask:b}").chars().filter(|&ch| ch == '1').count();
            if steps < fewest_steps {
                fewest_steps = steps;
            }
        }
    }
    fewest_steps
}

fn get_steps_2(machine: Machine) -> usize {
    #[derive(Clone, PartialEq, Eq, Hash)]
    struct State {
        joltages: Vec<usize>,
        steps: usize
    }

    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<Vec<usize>> = HashSet::new();

    let n = machine.joltages.len();
    let mut initial_state: State = State {
        joltages: Vec::new(),
        steps: 0usize
    };
    for _ in 0..n {
        initial_state.joltages.push(0)
    };
    queue.push_back(initial_state.clone());
    visited.insert(initial_state.joltages);

    let mut max_steps = 0;

    while queue.len() > 0 {
        let state = queue.pop_front().unwrap();
        for button in &machine.buttons {
            let mut new_state: State = State {
                joltages: state.joltages.clone(),
                steps: state.steps + 1
            };
            if new_state.steps > max_steps {
                max_steps = new_state.steps;
                println!("max_steps: {}, visited: {}", max_steps, visited.len());
            }
            let mut ok = true;
            for &pos in button {
                new_state.joltages[pos] = new_state.joltages[pos] + 1;
                if new_state.joltages[pos] > machine.joltages[pos] {
                    ok = false;
                    break
                }
            }
            if new_state.joltages == machine.joltages {
                println!("Found solution with steps: {}, visited: {}", new_state.steps, visited.len());
                return new_state.steps
            }
            if ok && !visited.contains(&new_state.joltages) {
                queue.push_back(new_state.clone());
                visited.insert(new_state.joltages);
            }
        }
    }

    panic!("Target joltage unreachable")
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    // println!("=== Part 1 ===");
    //
    // fn part1<R: BufRead>(reader: R) -> Result<usize> {
    //     // TODO: Solve Part 1 of the puzzle
    //     Ok(reader.lines().flatten().map(parse_line).map(get_steps_1).sum())
    // }
    //
    // // TODO: Set the expected answer for the test input
    // assert_eq!(2+3+2, part1(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file)?);
    // println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 2 of the puzzle
        Ok(reader.lines().flatten().map(parse_line).map(get_steps_2).sum())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(10+12+11, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
