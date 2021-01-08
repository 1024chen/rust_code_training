use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    //如果调用这段代码时不存在 hello.txt 文件，我们将会看到一个 unwrap 调用 panic! 时提供的错误信息

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    //expect 与 unwrap 的使用方式一样：返回文件句柄或调用 panic! 宏。
    //expect 用来调用 panic! 的错误信息将会作为参数传递给 expect ，而不像 unwrap 那样使用默认的 panic! 信息

    
}
