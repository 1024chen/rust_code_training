fn main() {
    let s: String = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    //start..end 语法代表一个以 start 开头并一直持续到但不包含 end 的 range。
    println!("1:{}", hello);
    println!("2:{}", world);

    //如果需要包含 end，可以使用 ..= 而不是 ..
    let hello: &str = &s[0..=4];
    let world: &str = &s[6..=10];
    println!("1:{}", hello);
    println!("2:{}", world);

    //对于 Rust 的 .. range 语法，如果想要从第一个索引（0）开始，可以不写两个点号之前的值。
    //换句话说，如下两个语句是相同
    let s: String = String::from("hello");
    let slice1: &str = &s[0..2];
    let slice2: &str = &s[..2];
    println!("1:{}", slice1);
    println!("2:{}", slice2);

    //如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字
    let s: String = String::from("hello");
    let len: usize = s.len();
    let slice1: &str = &s[3..len];
    let slice2: &str = &s[3..];
    println!("1:{}", slice1);
    println!("2:{}", slice2);

    //也可以同时舍弃这两个值来获取整个字符串的 slice。
    let s: String = String::from("hello");
    let len: usize = s.len();
    let slice1: &str = &s[0..len];
    let slice2: &str = &s[..];
    println!("1:{}", slice1);
    println!("2:{}", slice2);
}
