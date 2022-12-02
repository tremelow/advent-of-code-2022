use std::fs;

#[derive(PartialEq)]
enum Shape{
    Rock, 
    Paper, 
    Scissors,
}

fn shape_from_char(s: &char) -> Result<Shape,&str> {
    match s {
        'A' | 'X' => Ok(Shape::Rock),
        'B' | 'Y' => Ok(Shape::Paper),
        'C' | 'Z' => Ok(Shape::Scissors),
        _ => Err("Shape unknown!")
    }
}

fn win_score(opponent: &Shape, player: &Shape) -> usize {
    match (opponent, player) {
        (Shape::Rock,     Shape::Paper) => 6,
        (Shape::Paper,    Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Rock) => 6,
        // (so, sp) => 3,
        (so,sp) => if so == sp {3} else {0}
    }
}

fn shape_score(s: &Shape) -> usize {
    match s {
        Shape::Rock     => 1,
        Shape::Paper    => 2,
        Shape::Scissors => 3,
    }
}

fn parse_round(line: &str) -> (Shape, Shape) {
    let opponent = shape_from_char(&line.chars().nth(0).unwrap()).unwrap();
    let player = shape_from_char(&line.chars().nth(2).unwrap()).unwrap();
    return (opponent, player);
}

pub fn main() -> usize {
    let contents = fs::read_to_string("data/input02.txt").expect("Should have been able to read the file.");
    let mut total_score = 0;
    for line in contents.lines() {
        let (opponent, player) = parse_round(&line);
        total_score += win_score(&opponent, &player);
        total_score += shape_score(&player);
    }
    return total_score;
}


fn loses_against(opponent: &Shape) -> Shape {
    match opponent {
        Shape::Rock     => Shape::Paper,
        Shape::Paper    => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn draws_against(opponent: &Shape) -> Shape {
    match opponent {
        Shape::Rock     => Shape::Rock,
        Shape::Paper    => Shape::Paper,
        Shape::Scissors => Shape::Scissors,
    }
}

fn wins_against(opponent: &Shape) -> Shape {
    match opponent {
        Shape::Rock     => Shape::Scissors,
        Shape::Paper    => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

pub fn main_bonus() -> usize {
    let contents = fs::read_to_string("data/input02.txt").expect("Should have been able to read the file.");
    let mut total_score = 0;
    for line in contents.lines() {
        let opponent = shape_from_char(&line.chars().nth(0).unwrap()).unwrap();

        let strategy = match line.chars().nth(2).unwrap() {
            'X' => wins_against,  // so we lose
            'Y' => draws_against, // so we draw
            'Z' => loses_against,  // so we win
            _   => loses_against
        };
        let player = strategy(&opponent);

        total_score += win_score(&opponent, &player);
        total_score += shape_score(&player);
    }
    return total_score;
}