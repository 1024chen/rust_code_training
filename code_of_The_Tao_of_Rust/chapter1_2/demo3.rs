//词法作用域示例
fn main() {
    let v = "hello world";
    assert_eq!(v,"hello world");
    let v="hello Rust!";//连续声明即变量遮蔽
    assert_eq!(v,"hello world!");
    //块空间，实际上是一段词法作用域
    {
        let v = "hello world!";
        assert_eq!(v,"hello world!");
    }
    assert_eq!(v,"hello Rust!");
}