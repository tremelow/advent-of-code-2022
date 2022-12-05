use std::fs;

const INPUT_FILE: &str = "data/input05.txt";
type Crate = char;
type Stack = Vec<Crate>;
type Arrangement = Vec<Stack>;


fn do_instr(yard: &mut Arrangement, instr: &str) {
    let words: Vec<&str> = instr.split(" ").collect();
    let amount: usize = words[1].parse().expect("Could not find amount.");
    let from_idx: usize = words[3].parse().expect("Could not find 'from' stack.");
    let to_idx: usize = words[5].parse().expect("Could not find 'to' stack.");

    for _ in 0..amount {
        let content = yard[from_idx - 1].pop().expect("Stack was empty D:");
        yard[to_idx - 1].push(content);
    }
}

fn do_instr_9001(yard: &mut Arrangement, instr: &str) {
    let words: Vec<&str> = instr.split(" ").collect();
    let amount: usize = words[1].parse().expect("Could not find amount.");
    let from_idx: usize = words[3].parse().expect("Could not find 'from' stack.");
    let to_idx: usize = words[5].parse().expect("Could not find 'to' stack.");

    let mut tmp_stack = Vec::new();

    for _ in 0..amount {
        let content = yard[from_idx - 1].pop().expect("Stack was empty D:");
        tmp_stack.push(content);
    }
    for _ in 0..amount {
        let content = tmp_stack.pop().expect("Stack was empty D:");
        yard[to_idx - 1].push(content);
    }
}

fn initialize_yard(initial_yard: &str) -> Arrangement{
    let mut initial_yard = initial_yard.lines().rev();
    let stack_numbers = initial_yard.next().unwrap();

    let mut stack_indices = Vec::new();
    let mut yard = Arrangement::new();

    for (i,c) in stack_numbers.char_indices() {
        if c != ' ' {
            yard.push(Stack::new());
            stack_indices.push(i);
        }
    }

    for line in initial_yard {
        for (i, &idx_in_str) in (&stack_indices).into_iter().enumerate() {
            let char_to_add = line.chars().nth(idx_in_str).unwrap();
            if char_to_add != ' ' {
                yard[i].push(char_to_add);
            }
        }
    }

    return yard;
}


pub fn main() -> String {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let mut contents = contents.split("\n\n");
    let initial_yard = contents
        .next()
        .expect("Original arrangement not parsed.");
    
    let mut yard = initialize_yard(initial_yard);
    
    let instructions = contents.next().expect("Could not get instructions.").lines();
    for instr in instructions {
        do_instr(&mut yard, instr);
    }

    let res = yard.into_iter()
        .map(|y| y.into_iter().last().unwrap().to_string())
        .fold(String::new(), |a,b| a + &b);

    return res;
}

pub fn main_bonus() -> String {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let mut contents = contents.split("\n\n");
    let initial_yard = contents
        .next()
        .expect("Original arrangement not parsed.");
    
    let mut yard = initialize_yard(initial_yard);
    
    let instructions = contents.next().expect("Could not get instructions.").lines();
    for instr in instructions {
        do_instr_9001(&mut yard, instr);
    }

    let res = yard.into_iter()
        .map(|y| y.into_iter().last().unwrap().to_string())
        .fold(String::new(), |a,b| a + &b);

    return res;
}