//类型别名在两个方面有帮助：易于编写 并 在整个 std::io 中提供了一致的接口。
use std::fmt;
use std::io::Result;

fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let guess = "3";
    loop {//loop循环也是!类型的
        #[allow(unused)]
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,//continue 的值是 !，描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型
        };
        break;
    }
    //允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；相反它把控制权交回上层循环，所以在 Err 的情况，事实上并未对 guess 赋值。
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

//发散函数（diverging functions）
//fn bar() -> ! {
// --snip--
//}

//从不返回的 never type

//动态大小类型和 Sized trait
