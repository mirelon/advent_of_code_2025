use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"; // TODO: Add the test input

#[derive(Debug)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }

        // union by size
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }

        self.parent[b] = a;
        self.size[a] += self.size[b];
        true
    }
}

type JunctionBox = (usize, usize, usize);

fn dist(a: JunctionBox, b: JunctionBox) -> f64 {
    ((a.0 as f64 - b.0 as f64).powi(2)
        + (a.1 as f64 - b.1 as f64).powi(2)
        + (a.2 as f64 - b.2 as f64).powi(2))
    .sqrt()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, connections: usize) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();
        let mut junction_boxes = Vec::new();

        for line in lines {
            let parts: Vec<_> = line.split(',').collect();
            if parts.len() != 3 {
                panic!("Invalid line: {}", line);
            }
            junction_boxes.push((
                parts[0].parse::<usize>()?,
                parts[1].parse::<usize>()?,
                parts[2].parse::<usize>()?,
            ));
        }

        let n = junction_boxes.len();
        let mut dsu = DSU::new(n);

        let mut distances = Vec::new();
        for i in 0..n - 1 {
            for j in i + 1..n {
                distances.push((dist(junction_boxes[i], junction_boxes[j]), i, j));
            }
        }

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for k in 0..connections {
            let (_, i, j) = distances[k];
            dsu.union(i, j);
        }

        let mut sizes = Vec::new();
        for i in 0..n {
            if dsu.find(i) == i {
                sizes.push(dsu.size[i]);
            }
        }

        sizes.sort_unstable_by(|a, b| b.cmp(a));

        Ok(sizes.iter().take(3).product())
    }

    // Wrong solution where indexes messed up:
    // fn part1<R: BufRead>(reader: R) -> Result<usize> {
    //     // TODO: Solve Part 1 of the puzzle
    //     let lines: Vec<String> = reader.lines().flatten().collect();
    //     let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    //
    //     for line in lines {
    //         let parts: Vec<&str> = line.split(',').collect();
    //         if parts.len() != 3 {
    //             panic!("Invalid line: {}", line);
    //         }
    //
    //         let x = parts[0].parse::<usize>()?;
    //         let y = parts[1].parse::<usize>()?;
    //         let z = parts[2].parse::<usize>()?;
    //
    //         junction_boxes.push((x, y, z));
    //     }
    //
    //     let mut circuits: Vec<HashSet<usize>> =
    //         (0..junction_boxes.len()).map(|i| HashSet::from([i])).collect();
    //
    //     let mut distances: Vec<(f64, usize, usize)> = Vec::new();
    //     for i in 0..junction_boxes.len()-1 {
    //         for j in i+1..junction_boxes.len() {
    //             distances.push((dist(junction_boxes[i], junction_boxes[j]), i, j))
    //         }
    //     }
    //
    //     distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    //
    //     let mut count = 0;
    //
    //     for (distance, i, j) in distances.into_iter() {
    //         // find the set index that contains i
    //         let ci = circuits.iter()
    //             .position(|c| c.contains(&i))
    //             .unwrap();
    //
    //         // find the set index that contains j
    //         let cj = circuits.iter()
    //             .position(|c| c.contains(&j))
    //             .unwrap();
    //
    //         if ci != cj {
    //             // Merge cj into ci
    //             let (left, right) = if ci < cj { (ci, cj) } else { (cj, ci) };
    //
    //             let other = circuits.remove(right);        // take the second set out
    //             circuits[left].extend(other);              // merge it into the first
    //         }
    //
    //         count += 1;
    //         if count == 10 {
    //             break
    //         }
    //     }
    //
    //     let mut sizes: Vec<usize> =
    //         circuits.iter().map(|c| c.len()).collect();
    //
    //     sizes.sort_unstable_by(|a, b| b.cmp(a)); // descending
    //
    //     println!("{:?}", circuits);
    //     println!("{:?}", sizes);
    //
    //     let result: usize = sizes.iter().take(3).product();
    //
    //     Ok(result)
    // }

    // TODO: Set the expected answer for the test input
    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();
        let mut junction_boxes = Vec::new();

        for line in lines {
            let parts: Vec<_> = line.split(',').collect();
            if parts.len() != 3 {
                panic!("Invalid line: {}", line);
            }
            junction_boxes.push((
                parts[0].parse::<usize>()?,
                parts[1].parse::<usize>()?,
                parts[2].parse::<usize>()?,
            ));
        }

        let n = junction_boxes.len();
        let mut dsu = DSU::new(n);

        let mut distances = Vec::new();
        for i in 0..n - 1 {
            for j in i + 1..n {
                distances.push((dist(junction_boxes[i], junction_boxes[j]), i, j));
            }
        }

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut components = junction_boxes.len();

        for k in 0..distances.len() {
            let (_, i, j) = distances[k];
            let is_new_connection = dsu.union(i, j);
            if is_new_connection {
                components -= 1
            }
            if components == 1 {
                return Ok(junction_boxes[i].0 * junction_boxes[j].0);
                break;
            }
        }
        println!("Components were not 1 {components}");
        panic!();
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
