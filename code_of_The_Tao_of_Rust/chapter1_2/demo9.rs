//闭包作为返回值
fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j*i //将捕获变量的所有权转移到闭包中，不按引用进行捕获变量，这样可以安全的返回闭包
}
fn main() {
    let result = two_times_impl();
    assert_eq!(result(2), 4);
}