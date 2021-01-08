// 展示了如何将两个不同父模块的 Result 类型引入作用域并引用它们。
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Ok(())
}
fn function2() -> io::Result<()> {
    Ok(())
}

fn main() {

}
