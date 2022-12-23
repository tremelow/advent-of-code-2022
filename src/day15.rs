use std::fs;
const INPUT_FILE: &str = "data/test15.txt";

const Y_LINE: i32 = 10;

use std::{
    ops::RangeInclusive, 
    cmp::{max, min}
};
use nom::bytes::complete::{take_while, take_till1};
use nom::combinator::rest;
use nom::multi::{count, many_till};
use nom::sequence::{preceded, separated_pair};
use nom::{self, 
    IResult,
    character::complete::digit1,
    combinator::{map, map_res, not},
    bytes::complete::{tag, take},
};

fn str_to_i32(input: &str) -> IResult<&str, i32> {
    if input.chars().next() == Some('-') {
        return map(|s| str_to_i32(s), |x| -x)(&input[1..]);
    }
    return map_res(digit1, str::parse::<i32>)(input);
}


type Interval = RangeInclusive<i32>;

fn merge(i1: &Interval, i2: &Interval) -> Option<Interval> {
    if i1.contains(i2.start()) || i2.contains(i1.start()) {
        Some(min(*i1.start(), *i2.start())..=max(*i1.end(), *i2.end()))//Some(min(*i1.start(),*i2.start())..=max(*i1.end(), *i2.end()))
    } else {
        None
    }
}

#[derive(Debug)]
pub enum DisjointUnion {
    Empty,
    Elem(Interval, Box<DisjointUnion>),
}

impl DisjointUnion {
    fn new() -> Self {
        DisjointUnion::Empty
    }

    fn from(i: Interval) -> Self {
        Self::Elem(i, Box::new(Self::Empty))
    }

    fn take(&mut self) -> Self{
        std::mem::replace(self, Self::Empty)
    }

    pub fn concatenation(self, val: Interval) -> Self {
        Self::Elem(val, Box::new(self))
    }

    /// Replace self with val :: self
    pub fn concatenate(&mut self, val: Interval) {
        // Take out the value of self and front-concatenate it with val
        *self = self.take().concatenation(val);
    }

    fn push_non_empty_interval(&mut self, i: Interval) {
        *self = match self.take() {
            Self::Empty => Self::from(i),
            Self::Elem(i2, mut next) => {
                if let Some(u) = merge(&i, &i2) {
                    next.as_mut().push(u);
                    *next
                } else {
                    next.as_mut().push(i);
                    Self::Elem(i2, next)
                }
            }
        };
    }

    fn push(&mut self, i: Interval) {
        if !i.is_empty() {
            self.push_non_empty_interval(i);
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn x_coordinate(input: &str) -> IResult<&str, i32> {
    preceded(tag("x="), str_to_i32)(input)
}
fn y_coordinate(input: &str) -> IResult<&str, i32> {
    preceded(tag("y="), str_to_i32)(input)
}
fn xy_coordinate(input: &str) -> IResult<&str, Position> {
    map(separated_pair(x_coordinate, tag(", "), y_coordinate), |(x,y)| Position {x,y})(input)
}
fn first_position(input: &str) -> IResult<&str, Position> {
    map(many_till(take(1usize), xy_coordinate), |(_,p)| p)(input)
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    beacon: Position,
}

impl Sensor {
    /// Create a new Sensor from a line of the form
    /// "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    fn new(data: &str) -> Self {
        if let Ok((_,v)) = count(first_position, 2)(data) {
            let position = v[0];
            let beacon = v[1];
            Sensor { position, beacon }
        } else {
            unreachable!()
        }
    }

    fn distance_to_beacon(&self) -> i32 {
        return (self.position.x - self.beacon.x).abs() + (self.position.y - self.beacon.y).abs();
    }

    fn range_on_yline(&self, y: i32) -> Interval {
        let distance_to_line = (y - self.position.y).abs();
        let range_s = self.distance_to_beacon();
        let dy = range_s - distance_to_line;
        return (self.position.y - dy)..=(self.position.y + dy);
    }
}



pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let mut no_beacon = DisjointUnion::new();
    for l in contents.lines() {
        let s = Sensor::new(l);
        println!("{:?}", s);
        no_beacon.push(s.range_on_yline(Y_LINE));
    }
    println!("{:#?}", no_beacon);

    return 0;
}

pub fn main_bonus() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}
