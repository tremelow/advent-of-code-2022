# Advent of Code 2022 - Day 18

## Problem

You and the elephants finally reach fresh air. You've emerged near the base of a large volcano that seems to be actively erupting! Fortunately, the lava seems to be flowing away from you and toward the ocean.

Bits of lava are still being ejected toward you, so you're sheltering in the cavern exit a little longer. Outside the cave, you can see the lava landing in a pond and hear it loudly hissing as it solidifies.

Depending on the specific compounds in the lava and speed at which it cools, it might be forming obsidian! The cooling rate should be based on the surface area of the lava droplets, so you take a quick scan of a droplet as it flies past you (your puzzle input).

Because of how quickly the lava is moving, the scan isn't very good; its resolution is quite low and, as a result, it approximates the shape of the lava droplet with **1x1x1 cubes on a 3D grid**, each given as its `x,y,z` position.

To approximate the surface area, count the number of sides of each cube that are not immediately connected to another cube. So, if your scan were only two adjacent cubes like `1,1,1` and `2,1,1`, each cube would have a single side covered and five sides exposed, a total surface area of 10 sides.

Here's a larger example:

```txt
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
```

## Part One

In the above example, after counting up all the sides that aren't connected to another cube, the total surface area is `64`.

*What is the surface area of your scanned lava droplet?*

### Modeling the cubes

A cube has a position and 6 sides. At position `(x,y,z)`, its sides are characterised by their contact with `(x±1,y,z)`, `(x,y±1,z)` or `(x,y,z±1)`. Let's represent positions using tuples and keep track of which faces of the cube are visible using a `HashMap` mapping the neighbouring position to the visibility status.

```rust
use std::collections::HashMap

type Position = (i32,i32,i32);

impl Cube {
    pos: Position,
    face_is_visible: HashMap<Position,bool>,
}
```

Initially, we consider every face visible and we'll loop on all the cubes to determine whether the face is blocked by a neighbour or not. Let's sort out the initialisation and the parsing.

```rust
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
        let xyz: Vec<i32> = xyz
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        Self::new(xyz[0], xyz[1], xyz[2])
    }
}
```

The initialisation of the data is done by simply mapping `Cube::from` to every line of the input.

### Number of visible faces

The idea is to iterate on every couple of cubes and update the visibility status of the faces depending on whether the neighbour exists. Due to the borrow checks of `Rust`, iterating on pairs is a bit finnicky. One way I found[^1] that seems to work is using the following syntax:

[^1]: https://users.rust-lang.org/t/nested-iteration-within-mutable-iteration/14594/2

```rust
for i in 0..cubes.len() {
    let (_,  cright) = cubes.split_at_mut(i);
    let (c1, cright) = cright.split_first_mut().unwrap();
    for c2 in cright {
        // do stuff
    }
}
```

The first split returns a couple of arrays, the first representing `&cubes[..i]` and the second `&cubes[i..]`. The first is immediatly discarded, and the second is shadowed by separating `cubes[i]` from the rest.

---

```rust
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position { x: i32, y: i32, z: i32 }

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }
}
impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

const CARDINAL_DIRECTIONS: [Position; 6] = [
    Position { x:  1, y:  0, z: 0 },
    Position { x: -1, y:  0, z: 0 },
    Position { x:  0, y:  1, z: 0 },
    Position { x:  0, y: -1, z: 0 },
    Position { x:  0, y:  0, z:  1 },
    Position { x:  0, y:  0, z: -1 },
];

struct Cube {
    pos: Position,
    face_visible: [bool; 6],
}
```
