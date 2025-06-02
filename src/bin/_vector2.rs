fn main() {
    let v = vec![11,22,33,44,55];
    let slice = &v[1..=4];

    println!("{:?}",slice);
}