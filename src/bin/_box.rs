fn main() {
    // let arr = [0;1000];
    // let arr1 = arr;

    // println!("{:?}", arr.len());
    // println!("{:?}", arr1.len());
    // let arr = Box::new([0;1000]);
    // // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // // 所有权顺利转移给 arr1，arr 不再拥有所有权
    // let arr1 = arr;
    // println!("{:?}", arr1.len());

    let elems : Vec<Box<dyn Draw>>= vec![Box::new(Button{id:1}),Box::new(Select{id:2})];

    for e in elems {
        e.draw();
    }
}
trait Draw {
    fn draw(&self); 
}

struct Button {
    id: u32
}

impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
    
}

struct Select {
    id:u32
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}",self.id)
    }
}