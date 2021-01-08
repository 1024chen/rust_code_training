/*
impl 块的另一个有用的功能是：允许在 impl 块中定义 不 以 self 作为参数的函数。
这被称为关联函数（associated functions），因为它们与结构体相关联。
它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    println!("sq.width:{}", sq.width);
}
