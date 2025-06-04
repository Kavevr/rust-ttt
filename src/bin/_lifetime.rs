fn main() {
{
    let s1 = String::from("long string");
    let s2 = String::from("short");
    let result = longest(&s1, &s2);  
    println!("The longest string is {}", result);
}
    println!("s1 {}",s1);
}
fn longest<'a>(x: &'a str, y: &'a str ) -> &'a str {
    if x.len() > y.len(){
        x   
    }else {
        y
    }
}
