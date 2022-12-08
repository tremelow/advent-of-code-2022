use std::fs;
const INPUT_FILE: &str = "data/input08.txt";

use itertools::izip;
use ndarray::{Array, Array2, ArrayView1, ArrayViewMut1, Axis, s};

fn parse_tree_heights(s: &str) -> Array2<i32> {
    let ncol = s.find("\n").unwrap();
    let nrow = s.lines().count();

    let data: Vec<i32> = s.chars().filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let mat: Array2<i32> = Array::from_shape_vec((nrow, ncol), data).unwrap();

    return mat;
}

fn set_vec_visibility(v: &mut ArrayViewMut1<(i32, bool)>) {
    v.into_iter().fold(-1, |h: i32, x: &mut (i32, bool)| {
        if x.0 > h {
            x.1 = true;
            return x.0;
        } else {
            return h;
        }
    });
}

fn set_vec_score(
    scores_from_left: &mut ArrayViewMut1<i32>, 
    scores_from_right: &mut ArrayViewMut1<i32>, 
    heights: &ArrayView1<i32>) {
    for (i, (&h, sl, sr)) in izip!(heights, scores_from_left, scores_from_right).enumerate() {
        *sl = 0;
        *sr = 0;
        for &h_to_the_left in heights.slice(s![..i]).into_iter().rev() {
            *sr += 1;
            if h_to_the_left >= h {
                break;
            }
        }
        for &h_to_the_right in heights.slice(s![(i+1)..]) {
            *sl += 1;
            if h_to_the_right >= h {
                break;
            }
        }
    }
}

/// Fill the visibility left, right, up and down
fn get_visibility(tree_heights: &Array2<i32>) -> Array2<bool> {
    let mut trees: Array2<(i32, bool)> = tree_heights.map(|x| (*x, false));

    for mut row in trees.axis_iter_mut(Axis(0)) {
        // from the left
        set_vec_visibility(&mut row);
        // revert the direction
        row.invert_axis(Axis(0));
        // from the right
        set_vec_visibility(&mut row);
        // put the vector back in the right direction
        row.invert_axis(Axis(0));
    }
    // Same thing but from up and down
    for mut col in trees.axis_iter_mut(Axis(1)) {
        set_vec_visibility(&mut col);
        col.invert_axis(Axis(0));
        set_vec_visibility(&mut col);
        col.invert_axis(Axis(0));
    }

    return trees.map(|x| x.1);
}

fn get_scores(tree_heights: &Array2<i32>) -> Array2<i32> {
    let mut scores_left: Array2<i32> = 0*tree_heights;
    let mut scores_right: Array2<i32> = 0*tree_heights;
    let mut scores_up: Array2<i32> = 0*tree_heights;
    let mut scores_down: Array2<i32> = 0*tree_heights;

    for (i, row) in tree_heights.axis_iter(Axis(0)).enumerate() {
        set_vec_score(
            &mut scores_left.row_mut(i), 
            &mut scores_right.row_mut(i), 
            &row);
    }
    for (i, col) in tree_heights.axis_iter(Axis(1)).enumerate() {
        set_vec_score(
            &mut scores_up.column_mut(i), 
            &mut scores_down.column_mut(i), 
            &col);
    }
    return scores_left * scores_right * scores_up * scores_down;
}

pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let tree_heights: Array2<i32> = parse_tree_heights(&contents);
    let visibility = get_visibility(&tree_heights);

    // let mut scores: Array1<i32> = Array::from(vec![0,0,0,0,0]);
    // set_vec_score(&mut scores, &Array::from(vec![2,5,5,1,2]));
    // println!("{:?}", scores);

    return visibility.map(|b| *b as u32).sum();
}

pub fn main_bonus() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let tree_heights: Array2<i32> = parse_tree_heights(&contents);
    let scores = get_scores(&tree_heights);
    // println!("{}", scores);

    return scores.into_iter().max().unwrap() as u32;
}
