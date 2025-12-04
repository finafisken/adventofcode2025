use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    // let p2 = part2(&input);

    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
}

fn part1(data: &str) -> usize {
    let mut unblocked_count = 0;
    let mut coords: HashMap<(i16, i16), char> = HashMap::new();
    for (row_idx, line) in data.lines().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            match char {
                '@' => coords.insert((col_idx as i16, row_idx as i16), '@'),
                _ => continue,
            };
        }
    }

    for (x, y) in coords.keys() {
        let mut adjecent = 0;

        for y_i in (y - 1)..=(y + 1) {
            for x_i in (x - 1)..=(x + 1) {
                // check if potential adjecent roll exists and is not itself
                if coords.contains_key(&(x_i, y_i)) && (x_i, y_i) != (*x, *y) {
                    adjecent += 1;
                }
            }
        }

        if adjecent < 4 {
            unblocked_count += 1;
        }
    }

    unblocked_count
}

#[test]
fn test_p1() {
    let data = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    assert_eq!(part1(data), 13)
}

// #[test]
// fn test_p2() {
//     let data = r"987654321111111
// 811111111111119
// 234234234234278
// 818181911112111";

//     assert_eq!(part2(data), 3121910778619)
// }
