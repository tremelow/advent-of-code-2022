# Advent of Code 2022

Here are my codes for [AoC22](https://adventofcode.com/2022/). I took the opportunity to learn Rust, which is a bit of a challenge for me since I mostly use Julia. But I got some experience in C++ last year when teaching it, so I felt somewhat confident going into this.

## Opinions on modules & tools used

### ChatGPT

At some points, I used ChatGPT to help me with some code. Unfortunately, I never really managed to get it to give me "good" (read, compilable and correct) code, so I mostly used it to help me understand some concepts every once in a while.

### ndarray

For day 8, I used the library [`ndarray`](https://docs.rs/ndarray/latest/ndarray/), which is a library for n-dimensional arrays. I honestly found the library a bit confusing, although the drawings in the documentation where nice. What I particularly disliked was not having to specify `mut ArrayViewMut` in function arguments and still being able to modify the contents of the array, which seems pretty antithetic to Rust. However, I also think that my use case was unintended, and that it is normal if I struggled.

### nalgebra

The next day, I used [`nalgebra`](https://www.nalgebra.org/) to implement matrices. I particularly liked the in-place functionalities (such as `apply` and all sorts of `assign`) which reminded me of the best aspects of Julia. The matrix initialisation was a bit rough for my use case, but it was alright. The autocomplete suggests a lot of undocumented fluff which makes finding what you want more difficult. Some LAPACK routines are also implemented, which is nice but for someone who doesn't know their names, it is difficult to sift through them. Overall, a pretty nice module, in my opinion. 

### petgraph

For day 12, I used [`petgraph`](https://docs.rs/petgraph/latest/petgraph/). It worked pretty well, although I had trouble understanding the `Index` part for initialization. Once I realised the default was to re-index the nodes, and that `GraphMap` should be used in my case, it was pretty straightforward. Small gripe: the `dijkstra` algorithm does not return the shortest path, only the distances.

### nom

For day 13 I used [`nom`](https://github.com/Geal/nom). Overall, I think it is nice, but the learning curve is really steep and I really struggled with it. Hopefully I can actually learn to use it in the future. My first search results landed me on the Github page, the documentation, and some irrelevant tutorials for my use case, so I only used the "official" stuff, which seemed okay. This 

First of all, there is no method for nested environments (remember, the inputs were e.g. `[[1],[2,3,4]]`), so I had to use [the answer to this StackOverflow question](https://stackoverflow.com/questions/70630556/parse-allowing-nested-parentheses-in-nom). 

The most notable frustration, though, is the error system for which so the compiler does a lot of heavy lifting with infering types. So when it stops doing so it is *hard*. And I know [I'm not](https://www.reddit.com/r/rust/comments/mtmufz/struggling_with_noms_error_handling/) [the only](https://users.rust-lang.org/t/nom-how-to-raise-an-error-convert-other-errors-to-nom-errors/24701) [one](https://stackoverflow.com/questions/49395281/what-is-the-right-way-to-return-a-simple-custom-error-with-nom-errorkind). I think my gripes are well summarized with the following example, [straight from the documentation](https://docs.rs/nom/7.1.1/nom/combinator/fn.map.html).

```rust
use nom::{Err,error::ErrorKind, IResult,Parser};
use nom::character::complete::digit1;
use nom::combinator::map;

// `digit1` recognizes numbers
let mut parser = map(digit1, |s: &str| s.len());

// the parser will count how many characters were returned by digit1
assert_eq!(parser.parse("123456"), Ok(("", 6)));

// this will fail if digit1 fails
assert_eq!(parser.parse("abc"), Err(Err::Error(("abc", ErrorKind::Digit))));
```

This works, but commenting out the last line breaks the error type inference and the code stops compiling. It really took me a while to understand this problem. 

By the time I figured out my actual problem and found other tutorials, I was too drained to clean up my code and *actually* use `nom`. However, should you want to learn it 
- [Steve Donovan's Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/nom-intro.html) contains a chapter on Nom, although it might be a bit dated by now (the tutorial uses the macro syntax abandoned since `nom 5.0`).
- [Evan Kaledron's blog post](https://eyalkalderon.com/blog/nom-error-recovery/) giving the example of a simple arithmetic language, which resembles this use case. He also seems to circumvent the issue of nested environments, which would be nice.
