use std::fs;
const INPUT_FILE: &str = "data/input11.txt";

use std::collections::VecDeque;

#[allow(unused_imports)]
use itertools::Itertools;

#[derive(Debug)]
struct Item {
    worry_level: i32,
    monkey: usize,
}

#[derive(Debug)]

struct Operation {
    op: char,
    val: Option<i32>,
}

impl Operation {
    fn apply(&self, x: i32) -> i32 {
        let rhs = match self.val {
            Some(n) => n,
            _ => x,
        };
        return match self.op {
            '+' => {
                // println!("Adding {} to {}", x, rhs);
                x + rhs
            },
            '*' => {
                // println!("Multiplying {} by {}", x, rhs);
                x * rhs
            },
            _ => 0,
        };
    }
}

#[derive(Debug)]

struct DivTest {
    div_by: i32,
    if_true: usize,
    if_false: usize,
}

impl DivTest {
    fn apply(&self, x: i32) -> usize {
        if x%self.div_by == 0 { self.if_true } else {self.if_false }
    }
}

#[derive(Debug)]

struct Monkey {
    /// Points to a vector of all items
    items: VecDeque<usize>,
    /// The operation to apply, after having removed 
    op: Operation,
    test: DivTest,
}

impl Monkey {
    /// Take an argument of the form
    /// ```
    /// Monkey 0:
    ///   Starting items: 79, 98
    ///   Operation: new = old * 19
    ///   Test: divisible by 23
    ///     If true: throw to monkey 2
    ///     If false: throw to monkey 3
    /// ```
    /// Notably, all "operations" are of the form `new = old` followed by an
    /// operator.
    fn new(s: &str, item_pile: &mut Vec<Item>) -> Monkey {
        let mut lines = s.lines();
        // Monkey 0:
        let monkey_number: usize = lines.next().unwrap().trim()
            .strip_prefix("Monkey ").unwrap()
            .strip_suffix(":").unwrap()
            .parse().unwrap();
        // Starting items: 79, 98
        let item_values: Vec<i32> = lines.next().unwrap().trim()
            .strip_prefix("Starting items: ").unwrap()
            .split(", ").map(|i| i.parse().unwrap())
            .collect();
        let mut items: VecDeque<usize> = VecDeque::new();
        for item_val in item_values {
            items.push_back(item_pile.len());
            item_pile.push(Item { worry_level: item_val, monkey: monkey_number });
        }
        // Operation: new = old * 19
        let mut operation = lines.next().unwrap().trim()
            .strip_prefix("Operation: new = old ").unwrap()
            .split(" ");
        let op = operation.next().unwrap().chars().next().unwrap();
        let op_val = operation.next().unwrap().parse::<i32>().ok();
        let operation = Operation { op, val: op_val, };
        
        // Test: divisible by 23
        let div_by = lines.next().unwrap().trim()
            .split(" ").last().unwrap()
            .parse().unwrap();
        let if_true = lines.next().unwrap().trim()
            .split(" ").last().unwrap()
            .parse().unwrap();
        let if_false = lines.next().unwrap().trim()
            .split(" ").last().unwrap()
            .parse().unwrap();
        let div_test = DivTest {div_by, if_true, if_false };

        return Monkey {items, op: operation, test: div_test };
    }
}

fn perform_inspection(item: &mut Item, monkey: &mut Monkey) {
    item.worry_level = monkey.op.apply(item.worry_level);
    item.monkey = monkey.test.apply(item.worry_level);
}

pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let contents = contents.split("\n\n");
    let mut item_pile: Vec<Item> = Vec::new();
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_init_state in contents {
        monkeys.push(Monkey::new(monkey_init_state, &mut item_pile));
        // println!("{:?}", monkeys.last());
    }

    
    // Perform one round of inspection
    let number_of_monkeys = monkeys.len();
    let mut activity: Vec<u32> = vec![0; number_of_monkeys];
    for _ in 0..20 {

        // for (m_idx, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}: {}", m_idx, monkey.items.iter().map(|&i| item_pile[i].worry_level.to_string()).join(", "));
        // }
        // println!();

        for monkey_idx in 0..number_of_monkeys {
            activity[monkey_idx] += monkeys[monkey_idx].items.len() as u32;
            while let Some(item_idx) = monkeys[monkey_idx].items.pop_front() {
                // perform_inspection(&mut item_pile[item_idx], &mut monkeys[monkey_idx]);
                item_pile[item_idx].worry_level = monkeys[monkey_idx].op.apply(item_pile[item_idx].worry_level);
                item_pile[item_idx].worry_level /= 3;
                item_pile[item_idx].monkey = monkeys[monkey_idx].test.apply(item_pile[item_idx].worry_level);
                monkeys[item_pile[item_idx].monkey].items.push_back(item_idx);
            }
        }
    }

    // for (m_idx, monkey) in monkeys.iter().enumerate() {
    //     println!("Monkey {}: {}", m_idx, monkey.items.iter().map(|&i| item_pile[i].worry_level.to_string()).join(", "));
    // }


    let mut tmp_activity = activity.clone();
    tmp_activity.sort();
    tmp_activity.reverse();

    return tmp_activity[..2].into_iter().product();
}

pub fn main_bonus() -> u64 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let contents = contents.split("\n\n");
    let mut item_pile: Vec<Item> = Vec::new();
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_init_state in contents {
        monkeys.push(Monkey::new(monkey_init_state, &mut item_pile));
    }

    let number_of_rounds = 10000;

    let number_of_monkeys = monkeys.len();
    let moduli: Vec<i32> = monkeys.iter().map(|m| m.test.div_by).collect();
    let mut worry_levels: Vec<Vec<i32>> = item_pile.iter()
        .map(|i| 
            moduli.iter().map(|m| i.worry_level % m)
            .collect()
        ).collect();
    let mut monkey_of_item: Vec<usize> = item_pile.iter().map(|i| i.monkey).collect();

    let mut activity: Vec<u32> = vec![0; number_of_monkeys];

    for _ in 0..number_of_rounds {

        // for (m_idx, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}: {:?}", m_idx, worry_levels.iter().zip(monkey_of_item.iter()).filter(|(_,m)| **m == m_idx).map(|(wl,_)| wl).collect_vec());
        // }
        // println!();

        // Perform one round of inspection
        for monkey_idx in 0..number_of_monkeys {
            activity[monkey_idx] += monkeys[monkey_idx].items.len() as u32;
            while let Some(item_idx) = monkeys[monkey_idx].items.pop_front() {
                let wl_item = &mut worry_levels[item_idx];
                for (wl,m) in wl_item.iter_mut().zip(&moduli) {
                    *wl = monkeys[monkey_idx].op.apply(*wl) % *m;
                }
                let item_goes_to = monkeys[monkey_idx].test.apply(wl_item[monkey_idx]);
                monkey_of_item[item_idx] = item_goes_to;
                monkeys[item_goes_to].items.push_back(item_idx);
            }
        }
    }

    // for (m_idx, monkey) in monkeys.iter().enumerate() {
    //     println!("Monkey {}: {}", m_idx, monkey.items.iter().map(|&i| item_pile[i].worry_level.to_string()).join(", "));
    // }

    let mut tmp_activity = activity.iter().map(|x| *x as u32).collect_vec();
    tmp_activity.sort();
    tmp_activity.reverse();

    let x: u64 = (tmp_activity[0] as u64) * (tmp_activity[1] as u64);

    return x;
}