//bool类型示例
fn main() {
    let x = true;//自动推断类型为bool
    let y : bool = false;//显式声明
    let x = 5;
    //Rust不支持将0和1转换为bool
    if x > 1 { println!("x is bigger than 1")};
    assert_eq!(x as i32, 1);
    assert_eq!(y as i32, 0);
}