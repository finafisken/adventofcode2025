use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(data: &str) -> isize {
    let mut current = 50;
    let mut zero_count = 0;

    for line in data.lines() {
        let mut chars = line.chars();
        let dir = chars.next().expect("no dir");
        let num = chars.collect::<String>().parse::<isize>().unwrap();

        if dir == 'L' {
            current -= num;
        } else {
            current += num;
        }

        current = current % 100; // zero based

        if current < 0 {
            current += 100 // zero based
        }

        if current == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn part2(data: &str) -> isize {
    let mut current: isize = 50;
    let mut zero_count = 0;

    for line in data.lines() {
        let mut chars = line.chars();
        let dir = chars.next().expect("no dir");
        let num = chars.collect::<String>().parse::<isize>().unwrap();

        // going left from 0 means stepping to 99, not crossing 0
        let old_lap = if dir == 'L' {
            (current - 1).div_euclid(100)
        } else {
            current.div_euclid(100)
        };

        if dir == 'L' {
            current -= num;
        } else {
            current += num;
        }

        let new_lap = if dir == 'L' {
            (current - 1).div_euclid(100)
        } else {
            current.div_euclid(100)
        };

        zero_count += (new_lap - old_lap).abs();

        current = current.rem_euclid(100);
    }

    zero_count
}

#[test]
fn test_p1() {
    let data = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    assert_eq!(part1(data), 3)
}

#[test]
fn test_p2() {
    let data = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    assert_eq!(part2(data), 6)
}
