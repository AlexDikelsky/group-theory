use itertools::Itertools;
use std::cmp::Ord;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::once;

pub trait GroupElement = Clone + Hash + Ord + Eq + Debug + 'static;

// A group is a set and a binary operation
// where we can tell if x == y for all x,y in G
pub struct Group<T: GroupElement> {
    pub set: Vec<T>,
    pub op: Box<dyn Fn(&T, &T) -> T>,
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
                    *state = (self.op)(&x, state);
                    Some(state.clone())
                })
                .take_while(|x| *x != self.id)
                .count()
                + 1
        };
        self.set
            .iter()
            .map(|x| (order(x.clone()), x.clone()))
            .collect()
    }

    // (times it occurs, order)
    pub fn order_lens(&self) -> Vec<(usize, usize)> {
        let mut a: Vec<(usize, usize)> 
            = self.orders().into_iter().map(|(n, _)| n).counts().into_iter().collect();
        a.into_iter().map(|(a, b)| (b, a)).collect()
    }

    pub fn left_coset(&self, x: &T) -> Vec<T> {
        let mut k: Vec<T> = (self.set).iter().map(|y| (self.op)(x, y)).collect();
        k.sort();
        k
    }

    pub fn left_cosets(&self, g: &Group<T>) -> HashSet<Vec<T>> {
        g.set.iter().map(|x| self.left_coset(x)).collect()
    }

    pub fn right_coset(&self, x: &T) -> Vec<T> {
        let mut k: Vec<T> = (self.set).iter().map(|y| (self.op)(y, x)).collect();
        k.sort();
        k
    }

    pub fn right_cosets(&self, g: &Group<T>) -> HashSet<Vec<T>> {
        g.set.iter().map(|x| self.right_coset(x)).collect()
    }

    pub fn subgroup(self, s: Vec<T>) -> Group<T> {
        let a: HashSet<&T> = s.iter().collect();
        assert!(!s.is_empty());
        assert!(s.iter().all(|x| s.iter().all(|y| a.contains(&(&(self.op))(x, y)))));
        Group {
            set: s,
            op: self.op,
            id: self.id,
        }
    }

    pub fn gen_by(self, a: &T) -> Group<T> {
        let s = (0..).scan(a.clone(), |state, _| {
            *state = ((self.op))(a, state);
            Some(state.clone())
        }).take_while(|x| x != a).chain(once(a.clone())).collect();
        self.subgroup(s)
    }

    pub fn product<U: GroupElement>(self, other: Group<U>) -> Group<(T, U)> {
        Group {
            set: self
                .set
                .clone()
                .into_iter()
                .cartesian_product(other.set.clone().into_iter())
                .collect(),
            id: (self.id.clone(), other.id.clone()),
            op: Box::new(move |(a1, b1), (a2, b2)| ((self.op)(a1, a2), (other.op)(b1, b2))),
        }
    }

    // Find the isomorhpisms for G and H
    pub fn isomorphisms<U: GroupElement>(&self, other: &Group<U>) -> Vec<Vec<(T, U)>> {
        let a = self.orders().into_iter().into_group_map();
        let b = other.orders().into_iter().into_group_map();
        println!("{:?}, {:?}", &a, &b);

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
                .keys() // For each order
                .map(|x| poss_for_order_n(*x).into_iter()) // Find the bijections for that order
                .multi_cartesian_product() // Expand out the bijections
                .map(|bij| bij.into_iter().flatten().collect())
                .filter(|x| is_isomorphism(x, self, other)) // Keep only the isomorphisms
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
        .all(|(x, y)| biject[&(g1.op)(x, y)] == (g2.op)(&biject[x], &biject[y]))
}
