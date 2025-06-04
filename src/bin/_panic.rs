fn main(){
    // let v = vec![1,2,3];

    // v[99];
    // // panic!("crash an burn");

    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home);
}