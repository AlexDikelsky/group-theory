use crate::groups::Group;
use itertools::Itertools;
use std::collections::HashMap;

type Permutation = Vec<Vec<usize>>;

fn next_cyclic<T: Copy + Clone>(idx: usize, v: &[T]) -> T {
    if idx + 1 == v.len() {
        v[0]
    } else {
        v[idx+1]
    }
}

fn chase(n: usize, sigma: &Permutation) -> usize {
    dbg!(&n, &sigma);
    if sigma.is_empty() {
        n
    } else {
        let cycle = sigma.iter().last().unwrap();
        chase(match cycle.iter().position(|a| *a == n) {
            Some(idx) => next_cyclic(idx, &cycle),
            None => n,
        }, &sigma[..sigma.len()-1].to_vec())
    }
}

// fn comp(n: &Permutation, k: &Permutation) -> Permutation {
// 
// }
// 
fn identity(n: usize) -> Permutation {
    vec![]
}

// pub fn even_perms(n: usize) -> Vec<Permutation> {
//     let id = identity(n);
//     let transposition_locations: Vec<(usize, usize)> = (0..n).permutations(2).map(|a| (a[0], a[1])).collect();
//     dbg!(&transposition_locations);
//     let transpositions: Vec<Permutation> = transposition_locations.into_iter().map(|(a, b)| {
//         let mut func: Permutation = (0..n).collect();
//         func.swap(a, b);
//         dbg!(&func, a, b);
//         func
//     }).collect();
//     dbg!(&transpositions);
//     let in_n_transpositions = |n: usize| -> Vec<Vec<Permutation>> {
//         transpositions.clone().into_iter().permutations(n).collect()
//     };
// 
//     // dbg!(&in_n_transpositions(2));
//     // let a: Vec<Permutation> = 
//     //     (0..(n/2)).map(|n| transpositions.iter().permutations(n * 2)
//     //                    .map(|perm| perm.iter().fold(&id, |state, x| &comp(state, x))).collect()).collect()
//     vec![]
// }
// 
#[test]
fn comp_test() {
    let a: Permutation = [[1,0],[2,3]].map(|x| x.to_vec()).to_vec();
    let b: Permutation = [[1,0],[1,0]].map(|x| x.to_vec()).to_vec();
    let c: Permutation = [[1,0]].map(|x| x.to_vec()).to_vec();
    let e: Permutation = vec![];

    let k = vec!['a','b','c'];
    assert!(next_cyclic::<char>(0, &k) == 'b');
    assert!(next_cyclic::<char>(1, &k) == 'c');
    assert!(next_cyclic::<char>(2, &k) == 'a');

    // assert!(chase(1, &c) == 0);
    // assert!(chase(0, &c) == 1);
    assert!(chase(0, &b) == 0);
    // assert!(e == identity(4));
}
