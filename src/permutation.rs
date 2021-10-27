use crate::groups::Group;
use itertools::Itertools;
use std::iter;

pub type PermutationFlat = Vec<usize>;

fn gen_set(n: usize) -> Vec<PermutationFlat> {
    (0..n).permutations(n).collect()
}

fn comp(a: &PermutationFlat, b: &PermutationFlat) -> PermutationFlat {
    a.iter().map(|c| b[*c]).collect()
}

pub fn permutation(n: usize) -> Group<PermutationFlat> {
    Group {
        set: gen_set(n),
        op: Box::new(comp),
        id: (0..n).collect(),
    }
}

pub fn follow(i: usize, v: &PermutationFlat) -> Vec<usize> {
    iter::once(i).chain(
    v.iter().scan(i, move |state, _| {
        *state = v[*state];
        Some(*state)
    }).take_while(|x| *x != i)).collect()
}

pub fn cycles(v: &PermutationFlat) -> Vec<Vec<usize>> {
    v.iter().copied().filter_map(|x| {
        let k = follow(x, v);
        if *k.iter().min().unwrap_or(&(k[0] + 3)) == k[0] && k.len() != 1 {
            Some(k)
        } else {
            None
        }
    }).collect()
}
