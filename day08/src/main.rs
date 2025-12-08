use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

type PointIndex = usize;
type Distance = usize;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> Distance {
        // use squared euclidean distance to avoid sqrt op
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

struct UnionFind {
    parent: Vec<PointIndex>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for i in 0..size {
            parent[i] = i; // Each element is initially its own parent
        }

        UnionFind {
            parent,
            size: vec![1; size],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let mut i = i;
        while self.parent[i] != i {
            i = self.parent[i]
        }
        return i;
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
        }
    }
}

fn part1(data: &str) -> usize {
    let mut points: Vec<Point> = Vec::new();
    let mut distance_between: Vec<(PointIndex, PointIndex, Distance)> = Vec::new();
    for row in data.lines() {
        let vals = row
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        points.push(Point {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        });
    }

    for p1_index in 0..points.len() {
        for p2_index in (p1_index + 1)..points.len() {
            // calculate distance between all pair of points
            distance_between.push((
                p1_index,
                p2_index,
                points[p1_index].distance(&points[p2_index]),
            ));
        }
    }

    distance_between.sort_by_key(|&(_, _, distance)| distance);

    let mut uf = UnionFind::new(points.len());

    #[cfg(not(test))]
    let connections_to_make = 1000;

    #[cfg(test)]
    let connections_to_make = 10;

    // 1000 (10 in test case) shortest distance pairs, union them to build curcuits
    for (p1_idx, p2_idx, _distance) in distance_between.iter().take(connections_to_make) {
        uf.union(*p1_idx, *p2_idx);
    }

    let mut curcuits: HashMap<usize, usize> = HashMap::new();

    for point_idx in 0..points.len() {
        let parent = uf.find(point_idx);
        *curcuits.entry(parent).or_insert(0) += 1;
    }

    let mut sorted_curcuits_by_size: Vec<usize> = curcuits.values().cloned().collect();

    // descending, largest first
    sorted_curcuits_by_size.sort();
    sorted_curcuits_by_size.reverse();

    sorted_curcuits_by_size.iter().take(3).product()
}

fn part2(data: &str) -> usize {
    let mut points: Vec<Point> = Vec::new();
    let mut distance_between: Vec<(PointIndex, PointIndex, Distance)> = Vec::new();
    for row in data.lines() {
        let vals = row
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        points.push(Point {
            x: vals[0],
            y: vals[1],
            z: vals[2],
        });
    }

    for p1_index in 0..points.len() {
        for p2_index in (p1_index + 1)..points.len() {
            // calculate distance between all pair of points
            distance_between.push((
                p1_index,
                p2_index,
                points[p1_index].distance(&points[p2_index]),
            ));
        }
    }

    distance_between.sort_by_key(|&(_, _, distance)| distance);

    let mut uf = UnionFind::new(points.len());

    let mut last_connection = (0, 0);
    for (p1_idx, p2_idx, _distance) in distance_between.iter() {
        // actual merge of different circuits
        if uf.find(*p1_idx) != uf.find(*p2_idx) {
            uf.union(*p1_idx, *p2_idx);
            last_connection = (*p1_idx, *p2_idx);
        }
    }

    let p1 = &points[last_connection.0];
    let p2 = &points[last_connection.1];

    p1.x * p2.x
}

#[test]
fn test_p1() {
    let data = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    assert_eq!(part1(data), 40)
}

#[test]
fn test_p2() {
    let data = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    assert_eq!(part2(data), 25272)
}
