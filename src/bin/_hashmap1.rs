use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    // let team_name = String::from("Blue");
    // if let Some(a) = scores.get(&team_name)  {
    //     println!("{}",a)
    // };
    // for (key, value) in &scores {
    //     println!("{} : {}",key,value);
    // }

    let old = scores.insert("Blue".to_string(), 20);


      
    
}
