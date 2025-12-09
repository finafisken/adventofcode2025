use std::{
    cmp::{max, min},
    collections::HashMap,
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

// Represents a vertical edge in compressed coordinates
#[derive(Debug)]
struct VerticalEdge {
    x: usize,     // compressed x index
    y_min: usize, // compressed y index min
    y_max: usize, // compressed y index max
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

    // unique x,y coordinates
    let mut unique_x: Vec<usize> = red_points.iter().map(|p| p.0).collect();
    let mut unique_y: Vec<usize> = red_points.iter().map(|p| p.1).collect();
    unique_x.sort();
    unique_x.dedup();
    unique_y.sort();
    unique_y.dedup();

    // original coord -> compressed index
    let x_to_idx: HashMap<usize, usize> = unique_x
        .iter()
        .enumerate()
        .map(|(idx, &x)| (x, idx))
        .collect();
    let y_to_idx: HashMap<usize, usize> = unique_y
        .iter()
        .enumerate()
        .map(|(idx, &y)| (y, idx))
        .collect();

    // red points to compressed coordinates
    let compressed_red: Vec<(usize, usize)> = red_points
        .iter()
        .map(|&(x, y)| (x_to_idx[&x], y_to_idx[&y]))
        .collect();

    let grid_width = unique_x.len();
    let grid_height = unique_y.len();

    // 2d grid instead of HashSet for faster access
    let mut valid: Vec<Vec<bool>> = vec![vec![false; grid_height]; grid_width];
    let mut vertical_edges: Vec<VerticalEdge> = Vec::new();

    for i in 0..compressed_red.len() {
        let (x1, y1) = compressed_red[i];
        let (x2, y2) = compressed_red[(i + 1) % compressed_red.len()];

        // red points valid
        valid[x1][y1] = true;
        valid[x2][y2] = true;

        if x1 == x2 {
            vertical_edges.push(VerticalEdge {
                x: x1,
                y_min: min(y1, y2),
                y_max: max(y1, y2),
            });
            for y in min(y1, y2)..=max(y1, y2) {
                valid[x1][y] = true;
            }
        } else {
            for x in min(x1, x2)..=max(x1, x2) {
                valid[x][y1] = true;
            }
        }
    }

    // fill interior using scanline algorithm (in compressed space)
    for y in 0..grid_height {
        let mut crossings: Vec<usize> = vertical_edges
            .iter()
            .filter(|edge| edge.y_min <= y && y < edge.y_max)
            .map(|edge| edge.x)
            .collect();

        crossings.sort();

        for chunk in crossings.chunks(2) {
            if chunk.len() == 2 {
                let x_start = chunk[0];
                let x_end = chunk[1];
                for x in x_start..=x_end {
                    valid[x][y] = true;
                }
            }
        }
    }

    // find largest valid rectangle with red corners
    let mut max_area: usize = 0;

    for i in 0..compressed_red.len() {
        for j in (i + 1)..compressed_red.len() {
            let (cx1, cy1) = compressed_red[i];
            let (cx2, cy2) = compressed_red[j];

            let x_min = min(cx1, cx2);
            let x_max = max(cx1, cx2);
            let y_min = min(cy1, cy2);
            let y_max = max(cy1, cy2);

            // check if all cells in compressed rectangle are valid
            let all_valid = (x_min..=x_max).all(|x| (y_min..=y_max).all(|y| valid[x][y]));

            if all_valid {
                // calculate actual area using original coords
                let real_x1 = unique_x[x_min];
                let real_x2 = unique_x[x_max];
                let real_y1 = unique_y[y_min];
                let real_y2 = unique_y[y_max];

                let area = (real_x2 - real_x1 + 1) * (real_y2 - real_y1 + 1);
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
