#[derive(Debug, PartialEq)]
pub enum Maybe<T> {
    Has(T),
    Empty,
}

impl<T> Maybe<T> {
    /// Returns [[Maybe::Has]] value or panics on [[Maybe::None]].
    pub fn unwrap(self) -> T {
        match self {
            Maybe::Has(x) => x,
            Maybe::Empty => panic!("unwrap of empty maybe"),
        }
    }
}

/// Returns a maximum value from the slice `a`, or
/// `Maybe::Empty` if `a` is empty.
pub fn max(a: &[u32]) -> Maybe<u32> {
    if a.is_empty() {
        return Maybe::Empty;
    }
    let mut m = a[0];
    for x in &a[1..] {
        if *x > m {
            m = *x;
        }
    }
    Maybe::Has(m)
}

/// Returns a minimum value from the slice `a`, or
/// `Maybe::Empty` if `a` is empty.
pub fn min(a: &[u32]) -> Maybe<u32> {
    if a.is_empty() {
        return Maybe::Empty;
    }
    let mut m = a[0];
    for x in &a[1..] {
        if *x < m {
            m = *x;
        }
    }
    Maybe::Has(m)
}

#[test]
fn test_max() {
    assert_eq!(Maybe::Has(3), max(&[1, 3, 2]));
    assert_eq!(Maybe::Empty, max(&[]));
}
