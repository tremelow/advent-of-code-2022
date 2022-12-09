use std::{fs, ops::{AddAssign, Sub}};
const INPUT_FILE: &str = "data/input09.txt";

use nalgebra::{DMatrix, Vector2};
use ndarray::AssignElem;
use std::cmp::{min,max};

const L: Vector2<i32> = Vector2::new(-1, 0);
const R: Vector2<i32> = Vector2::new( 1, 0);
const U: Vector2<i32> = Vector2::new(0, 1);
const D: Vector2<i32> = Vector2::new( 0, -1);
const Z: Vector2<i32> = Vector2::new(0,0);

fn parse_instruction(instr: &str) -> (i32,Vector2<i32>) {
    return match (instr.chars().nth(0), &instr[2..]) {
        (Some('R'), n) => (n.parse().unwrap(), R),
        (Some('L'), n) => (n.parse().unwrap(), L),
        (Some('U'), n) => (n.parse().unwrap(), U),
        (Some('D'), n) => (n.parse().unwrap(), D),
        _ => (0,Z),
    };
}

/// Compute the bounding box of the Head, which contains a bounding box of the tail
fn parse_bounding_box(instructions: &str) -> (Vector2<i32>, Vector2<i32>) {
    let mut pos = Vector2::new(0,0);
    let mut pmin = pos.clone();
    let mut pmax = pos.clone();
    for instr in instructions.lines() {
        let (n,dir) = parse_instruction(instr);
        pos += n * dir;
        pmin.zip_apply(&pos, |a,b| *a = min(*a,b));
        pmax.zip_apply(&pos, |a,b| *a = max(*a,b));
    }
    return (pmin, pmax);
}

fn compute_tail_movement(phead: &Vector2<i32>, ptail: &Vector2<i32>) -> Vector2<i32> {
    let mut dir_vec = phead.sub(ptail);
    let amplitude = dir_vec.amax();
    assert!(amplitude <= 2, "Direction vector too large??!");
    if amplitude == 2 {
        dir_vec.apply(|x| if *x != 0 {*x = *x / x.abs()});
    } else {
        dir_vec.assign_elem(Z);
    }
    return dir_vec;
}

fn apply_instruction(
    instr: &str, 
    phead: &mut Vector2<i32>, 
    ptail: &mut Vector2<i32>,
    history_ptail: &mut DMatrix<i32>,
) {
    let (n,dir) = parse_instruction(instr);
    for _ in 0..n {
        phead.add_assign(dir);
        ptail.add_assign(compute_tail_movement(&phead, &ptail));
        history_ptail[(ptail[0] as usize, ptail[1] as usize)] += 1;
    }
}

fn get_tail_positions(instructions: &str) -> DMatrix<i32> {
    let (pmin, pmax) = parse_bounding_box(instructions);

    let initial_pos = -pmin;
    let mut phead = initial_pos.clone();
    let mut ptail = initial_pos.clone();
    
    let box_size = pmax.add_scalar(1).sub(pmin).map(|x| x as usize);
    println!("Size of the bounding box: {} by {}", box_size[0], box_size[1]);
    let mut history_ptail = DMatrix::from_element(box_size[0], box_size[1], 0);

    for instr in instructions.lines() {
        apply_instruction(instr, &mut phead, &mut ptail, &mut history_ptail);
    }
    // println!("{}", history_ptail);

    return history_ptail;
}

fn get_whiptail_positions(instructions: &str) -> DMatrix<i32> {
    let length_tail: usize = 10;
    let (pmin, pmax) = parse_bounding_box(instructions);
    let initial_pos = -pmin;
    let mut all_pos: [Vector2<i32>; 10] = [initial_pos.clone(); 10];
    
    let box_size = pmax.add_scalar(1).sub(pmin).map(|x| x as usize);
    println!("Size of the bounding box: {} by {}", box_size[0], box_size[1]);
    let mut history_ptail = DMatrix::from_element(box_size[0], box_size[1], 0);

    for instr in instructions.lines() {
        let (n,dir) = parse_instruction(instr);
        for _ in 0..n {
            all_pos[0].add_assign(dir);
            for i in 1..length_tail {
                let movement = compute_tail_movement(&all_pos[i-1], &all_pos[i]);
                all_pos[i].add_assign(movement);
            }
            let ptail = all_pos.last().unwrap();
            history_ptail[(ptail[0] as usize, ptail[1] as usize)] += 1;
        }
    }
    return history_ptail;
}

pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let history_ptail = get_tail_positions(&contents);

    return history_ptail.map(|x| (x > 0) as u32).sum();
}

pub fn main_bonus() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
    
    let history_ptail = get_whiptail_positions(&contents);
    // let display_history = history_ptail.map(|x| if x == 0 {"."} else {"#"});
    // println!("{}", display_history);
    return history_ptail.map(|x| (x > 0) as u32).sum();
}