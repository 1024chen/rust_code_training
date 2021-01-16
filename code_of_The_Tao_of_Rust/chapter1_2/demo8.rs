//闭包作为参数
//Rust中闭包实际就是由一个匿名结构体和trait来组合实现的
fn closure_math<F:Fn() -> i32>(op: F) -> i32 {
    op()
}
fn main() {
    let a = 2;
    let b = 3;
    assert_eq!(closure_math(|| a+b), 5);
    assert_eq!(closure_math(|| a*b), 6);
    //分别传入两个闭包
}