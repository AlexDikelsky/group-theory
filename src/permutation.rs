use crate::groups::Group;
use itertools::Itertools;

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
