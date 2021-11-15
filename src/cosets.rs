use crate::groups::GroupElement;
use std::cmp::Ordering;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::Ord;

#[derive(Eq,PartialEq,PartialOrd,Debug,Clone,Hash)]
pub struct Coset<T: GroupElement> {
    pub full_set: Vec<T>,
    pub elem: T
}

impl<T: GroupElement> Ord for Coset<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.full_set.cmp(&other.full_set)
    }
}
