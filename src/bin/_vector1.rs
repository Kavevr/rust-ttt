
// fn main(){
//     let mut v = vec![1,2,3,4,5];
//     let first = &v[0];
//     // v.push(6);

//     println!("The first element is:{first}");

//     for i in &v {
//         println!("{i}")
//     }


// }

// #[derive(Debug)]

// enum  IpAddr {
//     V4(String),
//     V6(String)
// }


// fn main() {
//     let v = vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];

//     for ip  in v {
//         show_addr(ip);
//     }
// }

// fn show_addr(ip: IpAddr) {
//     println!("{:?}",ip);
// }


// trait IpAddr {
//     fn display(&self);

// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}",self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}",self.0)
//     }   
// }
// fn main() {
//     let v: Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string()))
//     ];


//     for ip in v {
//         ip.display();
//     }
// }

// use std::vec;

// fn main() {
//     let v = vec![1;3];
//     let v_from = Vec::from([1,1,1]);
//     assert_eq!(v,v_from);
// }


fn main() {
    let mut v = Vec::with_capacity(10);
    v.extend([1,2,3]);
    println!("Vector 长度是：{}, 容量是: {}",v.len(),v.capacity());

    v.reserve(100);
    println!("Vector (reserve) 长度是：{}, 容量是: {}",v.len(),v.capacity());

    // v.shrink_to_fit();
    // println!("Vector (shrink) 长度是：{}, 容量是: {}",v.len(),v.capacity());


    // assert!(!v.is_empty());
    v.insert(2, 0xff);
    v.insert(2, 9);
    v.insert(2, 9);
    v.insert(2, 9);

    v.remove(0);
    // let a = v.pop();
    // if let Some(n) = v.pop(){
    //     println!("{n}");
    // }

    // println!("{:?}",a);

    // v.clear();

    // let mut v1 = [11,22].to_vec();
    // v.append(&mut v1);

    let mut v1 = [11,22].to_vec();
    v.append(&mut v1);

    v.truncate(30);

    v.retain(|x| *x > 10);

    println!("当前容器容量: {}",v.capacity());
    println!("当前数组长度: {}",v.len());
    println!("{:?}",v);

}