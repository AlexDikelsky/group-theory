use crate::groups::Group;
use std::iter::once;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

fn simplify1(ops: &str, n: usize) -> String {
    let r4: String = (0..n).map(|_| 'r').collect();
    let r3s: String = (0..(n-1)).map(|_| 'r').chain(once('s')).collect();

    // Replace r^n with identity
    let ops = ops.replace(&r4, "");

    // Replace s^2 with identity
    let ops = ops.replace("ss", "");

    // Replace rs = sr^{-1}
    let ops = ops.replace("sr", &r3s);

    ops
}

pub fn simplify(ops: String, n: usize) -> String {
    (0..).fold_while(ops, |old, _| {
        let new = simplify1(&old, n);
        if new == old {
            Done(new)
        } else {
            Continue(new)
        }
    }).into_inner().to_string()
}

pub fn dihedral(n: usize) -> Group<String> {
    let rs: Vec<String> = (0..n).map(|n| (0..n).map(|_| 'r').collect()).collect();
    let srs: Vec<String> = rs.clone().into_iter().map(|rs| rs + "s").collect();
    Group {
        set: rs.into_iter().chain(srs.into_iter()).collect(),
        op: Box::new(move |x, y| simplify(x.to_string() + y, n)),
        id: "".to_string(),
    }
}

// #[test]
// fn simplify_test() {
//     assert!(simplify("rrrr", 4) == "");
//     assert!(simplify("rrrr", 5) == "rrrr");
// 
//     assert!(simplify("ss", 5) == "");
// 
//     assert!(simplify("srsr", 4) == "");
//     assert!(simplify("rr", 4) == "rr");
//     assert!(simplify("sr", 4) == "rrrs");
// }
