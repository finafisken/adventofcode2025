use core::num;
use std::{cmp::max, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
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

        let problem_sum = problem.iter().fold(initial, |sum, val| {
            let n = val.parse::<usize>().unwrap();
            match operation {
                "*" => sum * n,
                "+" => sum + n,
                _ => unreachable!("bad input"),
            }
        });

        totals.push(problem_sum);
    }

    totals.iter().sum()
}

fn part2(data: &str) -> usize {
    #[derive(Debug)]
    struct ProblemSpec<'a> {
        symbol: &'a str,
        start: usize,
        end: usize,
        width: usize,
    }

    let mut totals: Vec<usize> = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for row in data.lines() {
        let cols: Vec<char> = row.chars().collect();
        grid.push(cols)
    }

    let mut specs: Vec<ProblemSpec> = Vec::new();

    // remove last row from grid, parse to problem specification
    for (i, char) in grid.pop().unwrap().iter().enumerate() {
        if let Some(prev_prob) = specs.last_mut() {
            prev_prob.end = i - 1;
            prev_prob.width = i - prev_prob.start - 1;
        }

        match *char {
            '+' => specs.push(ProblemSpec {
                symbol: "+",
                start: i,
                end: 0,
                width: 0,
            }),
            '*' => specs.push(ProblemSpec {
                symbol: "*",
                start: i,
                end: 0,
                width: 0,
            }),
            _ => (),
        };
    }
    println!("{:?}", specs);

    for spec in specs {
        let mut num_s: Vec<String> = vec![String::new(); spec.width];

        for row in grid.iter() {
            // only grab nr from our spec, start from top left
            for (x, col) in row[spec.start..spec.end].iter().enumerate() {
                if *col == ' ' {
                    continue;
                }

                // println!("{:?}", num_s);
                num_s[x].push(*col);
            }
        }

        println!("{:?}", num_s);

        let initial = match spec.symbol {
            "*" => 1,
            "+" => 0,
            _ => unreachable!("bad input"),
        };

        let problem_sum = num_s.iter().fold(initial, |sum, val| {
            let n = val.parse::<usize>().unwrap();
            match spec.symbol {
                "*" => sum * n,
                "+" => sum + n,
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

#[test]
fn test_p2() {
    let data = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    assert_eq!(part2(data), 3263827)
}
