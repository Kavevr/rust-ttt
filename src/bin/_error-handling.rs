use std::{fs::File, io::{self, Read}};


fn main() {
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("src/bin/hello.txt");
    
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    // 简化方法

    let mut f = File::open("src/bin/hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

}