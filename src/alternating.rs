use crate::groups::Group;
use itertools::Itertools;
use std::collections::HashMap;

type Permutation = Vec<usize>;

fn comp(n: &Permutation, k: &Permutation) -> Permutation {
    n.iter().map(|input| k[*input]).collect()
}

fn identity(n: usize) -> Permutation {
    (0..n).collect()
}

pub fn even_perms(n: usize) -> Vec<Permutation> {
    let id = identity(n);
    let transposition_locations: Vec<(usize, usize)> = (0..n).permutations(2).map(|a| (a[0], a[1])).collect();
    dbg!(&transposition_locations);
    let transpositions: Vec<Permutation> = transposition_locations.into_iter().map(|(a, b)| {
        let mut func: Permutation = (0..n).collect();
        func.swap(a, b);
        dbg!(&func, a, b);
        func
    }).collect();
    dbg!(&transpositions);
    let in_n_transpositions = |n: usize| -> Vec<Vec<Permutation>> {
        transpositions.clone().into_iter().permutations(n).collect()
    };

    // dbg!(&in_n_transpositions(2));
    // let a: Vec<Permutation> = 
    //     (0..(n/2)).map(|n| transpositions.iter().permutations(n * 2)
    //                    .map(|perm| perm.iter().fold(&id, |state, x| &comp(state, x))).collect()).collect()
    vec![]
}

#[test]
fn compp_test() {
    let a: Permutation = [1, 0, 3, 2].to_vec();
    let e: Permutation = [0, 1, 2, 3].to_vec();
    assert!(comp(&a, &a) == e);
    assert!(e == identity(4));
}
