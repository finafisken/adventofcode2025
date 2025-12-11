use std::{cmp::min, fs, u32};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(data: &str) -> usize {
    let mut min_pressed_total = 0;
    for row in data.lines() {
        let mut parts = row.split(" ");

        // parse target pattern bit mask
        let mut target: u16 = 0;
        let mut pos = 0;
        for c in parts.next().clone().unwrap().chars() {
            match c {
                '.' => pos += 1,
                '#' => {
                    target |= 1 << pos;
                    pos += 1;
                }
                _ => (),
            }
        }

        let mut button_masks: Vec<u16> = Vec::new();

        for part in parts {
            if part.contains("{") {
                break;
            }

            let mut button_mask: u16 = 0;

            let trimmed = &part[1..part.len() - 1];

            for index in trimmed.split(',') {
                let i: u8 = index.parse().unwrap();
                // set index to bit, eg (0,3,4) 0b00011001
                button_mask |= 1 << i;
            }

            button_masks.push(button_mask)
        }

        let nr_of_buttons = button_masks.len();
        // there is 2^nr_of_buttons combinations
        let two_pow_n = 1u32 << nr_of_buttons;

        let mut min_pressed_buttons = u32::MAX;
        for combo in 0..two_pow_n {
            let mut result = 0;

            // check all buttons
            for i in 0..nr_of_buttons {
                // extract bit nr i from combo
                if (combo >> i & 1) != 0 {
                    // XOR button mask with result
                    result ^= button_masks[i];
                }
            }

            if result == target {
                min_pressed_buttons = min(min_pressed_buttons, combo.count_ones());
            }
        }

        min_pressed_total += min_pressed_buttons
    }

    min_pressed_total as usize
}

fn part2(data: &str) -> usize {
    let mut min_pressed_total = 0;
    for row in data.lines() {
        let mut buttons: Vec<Vec<u8>> = Vec::new();
        let mut joltages: Vec<usize> = Vec::new();

        // skip first item
        for item in row.split(" ").skip(1) {
            // button
            if item.contains("(") {
                let trimmed = &item[1..item.len() - 1];
                let mut button: Vec<u8> = Vec::new();
                for nr in trimmed.split(',') {
                    button.push(nr.parse().unwrap())
                }
                buttons.push(button);
            } else {
                // joltage
                let trimmed = &item[1..item.len() - 1];
                for nr in trimmed.split(',') {
                    joltages.push(nr.parse().unwrap())
                }
            }
        }

        // Build matrix A where A[counter][button] = 1 if button affects counter
        let num_counters = joltages.len();
        let num_buttons = buttons.len();

        // Use Gaussian elimination to solve Ax = b (over rationals, then check integer)
        // A is num_counters x num_buttons, x is button presses, b is joltages

        let min_presses = solve_linear(&buttons, &joltages, num_counters, num_buttons);
        min_pressed_total += min_presses;
    }

    min_pressed_total
}

