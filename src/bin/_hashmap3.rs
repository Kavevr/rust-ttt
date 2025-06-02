// fn main(){

//     use std::collections::HashMap;

//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();
//     // 根据空格来切分字符串(英文单词都是通过空格切分)
//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }

fn main(){

    use std::hash::BuildHasherDefault;
    use std::collections::HashMap;
// 引入第三方的哈希函数
    use twox_hash::XxHash64;

// 指定HashMap使用第三方的哈希函数XxHash64
let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));

}
