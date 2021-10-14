use crate::groups::Group;
use crate::permutation::permutation;
use crate::permutation::PermutationFlat;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter;

fn insert_sort_permutations(a: &Vec<usize>) -> usize {
    let mut a = a.clone();
    let mut count = 0;
    for i in 0..a.len() - 1 {
        let (_, s) = a[i..].iter().zip(i..).min_by_key(|(a, _)| *a).unwrap();
        if s != i {
            count += 1;
        };
        a.swap(i, s);
    }
    count
}

pub fn alternating(n: usize) -> Group<PermutationFlat> {
    let ps = permutation(n);
    let set = ps
        .set
        .into_iter()
        .filter(|p| insert_sort_permutations(p) % 2 == 0)
        .collect();

    Group {
        set: set,
        op: ps.op,
        id: ps.id,
    }
}
