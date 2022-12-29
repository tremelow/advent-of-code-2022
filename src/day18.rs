use std::{fs, time};
const INPUT_FILE: &str = "data/input18.txt";

// use std::ops::Add;
use std::collections::HashMap;

// use itertools::Itertools;

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
// struct Position { x: i32, y: i32, z: i32 }

// impl Position {
//     fn new(x: i32, y: i32, z: i32) -> Self {
//         Position { x, y, z }
//     }
// }
// impl Add for Position {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Position::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
//     }
// }

// const CARDINAL_DIRECTIONS: [Position; 6] = [
//     Position { x:  1, y:  0, z: 0 },
//     Position { x: -1, y:  0, z: 0 },
//     Position { x:  0, y:  1, z: 0 },
//     Position { x:  0, y: -1, z: 0 },
//     Position { x:  0, y:  0, z:  1 },
//     Position { x:  0, y:  0, z: -1 },
// ];

type Position = (i32,i32,i32);

#[derive(Debug)]
struct Cube {
    pos: Position,
    face_visible: HashMap<Position,bool>,
}
impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        let pos = (x,y,z);
        let face_visible = HashMap::from([
            ((x+1,y,z), true),
            ((x-1,y,z), true),
            ((x,y+1,z), true),
            ((x,y-1,z), true),
            ((x,y,z+1), true),
            ((x,y,z-1), true),
        ]);
        Cube { pos, face_visible }
    }
    fn from(xyz: &str) -> Self {
        let xyz: Vec<i32> = xyz.split(",").map(|s| s.parse().unwrap()).collect();
        Self::new(xyz[0], xyz[1], xyz[2])
    }
}

// fn pairs<'a, T>(v: &'a mut Vec<T>) -> MutVecPairs<T> {
//     MutVecPairs { v, at: (0,1) }
// }
// struct MutVecPairs<'a, T> {
//     v: &'a mut Vec<T>,
//     at: (usize,usize),
// }
// impl<'a, T> Iterator for MutVecPairs<'a, T> {
//     type Item = (&'a mut T, &'a mut T);
//     fn next(&mut self) -> Option<Self::Item> {
//         let (mut i, mut j) = self.at;
//         if i+j == self.v.len() {
//             j = 0;
//             i += 1;
//         }
//         let (_, cright) = self.v.split_at_mut(i);
//         if let Some((c1,cright)) = cright.split_first_mut() {
//             let c2: &'a mut T = cright.get_mut(j).unwrap();
//             Some((c1, c2))
//         } else {
//             None
//         }
//     }
// }

pub fn main() -> usize {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    // let start = time::Instant::now();

    let mut cubes: Vec<Cube> = contents.lines().map(Cube::from).collect();
    for i in 0..cubes.len() {
        let (c1,cright) = cubes.split_at_mut(i).1.split_first_mut().unwrap();
        // let neighbours: Vec<Position> = c1.face_visible.iter()
        //     .filter(|&(_,v)| *v)
        //     .map(|(&k,_)| k)
        //     .collect();
        let neighbours: Vec<Position> = c1.face_visible.keys().cloned().collect();
        for c2 in cright.iter_mut().filter(|c| neighbours.contains(&c.pos)) {
            c1.face_visible.get_mut(&c2.pos).map(|v| *v = false);
            c2.face_visible.get_mut(&c1.pos).map(|v| *v = false);
            // if let Some(v) = c1.face_visible.get_mut(&c2.pos) {
            //     *v = false;
            // }
            // if let Some(v) = c2.face_visible.get_mut(&c1.pos) {
            //     *v = false;
            // }
        }
    }
    // println!("Took {} ms.", start.elapsed().as_millis());
    return cubes.iter().map(|c| c.face_visible.iter().filter(|(_,v)| **v).count()).sum();
}

pub fn main_bonus() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let x: [i32;3] = [1,2,3];
    let y = x.map(|u| u+1);
}
