use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    // let p2 = part2(&input);

    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
}

fn part1(data: &str) -> usize {
    let mut max_jolts: Vec<usize> = Vec::new();
    for line in data.lines() {
        let numbers: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        // find first highest nr + index, leave at least 1 char
        let num_minus_last = &numbers[..numbers.len() - 1];
        let m1 = num_minus_last.iter().max().unwrap();
        // first position of number
        let max_idx = num_minus_last.iter().position(|x| x == m1).unwrap();

        // find next highest nr after first max index +1
        let m2 = numbers[max_idx + 1..].iter().max().unwrap();

        let max: usize = format!("{m1}{m2}").parse().unwrap();

        println!("{m1}{m2} = {max}");

        max_jolts.push(max)
    }

    // println!("{:?}", max_jolts);

    max_jolts.iter().sum()
}

#[test]
fn test_p1() {
    let data = r"987654321111111
811111111111119
234234234234278
818181911112111";

    assert_eq!(part1(data), 357)
}

#[test]
fn test_p1_1() {
    let data = r"9891";

    assert_eq!(part1(data), 99)
}
