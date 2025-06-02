fn main() {

    let r;

    let x = 4;
    r = &x;
    // {
    //     let x = 5;
    //     let r = &x;
    // }
    println!("{:p}", r);
}