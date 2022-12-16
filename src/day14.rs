use std::{fs, cmp::{max,min}};
const INPUT_FILE: &str = "data/input14.txt";

use itertools::Itertools;
use nalgebra::{DMatrix};
use nom::{self, 
    IResult,
    multi::separated_list1, 
    sequence::separated_pair, 
    character::complete::digit1,
    combinator::map_res,
    bytes::complete::tag,
};

const LEFT_BUFFER: usize = 1;
const RIGHT_BUFFER: usize = 1;
const LOWER_BUFFER: usize = 2;
const SOURCE_POSITION: (usize, usize) = (500,0);
const INFINITE_FLOOR_BUFFER: usize = 10;


type MatIndex = (usize, usize);
type Path = Vec<MatIndex>;

fn str_to_usize(input: &str) -> IResult<&str, usize> {
    return map_res(digit1, str::parse::<usize>)(input);
}

fn parse_path(s: &str) -> Path {
    let parse_matindex = separated_pair(str_to_usize, tag(","), str_to_usize);
    let mut parser = separated_list1(tag(" -> "), parse_matindex);
    if let Ok(("",v)) = parser(s) {
        return v;
    }
    unreachable!();
}

// Apply horizontal offset and swap axes to correspond to matrix indices
fn normalize_paths(paths: &mut Vec<Path>) -> MatIndex {
    let x_offset = paths.iter()
        .fold(usize::MAX, |min_x,p| min(min_x, p.iter().map(|(x,_)| *x).min().unwrap()))
        - LEFT_BUFFER;

    for p in paths {
        for (x,y) in p {
            *x = x.abs_diff(x_offset);
            (*x,*y) = (*y,*x);
        }
    }
    return (SOURCE_POSITION.1, SOURCE_POSITION.0 - x_offset);
}

fn domain_size(paths: &Vec<Path>) -> (usize,usize) {
    return paths.iter()
        .map(|p| p.iter())
        .flatten()
        .fold((0,0), |(ymax,xmax),&(y,x)| (max(ymax,y+LOWER_BUFFER), max(xmax,x+RIGHT_BUFFER)));
}

fn fill_rock_path(obstacle_matrix: &mut DMatrix<bool>, p: &Path) {
    for (&(u0,v0),&(u1,v1)) in p.iter().zip(p[1..].iter()) {
        let (y0,x0) = (min(u0,u1), min(v0,v1));
        let (y1,x1) = (max(u0,u1), max(v0,v1));
        for (y,x) in (y0..=y1).cartesian_product(x0..=x1) {
            obstacle_matrix[(y,x)] = true;
        }
    }
}

fn obstacle_matrix(paths: &Vec<Path>) -> DMatrix<bool> {
    let (ymax,xmax) = domain_size(paths);
    let mut mat = DMatrix::from_element(ymax+1, xmax+1, false);
    for p in paths {
        fill_rock_path(&mut mat, &p);
    }
    return mat;
}

fn next_sand_position(occupancy: &DMatrix<bool>, (y,x): MatIndex) -> MatIndex {
    return [(y+1,x), (y+1,x-1), (y+1,x+1), (y,x)].into_iter().find(|&idx| !occupancy[idx]).unwrap();
}

pub fn main() -> usize {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let mut paths = contents.lines().map(parse_path).collect_vec();
    let source_position = normalize_paths(&mut paths);

    let obstacle_matrix = obstacle_matrix(&paths);
    let domain_length = obstacle_matrix.shape().1;
    let infinite_depth = obstacle_matrix.shape().0 - LOWER_BUFFER;
    println!("Infinite Depth: {}", infinite_depth);
    // println!("{}", obstacle_matrix.map(|b| if b {"#"} else {"."}));
    let mut occupancy = obstacle_matrix.clone();

    let is_out = |(y,x)| (y >= infinite_depth) || (x < LEFT_BUFFER) || (x + RIGHT_BUFFER > domain_length);

    let mut active_sand_grain = source_position;
    let mut number_of_grains = 0;
    while !is_out(active_sand_grain) {
        active_sand_grain = match next_sand_position(&occupancy, active_sand_grain) {
            next_position if next_position == active_sand_grain => {
                occupancy[active_sand_grain] = true;
                number_of_grains += 1;
                source_position
            },
            next_position => next_position,
        };
    }
    // println!("{}", occupancy.zip_map(&obstacle_matrix, |is_occupied,is_rock| if is_rock {"#"} else if is_occupied {"o"} else {"."}));
    return number_of_grains;
}

pub fn main_bonus() -> usize {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let mut paths = contents.lines().map(parse_path).collect_vec();
    let max_depth = paths.iter().map(|p| p.iter()).flatten().map(|&(_,y)| y).max().unwrap();
    paths.push(vec![
        (SOURCE_POSITION.0 - max_depth - INFINITE_FLOOR_BUFFER, max_depth + 2), 
        (SOURCE_POSITION.0 + max_depth + INFINITE_FLOOR_BUFFER, max_depth + 2)
    ]);
    let source_position = normalize_paths(&mut paths);

    let obstacle_matrix = obstacle_matrix(&paths);
    // println!("{}", obstacle_matrix.map(|b| if b {"#"} else {"."}));
    let mut occupancy = obstacle_matrix.clone();

    let mut active_sand_grain = source_position;
    let mut number_of_grains = 0;
    while !occupancy[source_position] {
        active_sand_grain = match next_sand_position(&occupancy, active_sand_grain) {
            next_position if next_position == active_sand_grain => {
                occupancy[active_sand_grain] = true;
                number_of_grains += 1;
                source_position
            },
            next_position => next_position,
        };
    }
    // println!("{}", occupancy.zip_map(&obstacle_matrix, |is_occupied,is_rock| if is_rock {"#"} else if is_occupied {"o"} else {"."}));
    return number_of_grains;
}
