use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    // let p2 = part2(&input);

    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
}

fn part1(data: &str) -> usize {
    let mut values_on_col: HashMap<usize, Vec<&str>> = HashMap::new();
    for row in data.lines() {
        for (index, val) in row.split_whitespace().enumerate() {
            values_on_col.entry(index).or_insert(Vec::new()).push(val);
        }
    }

    let mut totals: Vec<usize> = Vec::new();

    for (_, problem) in values_on_col.iter_mut() {
        let operation = problem.pop().unwrap();

        let initial = match operation {
            "*" => 1,
            "+" => 0,
            _ => unreachable!("bad input"),
        };

        let problem_sum = problem.iter().fold(initial, |acc, val| {
            let n = val.parse::<usize>().unwrap();
            match operation {
                "*" => acc * n,
                "+" => acc + n,
                _ => unreachable!("bad input"),
            }
        });

        totals.push(problem_sum);
    }

    totals.iter().sum()
}

#[test]
fn test_p1() {
    let data = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    assert_eq!(part1(data), 4277556)
}
