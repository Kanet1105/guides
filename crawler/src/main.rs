fn main() {
    let mut a = String::new();
    let b = &a;
    let c = &mut a;
    c.push_str("Hello, world!");
    // println!("{:?}", b);
}
