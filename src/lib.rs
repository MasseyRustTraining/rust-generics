
/// Returns a maximum value from the slice `a`, or
/// `None` if `a` is empty.
pub fn max<T: PartialOrd + Copy>(a: &[T]) -> Option<T> {
    if a.is_empty() {
        return None;
    }
    let mut m = a[0];
    for &x in &a[1..] {
        if x > m {
            m = x;
        }
    }
    Some(m)
}

/// Returns a minimum value from the slice `a`, or
/// `None` if `a` is empty.
pub fn min<T: PartialOrd + Copy>(a: &[T]) -> Option<T> {
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

#[test]
fn test_max() {
    assert_eq!(Some(3u32), max(&[1, 3, 2]));
    assert_eq!(None, max::<u32>(&[]));
}
