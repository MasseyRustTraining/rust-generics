// Generic Lifetimes

#[derive(Debug)]
struct Name<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str,
}

fn f<'b>(last_name: &'b str) -> Name<'_, 'b> {
    let n = Name { first_name: "Bart", last_name };
    println!("{:?}", n);
    n 
}

fn main() {
    let generic_name = "Doe".to_string();
    let n = f(&generic_name);
    println!("{}", n.first_name);
    drop(generic_name);
}
