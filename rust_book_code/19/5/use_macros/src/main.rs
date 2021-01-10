///宏（Macro）指的是 Rust 中一系列的功能：声明（Declarative）宏，使用 macro_rules!，和三种 过程（Procedural）宏：
///
/// 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
/// 类属性（Attribute-like）宏定义可用于任意项的自定义属性
/// 类函数宏看起来像函数不过作用于作为参数传递的 token。
/// 从根本上来说，宏是一种为写其他代码而写代码的方式，即所谓的 元编程（metaprogramming）
///宏和函数的最后一个重要的区别是：在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用
fn main() {
    println!("Hello, world!");
}
