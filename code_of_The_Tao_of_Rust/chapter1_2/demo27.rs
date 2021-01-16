//代码清单2-37：New Type模式示例
struct Integer(u32);

type Int = i32;

fn main() {
    let int = Integer(10);
    assert_eq!(int.0, 10);
    let int: Int = 10;
    assert_eq!(int, 10);
}
