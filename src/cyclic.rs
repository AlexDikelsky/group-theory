use crate::groups::Group;


pub fn cyclic(n: usize) -> Group<usize> {
    Group {
        set: (0..n).collect(),
        op: Box::new(move |x, y| (x + y) % n),
        id: 0,
    }
}
