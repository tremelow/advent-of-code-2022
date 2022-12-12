use std::fs;
const INPUT_FILE: &str = "data/input10.txt";

const LENGTH_DISPLAY: i32 = 40;

fn get_increments(instructions: &str) -> Vec<i32> {
    let mut add = Vec::new();
    for inst in instructions.trim().lines() {
        add.push(0);
        if inst[..4].eq("addx") {
            add.push(inst.split(" ").last().unwrap().parse().unwrap());
        }
    }
    return add;
}

pub fn main() -> i32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let increments = get_increments(&contents);
    let mut sum_of_signal_strengths = 0;
    let mut x = 1;
    for (i,&n) in increments.iter().enumerate() {
        // Monitor the cycle
        if (i as i32 + 1) % LENGTH_DISPLAY == 20 {
            let signal_strength: i32 = ((i+1) as i32) * x;
            sum_of_signal_strengths += signal_strength;
        }
        x += n;
    }
    // println!("{x}");
    
    return sum_of_signal_strengths;
}

pub fn main_bonus() -> String {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let increments = get_increments(&contents);

    let mut crt = vec!["\n"];
    let mut x: i32 = 1;
    for (i,&n) in increments.iter().enumerate() {
        let pixel_position = (i as i32) % LENGTH_DISPLAY;
        if x.abs_diff(pixel_position) <= 1 {
            crt.push("⠿");
        } else {
            crt.push(" ");
        }
        if pixel_position + 1 == LENGTH_DISPLAY { crt.push("\n"); }
        x += n;
    }

    return String::from(crt.join(""));
}