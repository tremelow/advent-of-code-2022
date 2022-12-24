use std::fs;
const INPUT_FILE: &str = "data/input15.txt";

const Y_LINE: i32 = 2000000;

use std::{
    ops::RangeInclusive, 
    cmp::{max, min}
};
use itertools::Itertools;
use nom::multi::{count, many_till};
use nom::sequence::{preceded, separated_pair};
use nom::{self, 
    IResult,
    character::complete::digit1,
    combinator::{map, map_res},
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
    if i1.is_empty() || i2.is_empty() {
        None
    } else if i1.contains(i2.start()) || i2.contains(i1.start()) {
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

    fn iter(&self) -> Iter {
        Iter { at: Some(self) }
    }
}

struct Iter<'a> {
    at: Option<&'a DisjointUnion>
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Interval;
    fn next(&mut self) -> Option<Self::Item> {
        match self.at.take() {
            None => None,
            Some(DisjointUnion::Empty) => None,
            Some(DisjointUnion::Elem(i, next)) => {
                self.at = Some(next.as_ref());
                Some(i)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
        let dx = range_s - distance_to_line;
        return (self.position.x - dx)..=(self.position.x + dx);
    }
}



pub fn main() -> i32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let sensors: Vec<Sensor> = contents.lines().map(|l| Sensor::new(l)).collect();
    let ranges_on_line = sensors.iter()
        .map(|s| s.range_on_yline(Y_LINE))
        .fold(DisjointUnion::new(), 
            |mut u, i| {u.push(i); u});
    let size_visible_on_line: i32 = ranges_on_line.iter()
        .filter(|i| !i.is_empty())
        .map(|i| i.end() - i.start() + 1)
        .sum();
    let beacons_on_line = sensors.iter()
        .map(|s| s.beacon)
        .filter(|b| b.y == Y_LINE && ranges_on_line.iter().any(|i| i.contains(&b.x)))
        .unique()
        .count() as i32;

    let res = size_visible_on_line - beacons_on_line;

    return res;
}

pub fn main_bonus() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}
