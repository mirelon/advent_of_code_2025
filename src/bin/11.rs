use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"; // TODO: Add the test input

const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"; // TODO: Add the test input

fn parse_line(line: String) -> (String, Vec<String>) {
    let mut parts = line.splitn(2, ": ");

    let node = parts.next().unwrap().to_string();
    let right = parts.next().unwrap_or("").trim();

    let children = right
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    (node, children)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let graph: HashMap<String, Vec<String>> = reader
            .lines()
            .flatten()
            .map(|line| parse_line(line))
            .collect();

        fn dfs(
            graph: &HashMap<String, Vec<String>>,
            current: &str,
            visited: &mut HashSet<String>,
        ) -> usize {
            if visited.contains(current) {
                println!("Cycle detected at {}", current);
                panic!("Cycle");
            }

            visited.insert(current.to_string());

            let downstream = graph.get(current).unwrap();
            let mut count = 0;

            for next in downstream {
                if next == "out" {
                    count += 1;
                    continue;
                }
                count += dfs(graph, next, visited);
            }

            visited.remove(current); // allow other branches to reuse nodes
            count
        }

        let mut visited = HashSet::new();
        Ok(dfs(&graph, "you", &mut visited))
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(5, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let graph: HashMap<String, Vec<String>> = reader
            .lines()
            .flatten()
            .map(|line| parse_line(line))
            .collect();

        fn dfs(
            graph: &HashMap<String, Vec<String>>,
            current: &str,
            visited: &mut HashSet<String>,
        ) -> usize {
            if visited.contains(current) {
                println!("Cycle detected at {}", current);
                panic!("Cycle");
            }

            if visited.len() < 20 {
                println!("Entering {current} visited {:?}", visited)
            }

            visited.insert(current.to_string());

            let downstream = graph
                .get(current)
                .unwrap_or_else(|| panic!("Node {current} is not defined"));
            let mut count = 0;

            for next in downstream {
                if next == "out" {
                    if visited.contains("dac") && visited.contains("fft") {
                        count += 1;
                    };
                    continue;
                }
                count += dfs(graph, next, visited);
            }

            visited.remove(current); // allow other branches to reuse nodes
            if visited.len() < 20 {
                println!("Finished visiting {:?}, found {count}", visited)
            }
            count
        }

        let mut visited = HashSet::new();
        Ok(dfs(&graph, "svr", &mut visited))
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);

    println!();
    println!("Now the real part");
    println!();

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
