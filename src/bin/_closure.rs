use std::{thread, time::Duration};

fn main(){
    // let x = 1;
    // let sum = |y| x + y;

    // // println!(sum(3));
    // assert_eq!(3,sum(2));
    let intensity = 10;
    let random_number = 3;

    workout(intensity, random_number);

}
// fn woooo(intensity: u32) -> u32 {
//     println!("muuuu.....");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }    

fn workout(intensity:u32, random_number:u32) {
    // let action = woooo;

    let action = || {
        println!("muuuuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };


    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            action()
        );
        println!(
            "旁边有妹子在看, 俯卧撑太low, 再来 {} 组卧推!",
            action()
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧! ");
    } else {
        println!("昨天练过度了，今天干干有氧，跑步 {} 分钟! ",
        action()
        );
    }
}

