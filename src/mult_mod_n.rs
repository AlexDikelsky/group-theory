use crate::groups::Group;
use num_integer::gcd;


pub fn multiplicitive(n: usize) -> Group<usize> {
    Group {
        set: (1..n).filter(|x| gcd(*x, n) == 1).collect(),
        op: Box::new(move |x, y| (x * y) % n),
        id: 1,
    }
}
