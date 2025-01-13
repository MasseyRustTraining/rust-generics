use rust_generics::{max, min};

fn main() {
    let a = [5, 1, 3];
    println!("'center': {}", (max(&a).unwrap() + min(&a).unwrap()) / 2)
}
