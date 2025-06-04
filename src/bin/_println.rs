#[derive(Debug)]
struct Person {
    name: String,
    age:u8,
}
fn main() {

    let i = 3.13323;
    let s = String::from("hello");
    let v = vec![1,23,3];
    let p = Person{
        name:"stoo".to_string(),
        age:18  
    };
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);


    
    
}