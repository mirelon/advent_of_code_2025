use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
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

    // SLOW SOLUTION
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     let graph: HashMap<String, Vec<String>> = reader
    //         .lines()
    //         .flatten()
    //         .map(|line| parse_line(line))
    //         .collect();
    //
    //     fn dfs(
    //         graph: &HashMap<String, Vec<String>>,
    //         current: &str,
    //         visited: &mut HashSet<String>,
    //     ) -> usize {
    //         if visited.contains(current) {
    //             println!("Cycle detected at {}", current);
    //             panic!("Cycle");
    //         }
    //
    //         if visited.len() < 20 {
    //             println!("Entering {current} visited {:?}", visited)
    //         }
    //
    //         visited.insert(current.to_string());
    //
    //         let downstream = graph
    //             .get(current)
    //             .unwrap_or_else(|| panic!("Node {current} is not defined"));
    //         let mut count = 0;
    //
    //         for next in downstream {
    //             if next == "out" {
    //                 if visited.contains("dac") && visited.contains("fft") {
    //                     count += 1;
    //                 };
    //                 continue;
    //             }
    //             count += dfs(graph, next, visited);
    //         }
    //
    //         visited.remove(current); // allow other branches to reuse nodes
    //         if visited.len() < 20 {
    //             println!("Finished visiting {:?}, found {count}", visited)
    //         }
    //         count
    //     }
    //
    //     let mut visited = HashSet::new();
    //     Ok(dfs(&graph, "svr", &mut visited))
    // }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // Parse graph
        let graph: HashMap<String, Vec<String>> =
            reader.lines().flatten().map(parse_line).collect();

        // 1. Compute in-degrees
        let mut indegree: HashMap<String, usize> = HashMap::new();
        for (node, outs) in &graph {
            indegree.entry(node.clone()).or_insert(0);
            for next in outs {
                *indegree.entry(next.clone()).or_insert(0) += 1;
            }
        }

        // 2. Kahn’s algorithm: build topological order
        let mut topo: Vec<String> = Vec::new();
        let mut queue: VecDeque<String> = indegree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(k, _)| k.clone())
            .collect();

        while let Some(n) = queue.pop_front() {
            topo.push(n.clone());
            if let Some(outs) = graph.get(&n) {
                for out in outs {
                    let deg = indegree.get_mut(out).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(out.clone());
                    }
                }
            }
        }

        println!("Topo: {:?}", topo);

        // 3. DP over nodes in topological order
        // path counts: (both, dac, fft, none)
        let mut paths: HashMap<String, (usize, usize, usize, usize)> = HashMap::new();
        paths.insert("svr".to_string(), (0, 0, 0, 1));

        for node in topo {
            let (both, dac, fft, none) = *paths.get(&node).unwrap_or(&(0, 0, 0, 0));

            println!("Visiting {node}: ({both}, {dac}, {fft}, {none})");

            let nexts = match graph.get(&node) {
                Some(v) => v,
                None => continue,
            };

            for next in nexts {
                let (nb, nd, nf, nn) = paths.get(next).copied().unwrap_or((0, 0, 0, 0));

                let updated = match next.as_str() {
                    "dac" => (nb + both + fft, nd + none, 0, 0),
                    "fft" => (nb + both + dac, 0, nf + none, 0),
                    _ => (nb + both, nd + dac, nf + fft, nn + none),
                };

                paths.insert(next.clone(), updated);
            }
        }

        // 4. Output
        Ok(paths.get("out").unwrap().0)
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
