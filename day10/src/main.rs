use std::{cmp::min, fs, u32};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    // let p2 = part2(&input);

    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
}

fn part1(data: &str) -> usize {
    let mut min_pressed_total = 0;
    for row in data.lines() {
        let mut parts = row.split(" ");

        // parse target pattern bit mask
        let mut target: u16 = 0;
        let mut pos = 0;
        for c in parts.next().clone().unwrap().chars() {
            match c {
                '.' => pos += 1,
                '#' => {
                    target |= 1 << pos;
                    pos += 1;
                }
                _ => (),
            }
        }

        let mut button_masks: Vec<u16> = Vec::new();

        for part in parts {
            if part.contains("{") {
                break;
            }

            let mut button_mask: u16 = 0;

            let trimmed = &part[1..part.len() - 1];

            for index in trimmed.split(',') {
                let i: u8 = index.parse().unwrap();
                // set index to bit, eg (0,3,4) 0b00011001
                button_mask |= 1 << i;
            }

            button_masks.push(button_mask)
        }

        let nr_of_buttons = button_masks.len();
        // there is 2^nr_of_buttons combinations
        let two_pow_n = 1u32 << nr_of_buttons;

        let mut min_pressed_buttons = u32::MAX;
        for combo in 0..two_pow_n {
            let mut result = 0;

            // check all buttons
            for i in 0..nr_of_buttons {
                // extract bit nr i from combo
                if (combo >> i & 1) != 0 {
                    result ^= button_masks[i];
                }
            }

            if result == target {
                min_pressed_buttons = min(min_pressed_buttons, combo.count_ones());
            }
        }

        min_pressed_total += min_pressed_buttons
    }

    min_pressed_total as usize
}

#[test]
fn test_p1() {
    let data = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    assert_eq!(part1(data), 7)
}
