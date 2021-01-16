//代码清单2-38：单元结构体示例
struct Empty;

fn main() {
    let x = Empty;
    println!("{:p}",&x);
    let y = x;
    println!("{:p}",&y);
    let z = Empty;
    println!("{:p}",&z);
    assert_eq!((..),std::ops::RangeFull);
}