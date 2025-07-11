
// fn main() {

//     // let sum = |x,y| x+y;
//     // let v = sum(1,2); 

//     let example_closure = |x|x;
//     let s = example_closure(String::from("value"));
//     // let n  = example_closure(1);
// }

struct Cacher<T> where T:Fn(u32) -> u32 {
    query:T,
    value: Option<u32>
}
impl <T> Cacher<T>
where 
    T: Fn(u32) -> u32 
{
    fn new(query:T) -> Cacher<T> {
        Cacher { 
            query, 
            value:None
        }
    }

    fn value(&mut self,arg:u32) ->u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v= (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn main() {

}