use std::fs;
const INPUT_FILE: &str = "data/input13.txt";

use itertools::Itertools;
use nom::{
    Err, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::recognize,
    error::{Error, ErrorKind, ParseError}, 
    multi::separated_list0,
    sequence::delimited,
};
use std::cmp::{self,Ordering};

#[derive(Debug, PartialEq, Eq)]
enum NestedList<T: Copy> {
    Elem(T),
    List(Vec<NestedList<T>>),
}

impl<T> cmp::PartialOrd for NestedList<T> 
    where T: PartialOrd + Copy 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Elem(n), Self::Elem(m)) => n.partial_cmp(m),
            (Self::List(u), Self::List(v)) => u.partial_cmp(v),
            (Self::Elem(n), Self::List(v)) => vec![Self::Elem(*n)].partial_cmp(v),
            (Self::List(u), Self::Elem(m)) => u.partial_cmp(&vec![Self::Elem(*m)]),
        }
    }
}

impl<T> cmp::Ord for NestedList<T> 
    where T: Ord + Copy 
{
    fn cmp(&self, other: &Self) -> Ordering {
        println!("using custom comparator");
        match (self, other) {
            (Self::Elem(n), Self::Elem(m)) => n.cmp(m),
            (Self::List(u), Self::List(v)) => u.cmp(v),
            (Self::Elem(n), Self::List(v)) => vec![Self::Elem(*n)].cmp(v),
            (Self::List(u), Self::Elem(m)) => u.cmp(&vec![Self::Elem(*m)]),
        }
    }
}

/// taken from https://github.com/Geal/nom/issues/1253
fn take_until_unbalanced(
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


/// Transforms s string list into a nested list, e.g.
/// ```rust
/// assert_eq!(convert_to_list("[1,2,3]"), List([Elem(1), Elem(2), Elem(3)]));
/// assert_eq!(convert_to_list("[[1],[2]]"), List([List([Elem(1)]), List([Elem(2)])]));
/// assert_eq!(convert_to_list("[[]]"), List([List([])]));
/// ```
/// Could probably be done using a single nom query, but uh... nom is hard.
fn convert_to_list(s: &str) -> NestedList<i32> {
    if let Ok(n) = s.parse() {
        return NestedList::Elem(n);
    }

    let is_list = delimited(tag("["), take_until_unbalanced('[', ']'), tag("]"));
    // can not borrow is_list, weirdly
    let recognize_list = recognize(delimited(tag("["), take_until_unbalanced('[', ']'), tag("]")));
    let mut separate_list_contents = is_list.and_then(separated_list0(tag(","), alt((digit1, recognize(recognize_list)))));
    // else 
    if let Ok(("", v)) = separate_list_contents.parse(s) {
        return NestedList::List(v.into_iter().map(convert_to_list).collect_vec());
    }

    unreachable!();
}

pub fn main() -> usize {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
    let contents = contents.trim().split("\n\n");

    let mut lhs = Vec::new();
    let mut rhs = Vec::new();

    for to_compare in contents.clone() {
        let mut lists = to_compare.lines();
        lhs.push(convert_to_list(lists.next().unwrap()));
        rhs.push(convert_to_list(lists.next().unwrap()));
    }

    let res = lhs.iter()
        .zip(rhs.iter())
        .enumerate()
        .fold(0, |acc, (i, (l,r))| if l <= r {acc+i+1} else {acc});

    return res;
}

pub fn main_bonus() -> usize {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let mut lists = contents.split("\n\n")
        .map(|s| s.lines()).flatten()
        .map(convert_to_list)
        .collect_vec();

    let div_packet1 = convert_to_list("[[2]]");
    let div_packet2 = convert_to_list("[[6]]");
    lists.append(&mut vec![div_packet1, div_packet2]);
    lists.sort();
    
    
    let div_packet1 = convert_to_list("[[2]]");
    let div_packet2 = convert_to_list("[[6]]");
    let decoder_key = lists.iter().enumerate()
        .fold(1, |acc, (i,l)| if *l == div_packet1 || *l == div_packet2 {acc * (i+1)} else {acc});

    return decoder_key;
}
