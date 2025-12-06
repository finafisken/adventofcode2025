use std::{collections::HashMap, fs};

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
    struct ProblemSpec {
        symbol: char,
        start: usize,
        end: usize, // exclusive
    }

    let mut totals: Vec<usize> = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();

    // build grid and pad rows to same width
    let lines: Vec<&str> = data.lines().collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    for row in &lines {
        let mut cols: Vec<char> = row.chars().collect();
        cols.resize(max_width, ' ');
        grid.push(cols);
    }

    let bottom_row = grid.pop().unwrap(); // remove last row (operators)

    // check if column is a separator (all spaces in all rows)
    let is_separator_col =
        |col: usize| -> bool { grid.iter().all(|row| row[col] == ' ') && bottom_row[col] == ' ' };

    let mut specs: Vec<ProblemSpec> = Vec::new();
    let mut in_problem = false;
    let mut problem_start = 0;
    let mut problem_op = ' ';

    // find problem boundaries
    for i in 0..max_width {
        if is_separator_col(i) {
            if in_problem {
                // end of current problem
                specs.push(ProblemSpec {
                    symbol: problem_op,
                    start: problem_start,
                    end: i,
                });
                in_problem = false;
            }
        } else {
            if !in_problem {
                // start of new problem
                in_problem = true;
                problem_start = i;
                problem_op = ' ';
            }

            if bottom_row[i] == '+' || bottom_row[i] == '*' {
                problem_op = bottom_row[i];
            }
        }
    }
    // finalize last problem if we ended inside one
    if in_problem {
        specs.push(ProblemSpec {
            symbol: problem_op,
            start: problem_start,
            end: max_width,
        });
    }

    for spec in specs {
        let width = spec.end - spec.start;
        let mut num_s: Vec<String> = vec![String::new(); width];

        for row in grid.iter() {
            for (x, col) in row[spec.start..spec.end].iter().enumerate() {
                if col.is_ascii_digit() {
                    num_s[x].push(*col);
                }
            }
        }

        // filter out empty columns
        let operands: Vec<usize> = num_s
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let initial = match spec.symbol {
            '*' => 1,
            '+' => 0,
            _ => unreachable!("bad input"),
        };

        let problem_sum = operands.iter().fold(initial, |sum, val| match spec.symbol {
            '*' => sum * val,
            '+' => sum + val,
            _ => unreachable!("bad input"),
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
