#![feature(trait_alias)]
#![feature(is_sorted)]

mod dihedral;
mod groups;
mod alternating;
mod permutation;

use crate::groups::Group;
use crate::dihedral::dihedral;
use crate::permutation::permutation;
use crate::alternating::alternating;

fn main() {
    // let z13c = Group {
    //     set: (1..=12).collect::<Vec<usize>>(),
    //     op: Box::new(|x, y| (x * y) % 13),
    //     id: 1,
    // };
    // let z12 = Group {
    //     set: (0..=11).collect::<Vec<usize>>(),
    //     op: Box::new(|x, y| (x + y) % 12),
    //     id: 0,
    // };

    // let z4 = Group {
    //     set: (0..=3).collect::<Vec<usize>>(),
    //     op: Box::new(|x, y| (x + y) % 4),
    //     id: 0,
    // };
    // let v4 = Group {
    //     set: vec!['a', 'b', 'c', 'e'],
    //     op: Box::new(|x, y| match (x, y) {
    //         ('e', y) => y,
    //         (x, 'e') => x,
    //         ('a', 'b') => 'c',
    //         ('a', 'c') => 'b',
    //         ('b', 'a') => 'c',
    //         ('b', 'c') => 'a',
    //         ('c', 'a') => 'b',
    //         ('c', 'b') => 'a',
    //         (x, _) => 'e',
    //     }),
    //     id: 'e',
    // };

    
    dbg!(alternating(4).isomorphisms(&dihedral(6)));
}
