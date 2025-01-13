pub type Value = u32;

/// Returns a maximum value from the slice `a`, or
/// `None` if `a` is empty.
pub fn max(a: &[Value]) -> Option<Value> {
    if a.is_empty() {
        return None;
    }
    let mut m = a[0];
    for x in &a[1..] {
        if *x > m {
            m = *x;
        }
    }
    Some(m)
}

/// Returns a minimum value from the slice `a`, or
/// `None` if `a` is empty.
pub fn min(a: &[Value]) -> Option<Value> {
    if a.is_empty() {
        return None;
    }
    let mut m = a[0];
    for &x in &a[1..] {
        if x < m {
            m = x;
        }
    }
    Some(m)
}

/*
struct S {
    x: u32,
    y: u32,
    z: u32,
}

pub fn f({x, y, ...}: S) {
}
*/

#[test]
fn test_max() {
    assert_eq!(Some(3), max(&[1, 3, 2]));
    assert_eq!(None, max(&[]));
}
