fn main() {
    let arr = vec![Box::new(1),Box::new(2)];
    let a = &arr[0];
    println!("{}",a);
}