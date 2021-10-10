use itertools::Itertools;
use std::cmp::Ord;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::fmt;

pub trait GroupElement = Clone + Hash + Ord + Eq + Debug;

// A group is a set and a binary operation
// where we can tell if x == y for all x,y in G
// I'm only considering groups where you can represent the elements as
// nonnegative integers to make the code a bit cleaner
pub struct Group<T: GroupElement> {
    pub set: Vec<T>,
    pub op: Box<dyn Fn(T, T) -> T>,
    pub id: T,
}

impl<T: GroupElement> Debug for Group<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Group")
         .field("set", &self.set)
         .field("id", &self.id)
         .finish()
    }
}

impl<T: GroupElement> Group<T> {
    // Find the order of every element of G
    pub fn orders(&self) -> Vec<(usize, T)> {
        let order = |x: T| {
            (0..)
                .scan(self.id.clone(), move |state, _| {
                    *state = (self.op)(x.clone(), state.clone());
                    Some(state.clone())
                })
                .take_while(|x| *x != self.id)
                .count()
                + 1
        };
        self.set.iter().map(|x| (order(x.clone()), x.clone())).collect()
    }

    // Find the isomorhpisms for G and H
    pub fn isomorphisms<U: GroupElement>(&self, other: &Group<U>) -> Vec<Vec<(T, U)>> {
        let a = self.orders().into_iter().into_group_map();
        let b = other.orders().into_iter().into_group_map();
        println!("{:?}, {:?}",&a, &b);

        // First make sure the orders match up
        if a.keys().all(|k| a[k].len() == b[k].len()) {

            // Function for finding the possible bijections for order k elements of G and H
            let poss_for_order_n = |k| -> Vec<Vec<(T, U)>> {
                b[&k]
                    .clone()
                    .into_iter()
                    .permutations(b[&k].len())
                    .map(|perm| a[&k].clone().into_iter().zip(perm.into_iter()).collect())
                    .collect()
            };

            let isomorphisms: Vec<HashMap<T, U>> = a
                .keys()                                             // For each order
                .map(|x| poss_for_order_n(*x).into_iter())          // Find the bijections for that order
                .multi_cartesian_product()                          // Expand out the bijections
                .map(|bij| bij.into_iter().flatten().collect())
                .filter(|x| is_isomorphism(x, self, other))         // Keep only the isomorphisms
                .collect();

            // This sorts the isomorphisms nicely
            isomorphisms
                .into_iter()
                .map(|iso| {
                    let mut a: Vec<(T, U)> = iso.into_iter().collect();
                    a.sort_by_key(|(b, _)| b.clone());
                    a
                })
                .collect()
        } else {
            vec![]
        }
    }
}

fn is_isomorphism<T: GroupElement, U: GroupElement>(
    biject: &HashMap<T, U>,
    g1: &Group<T>,
    g2: &Group<U>,
) -> bool {
    g1.set
        .iter()
        .cartesian_product(g1.set.iter())
        .all(|(x, y)| biject[&(g1.op)(x.clone(), y.clone())] == (g2.op)(biject[x].clone(), biject[y].clone()))
}
