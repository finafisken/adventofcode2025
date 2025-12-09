use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    // let p2 = part2(&input);

    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
}

type Point = (usize, usize);

fn part1(data: &str) -> usize {
    let mut points: Vec<Point> = Vec::new();
    for row in data.lines() {
        let coord: Vec<usize> = row
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        points.push((coord[0], coord[1]));
    }

    let mut points_rec_size: HashMap<(Point, Point), usize> = HashMap::new();

    // calc size of all
    for p1_idx in 0..points.len() {
        for p2_idx in (p1_idx + 1)..points.len() {
            let p1 = points[p1_idx];
            let p2 = points[p2_idx];

            // add self to x and y before area calc
            let rec_size = (p2.0.abs_diff(p1.0) + 1) * (p2.1.abs_diff(p1.1) + 1);

            points_rec_size.insert((p1, p2), rec_size);
        }
    }

    *points_rec_size.values().max().unwrap()
}

#[test]
fn test_p1() {
    let data = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    assert_eq!(part1(data), 50)
}
