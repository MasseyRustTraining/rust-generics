#[derive(Copy, Clone)]
struct S<T> {
    x: (),
    y: u32,
    z: T,
}

/*
impl<T: Clone> Clone for S<T> {
    fn clone(&self) -> Self {
        S {
            x: (),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl<T: Copy> Copy for S<T> {}
*/

fn main() {
    let _ = S { x: (), y: 0, z: "z".to_string() };
}
