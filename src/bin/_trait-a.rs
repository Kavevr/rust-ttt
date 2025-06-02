pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // String::from("Read more...",)
        format!("(Read more form {}...)",self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章{}, 作者是{},内容是:{}", self.title,self.author,self.content)
//     }
// }

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
}


pub struct Weibo{
    pub username: String,
    pub content: String,
}

// impl Summary for Weibo {
//     // fn summarize(&self) -> String {
//     //     format!("{}发表了微博,{}", self.username,self.content)
//     // }

    
// }

pub fn notify<T: Summary>(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

fn main(){
    let post = Post{
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content:"Rust牛皮!".to_string()
    };
    let weibo = Weibo{
        username:"stoo".to_string(),
        content:"这是一条微博".to_string(),
    };
    // println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    println!("{}", weibo.summarize_author());
    // println!("{}",notify(&weibo));
    // println!("{}",notify(&weibo)
    notify(&weibo)；
}
