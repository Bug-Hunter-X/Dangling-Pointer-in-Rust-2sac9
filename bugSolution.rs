fn main() {
    let mut v = vec![1, 2, 3];
    // safer way to modify vector
    v[0] = 10;
    println!("{:?}", v);
}