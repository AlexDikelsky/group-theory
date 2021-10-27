use crate::alternating::alternating;
use crate::cyclic::cyclic;
use crate::dihedral::dihedral;
use crate::groups::Group;
use crate::mult_mod_n::multiplicitive;
use crate::permutation::permutation;
use crate::permutation::follow;

#[test]
fn s() {
    let d4 = dihedral(4);
    assert!(d4.set.len() == 8);
    let k = ["", "r", "rr", "rrr"].iter().map(|x| x.to_string()).collect();
    let v = d4.subgroup(k);
    assert!(v.set.len() == 4);
}

#[test]
#[should_panic]
fn s2() {
    let d4 = dihedral(4);

    // not a subgroup
    let k = ["", "rr", "rrr"].iter().map(|x| x.to_string()).collect();
    let v = d4.subgroup(k);
}

#[test]
fn s3() {
    let s3 = permutation(3);
    let h1 = s3.subgroup(vec![vec![1,0,2], vec![0,1,2]]);
    let s3 = permutation(3);
    assert!(h1.left_cosets(&s3) != h1.right_cosets(&s3));
}
