use std::fmt;

struct Point{
    x:i32,
    y:i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point(x: aaa{}, y: {})",self.x,self.y)
    }
}
fn main(){
    let p = Point {x:10, y:20};
    println!("{}",p);
}

