// Const Generics

struct Bins<const N: usize>([u8; N]);

impl<const N: usize> Bins<N> {
    fn new() -> Self {
        Bins([0; N])
    }
}

fn main() {
    let mut b: Bins::<100> = Bins::new();
    b.0[1] = 7;
    let c = Bins([1, 0]);
    println!("{}", b.0[1] == c.0[1]);
}
