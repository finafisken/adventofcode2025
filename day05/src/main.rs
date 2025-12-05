use std::{cmp::max, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(data: &str) -> usize {
    let (ranges, ids) = data.split_once("\n\n").unwrap();
    let fresh_ranges: Vec<std::ops::RangeInclusive<usize>> = ranges
        .lines()
        .filter_map(|r| {
            let (start, end) = r.split_once("-").unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            Some(start..=end)
        })
        .collect();

    let mut fresh_count = 0;

    for id in ids.lines() {
        let id_num: usize = id.parse().unwrap();
        let mut fresh = false;
        for range in &fresh_ranges {
            if range.contains(&id_num) {
                fresh = true;
                break;
            }
        }

        if fresh {
            fresh_count += 1;
        }
    }

    fresh_count
}

fn part2(data: &str) -> usize {
    #[derive(Clone)]
    struct SimpleRange {
        start: usize,
        end: usize,
    }

    let (ranges, _) = data.split_once("\n\n").unwrap();
    let mut fresh_ranges: Vec<SimpleRange> = ranges
        .lines()
        .filter_map(|r| {
            let (start, end) = r.split_once("-").unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            Some(SimpleRange { start, end })
        })
        .collect();

    // sort to simplify merge
    fresh_ranges.sort_by_key(|range| range.start);

    // merge ranges to reduce compute and dedupe
    let mut merged_ranges = vec![fresh_ranges[0].clone()];

    for SimpleRange { start, end } in fresh_ranges[1..].iter() {
        let Some(current_range) = merged_ranges.last_mut() else {
            break;
        };

        // overlaps
        if current_range.end >= *start {
            current_range.end = max(current_range.end, *end)
        } else {
            // add to merged ranges
            merged_ranges.push(SimpleRange {
                start: *start,
                end: *end,
            })
        }
    }

    merged_ranges
        .iter()
        .map(|SimpleRange { start, end }| end - start + 1)
        .sum()
}

#[test]
fn test_p1() {
    let data = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    assert_eq!(part1(data), 3)
}

#[test]
fn test_p2() {
    let data = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    assert_eq!(part2(data), 14)
}
