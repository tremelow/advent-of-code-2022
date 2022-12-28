use std::fs;
const INPUT_FILE: &str = "data/test17.txt";

use std::iter::Cycle;
use std::collections::VecDeque;
use std::ops::{ShlAssign, ShrAssign};
use std::cmp::{max,min};

use core::slice::Iter;
use itertools::Itertools;

const NUMBER_OF_ROCKS: u64 = 1_000_000;
// const NUMBER_OF_ROCKS: u64 = 1_000_000_000_000;

#[derive(Clone, Copy, Debug)]
enum LeftOrRight {
    Left,
    Right,
}
impl LeftOrRight {
    fn from(c: char) -> Self {
        match c {
            '<' => LeftOrRight::Left,
            '>' => LeftOrRight::Right,
            _   => panic!(),
        }
    }
    fn opposite(self) -> Self {
        match self {
            LeftOrRight::Left  => LeftOrRight::Right,
            LeftOrRight::Right => LeftOrRight::Left,
        }
    }
}

/// Shapes are represented by 4 rows of 8 colums (the leftmost column being inaccessible),
/// which is "compressed" into a single u32.
/// 
/// The total rock formation will be represented by a VecDeque<u32>, where for each u32,
/// only the last 7 bits matter, and a buffer will represent the shape at the same height 
/// as the falling shape.
#[derive(Clone, Copy, Debug)]
struct Shape(u32);

impl Shape {
    fn touches(&self, obstacle: u32) -> bool {
        self.0 & obstacle != 0
    }
    fn move_lr(&mut self, dir: LeftOrRight, obstacle: u32) {
        match dir {
            LeftOrRight::Left  => self.move_left(obstacle),
            LeftOrRight::Right => self.move_right(obstacle),
        }
    }
    fn move_left(&mut self, obstacle: u32) {
        if !self.touches(LEFT_WALL | (obstacle >> 1)) { 
            self.0 <<= 1; 
        }
    }
    fn move_right(&mut self, obstacle: u32) {
        if !self.touches(RIGHT_WALL | (obstacle << 1)) { 
            self.0 <<= 1; 
        }
    }
    fn to_string(&self) -> String {
        let data = self.0;
        let mut out = String::new();
        for height in (0..4).rev() {
            for bit in (0..7).rev() {
                let offset = ROW_SIZE*height + bit;
                out.push(if (data >> offset) & 1 != 0 {'#'} else {'.'});
            }
            if height != 0 {out.push('\n');}
        }
        return out;
    }
}

const ROW_SIZE: u32 = 8;
/// Floor 
const MASK: u32 = 0b1111111;
/// ```
/// 0b01000000 i.e. .#......
///   01000000 i.e. .#......
///   01000000 i.e. .#......
///   01000000 i.e. .#......
/// ```
const LEFT_WALL: u32 = 0x40404040;
/// ```
/// 0b01000000 i.e. .......#
///   01000000 i.e. .......#
///   01000000 i.e. .......#
///   01000000 i.e. .......#
/// ```
const RIGHT_WALL: u32 = 0x01010101;
const FLOOR: Shape = Shape(0b1111111);

const FALLING_SHAPES: [Shape;5] = [
    // 0b00000000, i.e. ........
    //   00000000, i.e. ........
    //   00000000, i.e. ........
    //   00011110, i.e. ...####.
    Shape(0x0000001e),
    // 0b0000000, i.e. ........
    // 0b0001000, i.e. ....#...
    // 0b0011100, i.e. ...###..
    // 0b0001000, i.e. ....#...
    Shape(0x00081c08),
    // 0b0000000, i.e. ........
    // 0b0000100, i.e. .....#..
    // 0b0000100, i.e. .....#..
    // 0b0011100, i.e. ...###..
    Shape(0x0004041c),
    // 0b0010000, i.e. ...#....
    // 0b0010000, i.e. ...#....
    // 0b0010000, i.e. ...#....
    // 0b0010000, i.e. ...#....
    Shape(0x10101010),
    // 0b00000000, i.e. ........
    // 0b00000000, i.e. ........
    // 0b00011000, i.e. ...##...
    // 0b00011000, i.e. ...##...
    Shape(0x00001818),
];

