use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(data: &str) -> usize {
    let mut split_count = 0;
    let mut space: HashMap<(usize, usize), char> = HashMap::new();

    for (y, row) in data.lines().enumerate() {
        for (x, val) in row.chars().enumerate() {
            let beam_above = y != 0 && space.get(&(x, y - 1)) == Some(&'|');
            if val == 'S' {
                // starting beam below S
                space.insert((x, y + 1), '|');
            } else if val == '^' {
                // hit splitter with | above it
                if beam_above {
                    // fill out new lasers
                    space.insert((x - 1, y), '|');
                    space.insert((x + 1, y), '|');
                    space.insert((x - 1, y + 1), '|');
                    space.insert((x + 1, y + 1), '|');

                    split_count += 1
                }
            } else if beam_above {
                // beam continues
                space.insert((x, y), '|');
            }
        }
    }

    split_count
}

fn part2(data: &str) -> usize {
    let width = data.lines().next().unwrap().len();

    // track how many timelines have a particle at each column
    let mut timelines: HashMap<usize, usize> = HashMap::new();
    let mut total_timelines = 0;

    for row in data.lines() {
        let mut next_timelines: HashMap<usize, usize> = HashMap::new();

        for (x, val) in row.chars().enumerate() {
            if val == 'S' {
                // start with 1 timeline at this column
                next_timelines.insert(x, 1);
            } else if val == '^' {
                // check if any timelines have particles arriving here
                if let Some(&count) = timelines.get(&x) {
                    // timeline splits: count timelines go left, count go right
                    if x > 0 {
                        *next_timelines.entry(x - 1).or_insert(0) += count;
                    } else {
                        // left edge
                        total_timelines += count;
                    }
                    if x + 1 < width {
                        *next_timelines.entry(x + 1).or_insert(0) += count;
                    } else {
                        // right edge
                        total_timelines += count;
                    }
                }
            } else if val == '.' {
                if let Some(&count) = timelines.get(&x) {
                    *next_timelines.entry(x).or_insert(0) += count;
                }
            }
        }

        timelines = next_timelines;
    }

    // add timelines still active (exited bottom of grid)
    total_timelines += timelines.values().sum::<usize>();
    total_timelines
}

#[test]
fn test_p1() {
    let data = r".......S.......
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
...............";

    assert_eq!(part1(data), 21)
}

#[test]
fn test_p2() {
    let data = r".......S.......
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
...............";

    assert_eq!(part2(data), 40)
}
