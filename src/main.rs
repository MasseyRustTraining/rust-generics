use rust_generics::{max, min};

fn main() {
    let a = [5, 1, 3];
    let center = (max(&a).unwrap() + min(&a).unwrap()) / 2;
    match center {
        n if n & 1 == 1 => println!("odd center {}", n),
        n => println!("even center {}", n),
    }
}
