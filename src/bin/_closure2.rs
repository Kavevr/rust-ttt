use std::intrinsics::exp2f128;

fn main() {

    // let sum = |x,y| x+y;
    // let v = sum(1,2); 

    let example_closure = |x|x;
    let s = example_closure(String::from("value"));
    // let n  = example_closure(1);
}