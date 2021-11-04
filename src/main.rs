#![feature(trait_alias)]
#![feature(is_sorted)]

mod alternating;
mod cyclic;
mod dihedral;
mod groups;
mod mult_mod_n;
mod permutation;
mod test;

use crate::alternating::alternating;
use crate::cyclic::cyclic;
use crate::dihedral::dihedral;
use crate::groups::Group;
use crate::mult_mod_n::multiplicitive;
use crate::permutation::cycles;
use crate::permutation::permutation;

use itertools::Itertools;

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
    let v4 = Group {
        set: vec!['a', 'b', 'c', 'e'],
        op: Box::new(|x, y| match (x, y) {
            ('e', y) => *y,
            (x, 'e') => *x,
            ('a', 'b') => 'c',
            ('a', 'c') => 'b',
            ('b', 'a') => 'c',
            ('b', 'c') => 'a',
            ('c', 'a') => 'b',
            ('c', 'b') => 'a',
            (x, _) => 'e',
        }),
        id: 'e',
    };

    // dbg!(alternating(4).isomorphisms(&dihedral(6)));
    // dbg!(multiplicitive(8).isomorphisms(&v4));
    // for iso in multiplicitive(13).isomorphisms(&cyclic(12)) {
    //     println!("{:?}", &iso);
    // }

    // for iso in permutation(3).isomorphisms(&dihedral(3)) {
    //     println!("{:?}", &iso);
    // }

    // let a5 = alternating(5);
    // let z2 = multiplicitive(8);
    // // let a5xz2 = Group {
    // //     set: a5.set.clone().into_iter().cartesian_product(z2.set.clone().into_iter()).collect(),
    // //     id: (a5.id.clone(), z2.id.clone()),
    // //     op: Box::new(move |(a1, b1), (a2, b2)| ((a5.op)(a1, a2), (z2.op)(b1, b2))),
    // // };
    // // dbg!(a5xz2.isomorphisms(&permutation(6)));

    // println!("{:?}", z2.order_lens());

    // let z2z2z2 = permutation(5);
    // println!("{:?}", z2z2z2.order_lens());

    // let s3 = permutation(3);
    // let h1 = s3.subgroup(vec![vec![1,0,2], vec![0,1,2]]);
}
