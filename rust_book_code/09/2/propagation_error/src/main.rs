use std::fs::File;
use std::io;
use std::io::Read;

//传播错误这里没看懂

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt");
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {}
