use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let data = BufReader::new(file);

    let p1 = part1(data);

    println!("Part 1: {}", p1);
}

fn part1<R: BufRead>(data: R) -> isize {
    let mut current = 50;
    let mut zero_count = 0;

    for line in data.lines().flatten() {
        let mut chars = line.chars();
        let dir = chars.next().expect("no dir");
        let num = chars.collect::<String>().parse::<isize>().unwrap();

        // println!("dir: {}, num: {}, current: {}", dir, num, current);

        if dir == 'L' {
            current -= num;
        } else {
            current += num;
        }

        // println!("new: {}", current);

        current = current % 100; // zero based

        if current < 0 {
            current += 100 // zero based
        }

        // println!("normalized: {}", current);

        if current == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

#[test]
fn test_p1() {
    use std::io::Cursor;

    let text = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    let data = Cursor::new(text);

    assert_eq!(part1(data), 3)
}
