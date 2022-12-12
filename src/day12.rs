use std::fs;
const INPUT_FILE: &str = "data/input12.txt";

use itertools::Itertools;
use nalgebra::DMatrix;
use petgraph::graphmap::DiGraphMap;
use petgraph::algo::dijkstra;
// use petgraph::dot::{Dot, Config};
use std::cmp::{max,min};

type MatIndex = (usize,usize);

fn char_to_altitude(c: char) -> u32 {
    return match (c, c.is_lowercase()) {
        ('S',_) => 1,
        ('E',_) => 26,
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
    let d2 = idx2.1.abs_diff(idx1.1);
    if (d1 + d2) != 0 && d1 * d2 == 0 {
        if altitudes[idx2] <= altitudes[idx1] + 1 {
            graph_edges.push((idx1,idx2));
        }
    }
}

pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let data = contents.lines().map(|l| l.trim().chars().collect_vec()).collect_vec();
    let (nrows, ncols) = (data.len(), data[0].len());

    let data = DMatrix::from_fn(nrows, ncols, |i,j| data[i][j]);
    let altitudes = data.map(char_to_altitude);
    
    // println!("{} becomes {}", data, altitudes);
    
    let mut graph_edges = Vec::new();
    for i in 0..nrows {
        for j in 0..ncols {
            let u = (max(1,i)-1, j);
            let d = (min(nrows-1,i+1), j);
            let l = (i, max(1,j)-1);
            let r = (i, min(ncols-1,j+1));
            for n in [u,d,l,r] {
                add_edge_if_valid((i,j), n, &altitudes, &mut graph_edges);
            }
        }
    }

    let g = DiGraphMap::<MatIndex, ()>::from_edges(&graph_edges);
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let start_node = (0..nrows).cartesian_product(0..ncols).find(|(i,j)| data[(*i,*j)] == 'S').unwrap();
    let end_node = (0..nrows).cartesian_product(0..ncols).find(|(i,j)| data[(*i,*j)] == 'E').unwrap();
    let node_map = dijkstra(&g, start_node, Some(end_node), |_| 1);

    return node_map[&end_node];
}

pub fn main_bonus() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");

    let data = contents.lines().map(|l| l.trim().chars().collect_vec()).collect_vec();
    let (nrows, ncols) = (data.len(), data[0].len());

    let data = DMatrix::from_fn(nrows, ncols, |i,j| data[i][j]);
    let altitudes = data.map(char_to_altitude);
        
    let mut graph_edges = Vec::new();
    for (i,j) in (0..nrows).cartesian_product(0..ncols) {
        let u = (max(1,i)-1, j);
        let d = (min(nrows-1,i+1), j);
        let l = (i, max(1,j)-1);
        let r = (i, min(ncols-1,j+1));
        for n in [u,d,l,r] {
            add_edge_if_valid((i,j), n, &altitudes, &mut graph_edges);
        }
    }

    let reversed_edges = graph_edges.iter().map(|&(i1,i2)| (i2,i1));
    let g = DiGraphMap::<MatIndex, ()>::from_edges(reversed_edges);
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let start_node = (0..nrows).cartesian_product(0..ncols).find(|(i,j)| data[(*i,*j)] == 'E').unwrap();
    let node_map = dijkstra(&g, start_node, None, |_| 1);

    let shortest_hike = (0..nrows).cartesian_product(0..ncols)
        .filter(|i| data[*i] == 'a')
        .map(|i| *node_map.get(&i).unwrap_or(&u32::MAX))
        .min().unwrap();

    return shortest_hike;
}