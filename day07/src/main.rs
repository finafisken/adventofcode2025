use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    // let p2 = part2(&input);

    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
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
