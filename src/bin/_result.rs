use std::{f32::consts::E, fmt::Error, fs::File, io::ErrorKind};


fn main() {
    // let f: u32= File::open("hello.txt");
    // let f = File::open("src/bin/hello.txt");

    // let f = match f {
    //     Ok(file)=> file,
    //     Err(error)=> {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };
    // println!("{:?}",f);
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file:{:?}",e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}",other_error),
    //     }
    // };


    // let f  = File::open("src/bin/hello.txt").unwrap();
    // println!("{:?}",f);
    

    let f  = File::open("sc/bin/hello.txt").expect("哦嚯");
    println!("{:?}",f);
    

}