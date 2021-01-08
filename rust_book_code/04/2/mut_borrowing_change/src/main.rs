/*
将 s 改为 mut。然后必须创建一个可变引用 &mut s 和接受一个可变引用 some_string: &mut String。
不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。
*/
fn main() {
    let mut s: String = String::from("hello");
    change(&mut s);
    println!("s:{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}
