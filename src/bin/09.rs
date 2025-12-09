use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().flatten().collect();

        let tiles: Vec<(usize, usize)> = lines
            .iter()
            .map(|line| {
                let mut nums = line.split(',').map(|p| p.parse::<usize>().unwrap());
                (nums.next().unwrap(), nums.next().unwrap())
            })
            .collect();

        // First pass: compute leftmost y for each x
        let mut leftmost_tiles: HashMap<usize, usize> = HashMap::new();
        for &(x, y) in &tiles {
            leftmost_tiles
                .entry(x)
                .and_modify(|yy| {
                    if y < *yy {
                        *yy = y
                    }
                })
                .or_insert(y);
        }

        // Second pass: compute largest area
        let mut largest_area = 0;
        for &(x, y) in &tiles {
            for (&xx, &yy) in &leftmost_tiles {
                if xx <= x && yy <= y {
                    let area = (x - xx + 1) * (y - yy + 1);
                    println!("x {x}, y {y}, xx {xx}, yy {yy}, area {area}");
                    if area > largest_area {
                        largest_area = area;
                    }
                }
            }
        }

        Ok(largest_area)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 2 of the puzzle
        let lines: Vec<String> = reader.lines().flatten().collect();

        let mut red_tiles: Vec<(usize, usize)> = lines
            .iter()
            .map(|line| {
                let mut nums = line.split(',').map(|p| p.parse::<usize>().unwrap());
                (nums.next().unwrap(), nums.next().unwrap())
            })
            .collect();

        // WRONG SOLUTION:

        // vertical = same x, values = (x, (lower y, higher y)), segment = 2 consecutive tiles
        // let mut vertical_segments: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        //
        // let mut last_red_tile = *red_tiles.last().unwrap(); // copy the last tile
        // println!("last_red_tile {:?}", last_red_tile);
        //
        // for &red_tile in &red_tiles {
        //     if red_tile.0 == last_red_tile.0 {
        //         let y_low = red_tile.1.min(last_red_tile.1);
        //         let y_high = red_tile.1.max(last_red_tile.1);
        //         vertical_segments.entry(red_tile.0).or_default().push((y_low, y_high));
        //     }
        //
        //     last_red_tile = red_tile; // copy, no borrowing
        // }
        //
        // println!("vertical_segments {:?}", vertical_segments);
        //
        // let (min_x, max_x, min_y, max_y) = red_tiles
        //     .iter()
        //     .fold(
        //         (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
        //         |(min_x, max_x, min_y, max_y), &(x, y)| {
        //             (
        //                 min_x.min(x),
        //                 max_x.max(x),
        //                 min_y.min(y),
        //                 max_y.max(y),
        //             )
        //         },
        //     );
        //
        // let mut color_tiles: HashSet<(usize, usize)> = HashSet::new();
        //
        // // Iterate xs and remember which ys are in
        // let mut sweep_in_ys: HashSet<usize> = HashSet::new();
        //
        // println!("sweeping from {min_x} to {max_x}");
        //
        // for x in min_x..=max_x {
        //     if (x % 100 == 0) {
        //         println!("sweep {x}");
        //     }
        //     // println!("sweep_in_ys {:?}", sweep_in_ys);
        //     sweep_in_ys.iter().for_each(|&y| { color_tiles.insert((x, y)); });
        //     for &(y0, y1) in vertical_segments.get(&x).unwrap_or(&Vec::new()) {
        //         let y00 = if sweep_in_ys.contains(&y0) {
        //             y0+1
        //         } else {
        //             y0
        //         };
        //         let y11 = if sweep_in_ys.contains(&y1) {
        //             y1-1
        //         } else {
        //             y1
        //         };
        //         for y in y00..=y11 {
        //             color_tiles.insert((x,y));
        //             if sweep_in_ys.contains(&y) {
        //                 sweep_in_ys.remove(&y);
        //             } else {
        //                 sweep_in_ys.insert(y);
        //             }
        //         }
        //     }
        //     // println!("sweep_in_ys {:?}", sweep_in_ys);
        // }
        //
        // // println!("{:?}", color_tiles);
        //
        // let mut biggest_area_left_top: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        // for &(x,y) in &red_tiles {
        //     biggest_area_left_top.insert((x, y), (x, y));
        // }
        //
        // let mut biggest_area = 0;
        //
        // for x in min_x..=max_x {
        //     for y in min_y..=max_y {
        //         if !color_tiles.contains(&(x,y)) {
        //             // Skip uncolored tiles
        //             continue
        //         }
        //         match (color_tiles.contains(&(x-1, y)), biggest_area_left_top.get(&(x-1, y)), color_tiles.contains(&(x, y-1)), biggest_area_left_top.get(&(x, y-1))) {
        //             (false, _, false, _) => continue, // At left top corner, just keep 1
        //             (true, None, _, _) => panic!("Impossible"),
        //             (_, _, true, None) => panic!("Impossible"),
        //             (true, Some(&left_top), false, _) => {
        //                 let mut lowest_red_x = usize::MAX;
        //                 for xx in (0..=x).rev() { // yy goes from y down to 0
        //                     if red_tiles.contains(&(xx, y)) {
        //                         lowest_red_x = xx;
        //                     }
        //                     if !color_tiles.contains(&(xx, y)) {
        //                         break;
        //                     }
        //                 }
        //                 if lowest_red_x == usize::MAX {
        //                     panic!("Unreachable - did not find lowest_red_x for y");
        //                 }
        //                 biggest_area_left_top.insert((x, y), (lowest_red_x, y));
        //             }
        //             (false, _, true, Some(&left_top)) => {
        //                 let mut lowest_red_y = usize::MAX;
        //                 for yy in (0..=y).rev() { // yy goes from y down to 0
        //                     if red_tiles.contains(&(x, yy)) {
        //                         lowest_red_y = yy;
        //                     }
        //                     if !color_tiles.contains(&(x,yy)) {
        //                         break;
        //                     }
        //                 }
        //                 if lowest_red_y == usize::MAX {
        //                     panic!("Unreachable - did not find lowest_red_y for x");
        //                 }
        //                 biggest_area_left_top.insert((x, y), (x, lowest_red_y));
        //             },
        //             (true, Some(&(x1,y1)), true, Some(&(x2,y2)) ) => {
        //                 if x1 < x2 && y1 < y2 {
        //                     biggest_area_left_top.insert((x, y), (x2,y2));
        //                     continue;
        //                 }
        //                 if x1 > x2 && y1 > y2 {
        //                     biggest_area_left_top.insert((x, y), (x1,y1));
        //                     continue;
        //                 }
        //                 let area1 = (x-x1+1)*(y-y1+1);
        //                 let area2 = (x-x2+1)*(y-y2+1);
        //
        //                 if area1 > area2 {
        //                     biggest_area_left_top.insert((x, y), (x1,y1));
        //                 } else {
        //                     biggest_area_left_top.insert((x, y), (x2,y2));
        //                 }
        //             }
        //         }
        //         let &(x1,y1) = biggest_area_left_top.get(&(x,y)).unwrap();
        //         let area = (x-x1+1)*(y-y1+1);
        //         println!("{x}:{y} area {area}");
        //         if red_tiles.contains(&(x,y)) {
        //             if area > biggest_area {
        //                 biggest_area = area;
        //             }
        //         }
        //     }
        // }

        let mut biggest_area = 1;
        for i in 0..red_tiles.len() - 1 {
            let (x1, y1) = red_tiles[i];
            for j in i + 1..red_tiles.len() {
                let (x2, y2) = red_tiles[j];
                let xx1 = x1.min(x2);
                let xx2 = x1.max(x2);
                let yy1 = y1.min(y2);
                let yy2 = y1.max(y2);
                let area = (xx2 - xx1 + 1) * (yy2 - yy1 + 1);
                if area < biggest_area {
                    continue;
                }
                let mut ok = true;
                for k in 0..red_tiles.len() {
                    let (x3, y3) = red_tiles[k];
                    if xx1 < x3 && x3 < xx2 && yy1 < y3 && y3 < yy2 {
                        // Red point inside
                        ok = false;
                        break;
                    }
                    let l = (k + 1) % (red_tiles.len());
                    let (x4, y4) = red_tiles[l];
                    let xxx1 = x3.min(x4);
                    let xxx2 = x3.max(x4);
                    let yyy1 = y3.min(y4);
                    let yyy2 = y3.max(y4);
                    if x3 == x4 && xx1 < x3 && x3 < xx2 {
                        // It's vertical segment
                        if (yyy1 < yy2 && yyy2 > yy1) || (yyy2 > yy1 && yy2 > yyy1) {
                            ok = false;
                            break;
                        }
                    }
                    if y3 == y4 && yy1 < y3 && y3 < yy2 {
                        // It's horizontal segment
                        if (xxx1 < xx2 && xxx2 > xx1) || (xxx2 > xx1 && xx2 > xxx1) {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    if area > biggest_area {
                        println!("Found biggest area so far {area} from {x1}:{y1} to {x2}:{y2}");
                        biggest_area = area;
                    }
                }
            }
        }

        Ok(biggest_area)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
