use crate::groups::Group;


pub fn additive(n: usize) -> Group<usize> {
    Group {
        set: (1..n).collect(),
        op: Box::new(move |x, y| (x + y) % n),
        id: 0,
    }
}
