//展示了通过 as 重命名了其中一个 Result 类型
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}
fn function2() -> IoResult<()> {
    Ok(())
}

fn main() {

}
