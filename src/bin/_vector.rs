fn main(){
    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(2);
    // v.push(2);
    // println!("{:?}",v);
    {
        let mut v = vec![13,3123,123,312];
        v.push(1);

        let third = &v[2];
        println!("第三個元素: {}", third);

        match v.get(1) {
            Some(third) => println!("第三個元素是 {third}"),
            None => println!("沒有")
        }


    }
    // v.push(2);

}