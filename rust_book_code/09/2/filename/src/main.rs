use std::fs::File;
use std::io;
use std::io::Read;

//如果这个函数没有出任何错误成功返回，函数的调用者会收到一个包含 String 的 Ok 值 —— 函数从文件中读取到的用户名。
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    //在变量 s 中创建了一个新 String 并调用文件句柄 f 的 read_to_string 方法来将文件的内容读取到 s 中
    //read_to_string（）也返回一个Result
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn main() {}
