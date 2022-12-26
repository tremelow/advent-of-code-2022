use std::fs;
use std::cmp::max;

pub fn main01_1() -> i32 {
    let contents = fs::read_to_string("data/01.txt").expect("Should have been able to read the file.");

    let max_total = contents
        .split("\n\n")
        .map(|s| 
            s.lines()
            .map(|x| x.parse::<i32>().unwrap())
            .sum()
        )
        .max().unwrap();

    // let mut loc_total = 0;
    // let mut max_total = 0;
    // for line in contents.lines() {
    //     let loc: i32 = line.parse().unwrap_or(0);
    //     loc_total += loc;
    //     if loc == 0 {
    //         max_total = max(max_total, loc_total);
    //         loc_total = 0;
    //     }
    // }
    return max_total;
}


fn insert01_2(max_total : &mut [i64; 3], loc_total : i64) {
    if loc_total > max_total[2] {
        if loc_total > max_total[1] {
            if loc_total > max_total[0] {
                max_total[2] = max_total[1];
                max_total[1] = max_total[0];
                max_total[0] = max(max_total[0], loc_total);
            }
            else {
                max_total[2] = max_total[1];
                max_total[1] = max(max_total[1], loc_total);
            }
        }
        else {
            max_total[2] = max(max_total[2], loc_total);
        }
    }
}

pub fn main01_2() -> i64 {
    let contents = fs::read_to_string("data/01.txt").expect("Should have been able to read the file.");
    let mut loc_total = 0;
    let mut max_total = [0, 0, 0];
    for line in contents.lines() {
        let loc : i64 = line.parse().unwrap_or(0);
        loc_total += loc;
        if loc == 0 {
            insert01_2(&mut max_total, loc_total);
            loc_total = 0;
        }
    }
    insert01_2(&mut max_total, loc_total);
    return max_total.into_iter().sum();
}
