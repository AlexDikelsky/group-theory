use crate::groups::Group;
use itertools::Itertools;
use std::collections::HashMap;

type Permutation = HashMap<usize, usize>;

fn comp(n: &Permutation, k: &Permutation) -> Permutation {
    n.iter().map(|(input, output)| (*input, k[&output])).collect()
}

fn even_perms(n: usize) -> Vec<Permutation> {

}

#[test]
fn compp_test() {
    let a: Permutation = [(0, 1), (1, 0), (2, 3), (3, 2)].iter().copied().collect();
    let e: Permutation = [(0, 0), (1, 1), (2, 2), (3, 3)].iter().copied().collect();
    assert!(comp(&a, &a) == e);
}
