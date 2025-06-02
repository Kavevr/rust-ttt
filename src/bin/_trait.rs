


pub trait Summary{
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{},内容是:{}", self.title,self.author,self.content)
    }
}

pub struct Weibo{
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username,self.content)
        
    }
    
}


fn main(){
    let post = Post{
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content:"Rust牛皮!".to_string()
    };
    println!("{}", post.summarize());
}
