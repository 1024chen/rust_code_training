use std::env;
use std::process;

use minigrep::Config; //将 minigrep crate 引入 src/main.rs 的作用域中
fn main() {
    let args: Vec<String> = env::args().collect(); //调用了 env::args，并立即使用 collect 来创建了一个包含迭代器所有值的 vector
    let config = Config::new(&args).unwrap_or_else(|err| {
        //当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。然而，当其值是 Err 时，该方法会调用一个 闭包（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数
        eprintln!("Problem parsing arguments:{}", err); //使用 eprintln! 将错误信息写入标准错误而不是标准输出
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);

    //if let 来检查 run 是否返回一个 Err 值
    if let Err(e) = minigrep::run(config) {
        //添加了一行 use minigrep::Config，它将 Config 类型引入作用域，并使用 crate 名称作为 run 函数的前缀
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
