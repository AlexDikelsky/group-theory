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
    let transpositions: Vec<Vec<usize>> = (0..n).permutations(2).collect();
    dbg!(&transpositions);
    (0..(n/2)).map(|to_use| transpositions.iter().combinations(n * 2).fold(&id, comp)).collect()
    vec![]
}

#[test]
fn compp_test() {
    let a: Permutation = [1, 0, 3, 2].to_vec();
    let e: Permutation = [0, 1, 2, 3].to_vec();
    assert!(comp(&a, &a) == e);
    assert!(e == identity(4));
}
