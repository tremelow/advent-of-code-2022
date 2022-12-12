use std::ops::Index;
use std::{fs, hash::Hash};
const INPUT_FILE: &str = "data/test12.txt";

// use std::collections::HashSet;
use itertools::Itertools;
use nalgebra::DMatrix;
use petgraph::graph::{node_index, NodeIndex, UnGraph};
use petgraph::graphmap::UnGraphMap;
use petgraph::data::FromElements;
use petgraph::algo::dijkstra;
use petgraph::dot::{Dot, Config};

type MatIndex = (usize,usize);

fn char_to_altitude(c: char) -> u32 {
    return match (c, c.is_lowercase()) {
        ('S',_) => 0,
        ('E',_) => 27,
        (c, true) => c.to_digit(36).unwrap().abs_diff(9),
        (_,_) => u32::MAX,
    };
}

fn add_edge_if_valid(
    idx1: MatIndex, 
    idx2: MatIndex, 
    altitudes: &DMatrix<u32>, 
    graph_edges: &mut Vec<(MatIndex,MatIndex)>) 
{
    let d1 = idx2.0.abs_diff(idx1.0);
    let d2 = idx2.1.abs_diff(idx1.0);
    if (d1 + d2) != 0 && d1*d2 == 0 && altitudes[idx1].abs_diff(altitudes[idx2]) <= 1 {
        graph_edges.push((idx1,idx2));
    }
}

pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let data = contents.lines().map(|l| l.trim().chars().collect_vec()).collect_vec();
    let (nrows, ncols) = (data.len(), data[0].len());

    // Converts Cartesian coordinates to linear ones
    let cart_to_lin = |(i,j): MatIndex| i*ncols + j;
    let lin_to_cart = |i: usize| ((i/ncols), i%ncols);

    let altitudes = DMatrix::from_fn(nrows,ncols, |i,j| char_to_altitude(data[i][j]));
    let mut graph_edges = Vec::new();

    println!("{}", altitudes);

    for i in 0..(nrows-1) {
        add_edge_if_valid((i,0), (i+1,0), &altitudes, &mut graph_edges);
        for j in 0..(ncols-1) {
            add_edge_if_valid((i,j), (i,j+1), &altitudes, &mut graph_edges);
            add_edge_if_valid((i,j), (i+1,j), &altitudes, &mut graph_edges);
        }
    }

    // // Weirdly, this doesn't work (generating a graph from a vec of edges of type (i32,i32))
    let e = graph_edges.iter().map(|(i1,i2)| (cart_to_lin(*i1) as i32, cart_to_lin(*i2) as i32)).collect_vec();
    // let g = UnGraph::<i32,()>::from_edges(&e);
    // // but this works (generating a graph from a vec of edges of type (i32,i32))....
    // let x = vec![(1, 2), (2, 3), (3, 4), (1, 4)];
    // let g = UnGraph::<i32, ()>::from_edges(&x);

    let g = UnGraphMap::<MatIndex, ()>::from_edges(&graph_edges);
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    let node_map = dijkstra(&g, (0,0), Some((3,6)), |_| 1);
    println!("{:?}", node_map);

    // So we have to do a long workaround D:
    // let mut g = UnGraph::<MatIndex, ()>::new_undirected();
    // let cart_to_node = DMatrix::from_fn(nrows, ncols, |i,j| NodeIndex::<u32>::new(cart_to_lin((i,j))));
    // println!("{:?}", cart_to_node);
    // for i in 0..nrows {
    //     for j in 0..ncols {
    //         g.add_node((i,j));
    //         // // We can check the identity
    //         // let x = g.add_node((i,j));
    //         // println!("{:?} == {:?}? {}", cart_to_node[(i,j)], x, cart_to_node[(i,j)] == x);
    //     }
    // }
    // for (idx1, idx2) in graph_edges.iter() {
    //     g.add_edge(cart_to_node[*idx1], cart_to_node[*idx2], ());
    // }

    return 0;
}