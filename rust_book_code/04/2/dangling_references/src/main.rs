fn main() {
    //悬垂引用报错
    let reference_to_nothing: &String = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // 这里 s 离开作用域并被丢弃。其内存被释放。

//修改方法是直接返回String
fn no_dangle() -> String {
    let s = String::from("hello");

    s
} //这样就没有任何错误了。所有权被移动出去，所以没有值被释放。

/*
让我们概括一下之前对引用的讨论：
在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
引用必须总是有效。
*/
