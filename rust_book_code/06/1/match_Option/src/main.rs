//编写一个函数，它获取一个 Option<i32> 并且如果其中有一个值，将其加一。
//如果其中没有值，函数应该返回 None 值并不尝试执行任何操作
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
