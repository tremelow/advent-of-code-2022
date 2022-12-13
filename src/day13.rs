use std::fs;
const INPUT_FILE: &str = "data/test13.txt";

use itertools::Itertools;
use nom::{
    Err, IResult,
    branch::alt,
    bytes::complete::tag,
    sequence::{delimited, terminated},
    multi::{fold_many0, many0, separated_list0, separated_list1},
    character::complete::digit0,
    error::{Error, ErrorKind, ParseError},
};

#[derive(Debug)]
enum List {
    Number(i32),
    NestedList(Vec<List>),
}

pub fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
}

fn convert_to_list(s: &str) -> Option<List> {
    let mut is_list = delimited(tag("["), take_until_unbalanced('[', ']'), tag("]"));
    let mut parser = separated_list0(tag(","), alt((&mut is_list, digit0)));
    // [1,1,3,1,1]
    if s.is_empty() {
        return None;
    }
    if let Ok(n) = s.parse() {
        return Some(List::Number(n));
    }
    // else 
    if let Ok(("", v)) = parser(s) {
        return Some(List::NestedList(v.into_iter().map(convert_to_list).filter(|x| x.is_some()).map(|x| x.unwrap()).collect_vec()));
    }
    return None;
}

fn recognize_integer(input: &str) -> IResult<&str, &str> {
    digit0(input)
}

pub fn main() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
    let contents = contents.trim().split("\n\n");

    let mut str_lhs = Vec::new();
    let mut str_rhs = Vec::new();
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();

    for to_compare in contents.clone() {
        let mut lists = to_compare.lines();
        let lhs_list = lists.next().unwrap();
        let rhs_list = lists.next().unwrap();
        str_lhs.push(lhs_list);
        lhs.push(convert_to_list(lhs_list));
        str_rhs.push(lhs_list);
        rhs.push(convert_to_list(rhs_list));
    }

    for (v,s) in lhs.into_iter().map(|x| x.unwrap()).zip(str_lhs) {
        println!("{:?}", s);
        println!("{:?}", v);
        println!();
    }

    let is_list = take_until_unbalanced('[', ']');
    println!("{:?}", is_list("1"));
    println!("{:?}", is_list("[1,2]"));
    println!("{:?}", is_list("[1,2,[3,4],5]"));

    // println!("{:?}", rhs);

    let mut is_list = delimited(tag("["), take_until_unbalanced('[', ']'), tag("]"));
    println!("{:?}", is_list("[1,2,[3,4],5]")); // Ok(("","1,2,[3,4],5"))
    println!("{:?}", is_list("1,2,[3,4],5")); // Err
    println!();
    
    let mut parser = separated_list0(tag(","), alt((&mut is_list, digit0)));
    println!("{:?}", parser("[1,2,[3,[4,5]],6]")); // Ok(("", ["1,2,[3,[4,5]],6"]))
    println!("{:?}", parser("1,2,[3,[4,5]],6")); // Ok(("", ["1", "2", "3,[4,5]", "6"]))
    println!("{:?}", parser("1")); // Ok(("", ["1"]))
    println!("{:?}", parser("[[]]")); // Ok(("", ["[]"]))
    println!("{:?}", parser("[]")); // Ok(("", [""]))
    println!();
    drop(parser);

    // let mut hmm = alt((is_list, terminated(digit0, tag(",")), digit0));
    // println!("{:?}", hmm("1,2,[3,4]"));
    // println!("{:?}", hmm("[3,4],5"));
    // println!();
}

pub fn main_bonus() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
}