fn solve_linear(
    buttons: &Vec<Vec<u8>>,
    targets: &Vec<usize>,
    num_counters: usize,
    num_buttons: usize,
) -> usize {
    // Build augmented matrix [A | b] using rationals (as f64 for simplicity)
    let mut matrix: Vec<Vec<f64>> = vec![vec![0.0; num_buttons + 1]; num_counters];

    for counter in 0..num_counters {
        for (btn_idx, button) in buttons.iter().enumerate() {
            if button.contains(&(counter as u8)) {
                matrix[counter][btn_idx] = 1.0;
            }
        }
        matrix[counter][num_buttons] = targets[counter] as f64;
    }

    // Gaussian elimination with partial pivoting
    let mut pivot_col = 0;
    let mut pivot_row = 0;
    let mut pivot_cols = vec![];

    while pivot_row < num_counters && pivot_col < num_buttons {
        // Find pivot
        let mut max_row = pivot_row;
        for row in pivot_row + 1..num_counters {
            if matrix[row][pivot_col].abs() > matrix[max_row][pivot_col].abs() {
                max_row = row;
            }
        }

        if matrix[max_row][pivot_col].abs() < 1e-10 {
            pivot_col += 1;
            continue;
        }

        matrix.swap(pivot_row, max_row);
        pivot_cols.push(pivot_col);

        // Eliminate below
        for row in pivot_row + 1..num_counters {
            let factor = matrix[row][pivot_col] / matrix[pivot_row][pivot_col];
            for col in pivot_col..=num_buttons {
                matrix[row][col] -= factor * matrix[pivot_row][col];
            }
        }

        pivot_row += 1;
        pivot_col += 1;
    }

    let rank = pivot_cols.len();
    let num_free = num_buttons - rank;

    // Back substitution to get particular solution (with free vars = 0)
    let mut solution = vec![0.0; num_buttons];

    for i in (0..rank).rev() {
        let pc = pivot_cols[i];
        let mut sum = matrix[i][num_buttons];
        for j in pc + 1..num_buttons {
            sum -= matrix[i][j] * solution[j];
        }
        solution[pc] = sum / matrix[i][pc];
    }

    // Check for non-integer or negative solutions in basic vars
    // If we have free variables, we need to search
    if num_free == 0 {
        // Unique solution - check if valid
        let mut total = 0.0;
        for &x in &solution {
            if x < -1e-9 || (x - x.round()).abs() > 1e-9 {
                return usize::MAX; // No valid solution
            }
            total += x.round();
        }
        return total as usize;
    }

    // We have free variables - need to search over them
    // Free variables are columns not in pivot_cols
    let free_vars: Vec<usize> = (0..num_buttons)
        .filter(|c| !pivot_cols.contains(c))
        .collect();

    // For each free variable, find its coefficient in each basic variable
    // Then search over free variable values

    // Compute null space basis vectors
    let mut null_basis: Vec<Vec<f64>> = vec![];

    for &free_col in &free_vars {
        let mut null_vec = vec![0.0; num_buttons];
        null_vec[free_col] = 1.0;

        // Back-substitute to find effect on pivot vars
        for i in (0..rank).rev() {
            let pc = pivot_cols[i];
            let mut sum = 0.0;
            for j in pc + 1..num_buttons {
                sum += matrix[i][j] * null_vec[j];
            }
            null_vec[pc] = -sum / matrix[i][pc];
        }
        null_basis.push(null_vec);
    }

    // Search over combinations of free variables
    // Limit search range based on targets
    let max_search = targets.iter().max().unwrap_or(&0) + 1;

    let mut best = usize::MAX;
    search_free_vars(
        &solution,
        &null_basis,
        0,
        vec![0i64; num_free],
        max_search as i64,
        &mut best,
    );

    best
}

fn search_free_vars(
    particular: &Vec<f64>,
    null_basis: &Vec<Vec<f64>>,
    idx: usize,
    coeffs: Vec<i64>,
    max_val: i64,
    best: &mut usize,
) {
    if idx == null_basis.len() {
        // Evaluate solution
        let mut solution = particular.clone();
        for (i, &c) in coeffs.iter().enumerate() {
            for j in 0..solution.len() {
                solution[j] += c as f64 * null_basis[i][j];
            }
        }

        // Check validity and compute sum
        let mut total = 0usize;
        for &x in &solution {
            let rounded = x.round();
            if rounded < -0.5 || (x - rounded).abs() > 1e-6 {
                return; // Invalid
            }
            if rounded < 0.0 {
                return;
            }
            total += rounded as usize;
        }

        *best = (*best).min(total);
        return;
    }

    // Try values for this free variable
    // Estimate reasonable range
    for c in -max_val..=max_val {
        let mut new_coeffs = coeffs.clone();
        new_coeffs[idx] = c;
        search_free_vars(particular, null_basis, idx + 1, new_coeffs, max_val, best);
    }
}

#[test]
fn test_p1() {
    let data = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    assert_eq!(part1(data), 7)
}

#[test]
fn test_p2() {
    let data = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    assert_eq!(part2(data), 33)
}
