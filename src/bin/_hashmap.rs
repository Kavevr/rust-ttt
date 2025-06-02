use std::{collections::{hash_map, HashMap}, hash::Hash};

fn main() {
    // // HashMap::with_capacity(100)
    // let mut my_gems = HashMap::new();
    // my_gems.insert("宝马1", 1);
    // my_gems.insert("宝马2", 2);
    // my_gems.insert("宝马3", 3);
    // my_gems.insert("宝马4", 4);

    use std::collections::HashMap;
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    // let mut teams_map = HashMap::new();
    // for team in &teams_list {
    //     teams_map.insert(&team.0, team.1);
    //     // println!("{} => {}",team.0,team.1);
    // }

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();

    println!("{:?}",teams_map);


    
}