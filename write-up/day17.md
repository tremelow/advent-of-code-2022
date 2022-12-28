# Advent of Code 2022 - Day 17


## Problem

### Lore

Your handheld device has located an alternative exit from the cave for you and the elephants. The ground is rumbling almost continuously now, but the strange valves bought you some time. It's definitely getting warmer in here, though.

The tunnels eventually open into a very tall, narrow chamber. Large, oddly-shaped rocks are falling into the chamber from above, presumably due to all the rumbling. If you can't work out where the rocks will fall next, you might be crushed!

### Notation

The five types of rocks have the following peculiar shapes, where `#` is rock and `.` is empty space:

```
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
```

The rocks fall in the order shown above: first the `-` shape, then the `+` shape, and so on. **Once the end of the list is reached, the same order repeats**: the `-` shape falls first, sixth, 11th, 16th, etc.

The rocks don't spin, but they do get pushed around by jets of hot gas coming out of the walls themselves. A quick scan reveals the effect the jets of hot gas will have on the rocks as they fall (your puzzle input).

For example, suppose this was the jet pattern in your cave:

```
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
```

In jet patterns, `<` means a push to the left, while `>` means a push to the right. The pattern above means that the jets will push a falling rock right, then right, then right, then left, then left, then right, and so on. **If the end of the list is reached, it repeats.**

### Simulation details

The tall, vertical chamber is exactly **seven units wide**. Each rock appears so that its left edge is **two units away from the left wall** and its bottom edge is **three units above the highest rock** in the room (or the floor, if there isn't one).

After a rock appears, it alternates between being pushed by a jet of hot gas one unit (in the direction indicated by the next symbol in the jet pattern) and then falling one unit down. If any movement would cause any part of the rock to move into the walls, floor, or a stopped rock, the movement instead does not occur. If a downward movement would have caused a falling rock to move into the floor or an already-fallen rock, the falling rock stops where it is (having landed on something) and a new rock immediately begins falling.

## Part One

To prove to the elephants your simulation is accurate, they want to know how tall the tower will get after 2022 rocks have stopped (but before the 2023rd rock begins falling). 

<!-- In this example, the tower of rocks will be 3068 units tall. -->

How many units tall will the tower of rocks be after 2022 rocks have stopped falling?

### Modelling shapes, movement and the rock formation

**For the shapes**, I wanted to try something new for arrays of booleans, which is to use bit operations. Here, a row is 7 bits wide, therefore my first instinct was to use `u8`s. However, since every shape fits in a 4-by-7 box, I have decided to use `u32`s to describe a such a box. The shapes therefore become 
```
0x               0x               0x               0x               0x
00 .|.......|    00 .|.......|    00 .|.......|    10 .|..#....|    00 .|.......|
00 .|.......|    08 .|...#...|    04 .|....#..|    10 .|..#....|    00 .|.......|
00 .|.......|    1c .|..###..|    04 .|....#..|    10 .|..#....|    18 .|..##...|
1e .|..####.|    08 .|...#...|    1c .|..###..|    10 .|..#....|    18 .|..##...|
```
The pipes `|` do not appear in the data, but it may be useful to imagine them—they represent the walls of the chamber.[^1] The occupancy (`#` or `.`) is represented by the bits of the data (`1` or `0`). The hex code of each shape is represented on the left. I positioned each shape such that it corresponds to its spawn point, 2 units away from the left wall.

[^1]: As such, the first bit of each column is useless, since a shape should not be able to escape the 7-unit chamber, but I think the convenience of bit operations outweighs this loss of efficiency.

To check whether a shape is in contact with the left or right wall, we use masks:
```
0x             0x
40 .#......    01 .#......
40 .#......    01 .#......
40 .#......    01 .#......
40 .#......    01 .#......
```
This is implemented as
```rust
const SHAPES: [u32;5] = [0x0000001e, 0x00081c08, 0x0004041c, 0x10101010, 0x00001818];
const LEFT_WALL:  u32 = 0x40404040;
const RIGHT_WALL: u32 = 0x01010101;
```

**The movement** is very straightforward: the shape moves either left or right. This is what `enum`s are for.
```rust
#[derive(Clone, Copy, Debug)]
enum LeftOrRight {
    Left,
    Right,
}
impl LeftOrRight {
    fn from(c: char) -> Self {
        match c {
            '<' => LeftOrRight::Left,
            '>' => LeftOrRight::Right,
            _   => panic!(),
        }
    }
    fn opposite(self) -> Self {
        match self {
            LeftOrRight::Left  => LeftOrRight::Right,
            LeftOrRight::Right => LeftOrRight::Left,
        }
    }
}
```
Of course one could decide to use ±1 or some other type-dependant approach, but I actually appreciate the rigidity of this basic implementation.

**The rock formation** is just a vector of rows. The rows could technically be `u8`s, but if I want to easily create a 4-by-7 window and lower it by 1 easily, I should stick with `u32`s. 
```rust
const ROW_MASK: u32 = 0b1111111;
fn lower_window(window: u32, new_row: u32) -> u32 {
    (window << 8) | (new_row & ROW_MASK)
}
let mut rock_formation: Vec<u32> = Vec::new();
```


### Displacing a shape

The advantage of this system is that contact and left and right movements are very simple: they are bit shifts. Assuming that `obstacle: u32` represents the rock formation in the 4-by-7 window of the shape, we may move the shape in a given direction after checking whether the obstacle or a wall hinders movement.

```rust
#[derive(Clone, Copy, Debug)]
struct Shape(u32);

impl Shape {
    fn touches(&self, obstacle: u32) -> bool {
        self.0 & obstacle != 0
    }
    fn move_lr(&mut self, dir: LeftOrRight, obstacle: u32) {
        match dir {
            LeftOrRight::Left  => self.move_left(obstacle),
            LeftOrRight::Right => self.move_right(obstacle),
        }
    }
    fn move_left(&mut self, obstacle: u32) {
        if !self.touches(LEFT_WALL | (obstacle >> 1)) { 
            self.0 <<= 1; 
        }
    }
    fn move_right(&mut self, obstacle: u32) {
        if !self.touches(RIGHT_WALL | (obstacle << 1)) { 
            self.0 <<= 1; 
        }
    }
}
```


## Part Two

The elephants are not impressed by your simulation. They demand to know how tall the tower will be after `1_000_000_000_000` rocks have stopped! Only then will they feel confident enough to proceed through the cave.

<!-- In the example above, the tower would be `1514285714288` units tall! -->

How tall will the tower be after `1_000_000_000_000` rocks have stopped?

### Solving memory problems

Of course after one *trillion* iterations, the tower will be of size at least one *trillion*, and my computer does not have enough memory for all that.