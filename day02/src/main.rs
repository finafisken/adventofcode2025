use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(data: &str) -> usize {
    let range_with_dash = data.split(',');
    let ranges = range_with_dash.map(|r| {
        let (start, end) = r
            .split_once('-')
            .map(|(s1, s2)| (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap()))
            .unwrap();

        start..=end
    });

    let mut invalid_ids = Vec::new();

    for range in ranges {
        for n in range {
            let num = n.to_string();
            let num_len = num.len();

            if num_len % 2 != 0 {
                // we need an even amount of characters if they repeat
                continue;
            }

            let (left, right) = num.split_at(num_len / 2);

            if left == right {
                invalid_ids.push(n);
            }

            // println!("{} {}; {} == {}", n, left == right, right, left);
        }
    }

    invalid_ids.iter().sum()
}

fn part2(data: &str) -> usize {
    let range_with_dash = data.split(',');
    let ranges = range_with_dash.map(|r| {
        let (start, end) = r
            .split_once('-')
            .map(|(s1, s2)| (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap()))
            .unwrap();

        start..=end
    });

    let mut invalid_ids = Vec::new();

    for range in ranges {
        for n in range {
            let mut invalid = false;
            let num = n.to_string();
            let num_len = num.len();

            for pattern_len in 1..=num_len / 2 {
                // pattern cant repeat if it doesnt fit evenly
                if num_len % pattern_len != 0 {
                    continue;
                }

                let pattern = &num[..pattern_len];

                // repeat pattern x times to num_len and check if equal
                if pattern.repeat(num_len / pattern_len) == num {
                    invalid = true;
                    println!("{} {}; {} x {}", n, invalid, pattern, num_len / pattern_len);
                }
            }

            if invalid {
                invalid_ids.push(n);
            }
        }
    }

    invalid_ids.iter().sum()
}

#[test]
fn test_p1() {
    let data = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(part1(data), 1227775554)
}

#[test]
fn test_p2() {
    let data = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(part2(data), 4174379265)
}
