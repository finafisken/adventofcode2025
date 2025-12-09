use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
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

#[derive(Debug)]
struct VerticalEdge {
    x: usize,
    y_min: usize,
    y_max: usize,
}

fn part2(data: &str) -> usize {
    let mut red_points: Vec<Point> = Vec::new();
    for row in data.lines() {
        let coord: Vec<usize> = row
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        red_points.push((coord[0], coord[1]));
    }

    let mut valid_points: HashSet<Point> = HashSet::new();
    let mut vertical_edges: Vec<VerticalEdge> = Vec::new();

    // looks like the input is sorted, there is always x / y aligned with next point
    for i in 0..red_points.len() {
        let (x1, y1) = red_points[i];
        let (x2, y2) = red_points[(i + 1) % red_points.len()];

        // red points are valid points
        valid_points.insert((x1, y1));
        valid_points.insert((x2, y2));

        // find which axis aligns, fill line between start and end
        if x1 == x2 {
            // record for scanline algorithm
            vertical_edges.push(VerticalEdge {
                x: x1,
                y_min: min(y1, y2),
                y_max: max(y1, y2),
            });

            for y in min(y1, y2)..=max(y1, y2) {
                valid_points.insert((x1, y));
            }
        } else {
            for x in min(x1, x2)..=max(x1, x2) {
                valid_points.insert((x, y1));
            }
        }
    }

    // fill using scanline algorithm
    let y_min = red_points.iter().map(|p| p.1).min().unwrap();
    let y_max = red_points.iter().map(|p| p.1).max().unwrap();

    for y in y_min..=y_max {
        // find all vertical edges that this scanline crosses
        let mut crossings: Vec<usize> = vertical_edges
            .iter()
            .filter(|edge| edge.y_min <= y && y < edge.y_max)
            .map(|edge| edge.x)
            .collect();

        crossings.sort();

        // fill between pairs of crossings (inside the polygon)
        for chunk in crossings.chunks(2) {
            if chunk.len() == 2 {
                let x_start = chunk[0];
                let x_end = chunk[1];
                for x in x_start..=x_end {
                    valid_points.insert((x, y));
                }
            }
        }
    }

    // find largest rectangle with red corners where all points are valid
    let mut max_area = 0;

    for i in 0..red_points.len() {
        for j in (i + 1)..red_points.len() {
            let p1 = red_points[i];
            let p2 = red_points[j];

            let x_min = min(p1.0, p2.0);
            let x_max = max(p1.0, p2.0);
            let y_min = min(p1.1, p2.1);
            let y_max = max(p1.1, p2.1);

            // check if all points valid
            let all_valid =
                (x_min..=x_max).all(|x| (y_min..=y_max).all(|y| valid_points.contains(&(x, y))));

            if all_valid {
                let area = (x_max - x_min + 1) * (y_max - y_min + 1);
                max_area = max(max_area, area);
            }
        }
    }

    max_area
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

#[test]
fn test_p2() {
    let data = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    assert_eq!(part2(data), 24)
}