/// Fall 3 times 
fn initial_fall(shape: &mut Shape, directions: &mut Cycle<Iter<LeftOrRight>>) {
    for _ in 0..3 {
        shape.move_lr(*directions.next().unwrap(), 0);
    }
}
fn fall_until_stagnant(
    shape: &mut Shape, 
    directions: &mut Cycle<Iter<LeftOrRight>>,
    rock_formation: &VecDeque<u32>
) -> usize {
    let mut depth = 0;
    let mut buffer = 0;
    // Update depth until falling would mean shape and rock intersect
    loop {
        shape.move_lr(*directions.next().unwrap(), buffer);
        buffer <<= ROW_SIZE;
        buffer |= rock_formation[depth];
        if shape.touches(buffer) {
            return depth;
        }
        depth += 1;
    }
}
fn add_shape_to_rock_formation(
    rock_formation: &mut VecDeque<u32>,
    shape: &mut Shape,
    depth: usize,
) {
    let mut depth = depth;
    for height_above_depth in 1..=4 {
        let shape_row = shape.0 & MASK;
        if shape_row == 0 { break; }
        if depth < height_above_depth {
            rock_formation.push_front(0);
            depth += 1;
        } 
        rock_formation[depth - height_above_depth] |= shape_row;
        shape.0 >>= ROW_SIZE;
    }
}
/// Check whether that means that some previously accessible spots
/// are now inacessible and update rock formation as a consequence.
/// 
/// A spot is *accessible* if the spot above is accessible 
/// **OR** the spot to its left *and* the one above it both are accessible
/// **OR** the spot to its right *and* the one above it both are.
fn update_accessibility(rock_formation: &mut VecDeque<u32>, check_depth: usize) {
    let mut check_deeper = false;
    for depth in 1..min(check_depth,rock_formation.len()) {
        let row_above = rock_formation[depth - 1];
        let row = rock_formation[depth];
        rock_formation[depth] |= row_above & ((row >> 1) | (row_above >> 1)) & ((row << 1) | (row_above << 1)) & MASK;
        if row != rock_formation[depth] {
            check_deeper = true;
        }
    }
    if check_deeper {
        for depth in check_depth..rock_formation.len() {
            let row_above = rock_formation[depth - 1];
            let row = rock_formation[depth];
            rock_formation[depth] |= row_above & ((row >> 1) | (row_above >> 1)) & ((row << 1) | (row_above << 1)) & MASK;
        }
    }
}
fn drop_inaccessible(rock_formation: &mut VecDeque<u32>) -> usize {
    let after_floor = rock_formation.iter().find_position(|row| **row == MASK).unwrap().0 + 1;
    let number_of_removed = rock_formation.len() - after_floor;
    rock_formation.truncate(after_floor);
    // if number_of_removed != 0 { 
    //     println!("Removed {} rows, the height is now {}.", number_of_removed, rock_formation.len()); 
    // }
    return number_of_removed;
}
fn rock_pile_height(
    shapes: &Vec<Shape>,
    directions: &Vec<LeftOrRight>,
    number_of_falls: u64,
) -> usize {
    let mut shapes = shapes.iter().cycle();
    let mut directions = directions.iter().cycle();
    let mut rock_formation = VecDeque::new();
    rock_formation.reserve(256);
    rock_formation.push_back(MASK);

    let mut pile_height = 0;
    let mut max_memory_length = 0;
    for _ in 0..number_of_falls {
        let mut shape = shapes.next().unwrap().clone();
        initial_fall(&mut shape, &mut directions);
        let depth = fall_until_stagnant(&mut shape, &mut directions, &mut rock_formation);
        add_shape_to_rock_formation(&mut rock_formation, &mut shape, depth);
        max_memory_length = max(max_memory_length, rock_formation.len());
        update_accessibility(&mut rock_formation, 10);
        pile_height += drop_inaccessible(&mut rock_formation);
    }
    println!("{}", max_memory_length);

    // // Display
    // let mut out = String::new();
    // for row in &rock_formation {
    //     for offset in (0..7).rev() {
    //         out.push(if (row >> offset) & 1 != 0 {'#'} else {'.'});
    //     }
    //     out.push('\n');
    // }
    // println!("{}", out);


    pile_height += rock_formation.len() - 1;
    return pile_height;
}



pub fn main() -> usize {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let jet_directions = contents.trim().chars()
        .map(LeftOrRight::from)
        .collect_vec(); // hopefully this works like shadowing
    // let mut jet_directions = jet_directions.iter().cycle();
    // let mut shapes = FALLING_SHAPES.iter().cycle();

    let res = rock_pile_height(&FALLING_SHAPES.to_vec(), &jet_directions, NUMBER_OF_ROCKS);

    return res;
}

pub fn main_bonus() {
    let _contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}
