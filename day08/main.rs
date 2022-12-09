use std::collections::HashSet;

// Input:
// 30373
// 25512
// 65332
// 33549
// 35390

// Plan:
// 1. Count the number of visible trees in a line.
//    If a tree is taller than the previous, count it as visible from this direction
// 2. Repeat for rows and columns in both directions; skip first and last row/column as all trees are visible on the edge
// 3. Count edge trees as visible, don't count corners twice

fn a(num_rows: usize, num_cols: usize, trees: &[char]) -> usize {
    let tree_at = |row: usize, col: usize| {
        trees[row * num_cols + col]
    };
    let mut visible = HashSet::new();

    // rows forward
    for row in 1..(num_rows - 1) {
        let mut tallest = tree_at(row, 0); // first tree in row
        for col in 1..(num_cols - 1) {
            let tree_height = tree_at(row, col);
            if tree_height > tallest {
                visible.insert((row, col));
                tallest = tree_height;
            }
        }
    }

    // rows backward
    for row in 1..(num_rows - 1) {
        let mut tallest = tree_at(row, num_cols - 1); // last tree in row
        for col in (1..(num_cols - 1)).rev() {
            let tree_height = tree_at(row, col);
            if tree_height > tallest {
                visible.insert((row, col));
                tallest = tree_height;
            }
        }
    }

    // columns forward
    for col in 1..(num_cols - 1) {
        let mut tallest = tree_at(0, col); // first tree in column
        for row in 1..(num_rows - 1) {
            let tree_height = tree_at(row, col);
            if tree_height > tallest {
                visible.insert((row, col));
                tallest = tree_height;
            }
        }
    }

    // columns backward
    for col in 1..(num_cols - 1) {
        let mut tallest = tree_at(num_rows - 1, col); // last tree in column
        for row in (1..(num_rows - 1)).rev() {
            let tree_height = tree_at(row, col);
            if tree_height > tallest {
                visible.insert((row, col));
                tallest = tree_height;
            }
        }
    }

    // count edge trees
    let visible_count = visible.len() + 2 * (num_rows + num_cols) - 4;
    visible_count
}

// Plan:
// 1. Iterate through all trees
// 2. Look in all directions and count trees. Stop at tree of same height or edge
// 3. Collect max scenic score

fn b(num_rows: usize, num_cols: usize, trees: &[char]) -> usize {
    let tree_at = |row: usize, col: usize| {
        trees[row * num_cols + col]
    };

    let mut highest_scenic_score = 0;
    for (index, &tree_height) in trees.iter().enumerate() {
        let row = index / num_cols;
        let col = index % num_cols;
        
        // look up
        let mut visible_up = 0;
        for up_row in (0..row).rev() {
            let seen_height = tree_at(up_row, col);
            visible_up += 1;
            if seen_height >= tree_height {
                break;
            }
        }

        // look down
        let mut visible_down = 0;
        for down_row in (row + 1)..num_rows {
            let seen_height = tree_at(down_row, col);
            visible_down += 1;
            if seen_height >= tree_height {
                break;
            }
        }

        // look left
        let mut visible_left = 0;
        for left_col in (0..col).rev() {
            let seen_height = tree_at(row, left_col);
            visible_left += 1;
            if seen_height >= tree_height {
                break;
            }
        }

        // look right
        let mut visible_right = 0;
        for right_col in (col + 1)..num_cols {
            let seen_height = tree_at(row, right_col);
            visible_right += 1;
            if seen_height >= tree_height {
                break;
            }
        }

        let scenic_score = visible_up * visible_down * visible_left * visible_right;
        if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
        }
    }
    highest_scenic_score
}

pub fn main() {
    let input = include_str!("./input.txt");
    let num_rows = input.find(char::is_whitespace).expect("multi-line input"); // assume lines of equal length
    let num_cols = input.matches("\n").count();
    let trees: Vec<_> = input
        .as_bytes()
        .iter()
        .map(|&code| code as char)
        .filter(|ch| !ch.is_whitespace())
        .collect();
    
    println!("day08a: {}", a(num_rows, num_cols, &trees));
    println!("day08b: {}", b(num_rows, num_cols, &trees));
}
