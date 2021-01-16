//match分支使用了模式匹配（Pattern Matching）技术，在编程语言中，模式匹配用于判断类型或值是否存在可以匹配的模式
fn main() {
    let number = 42;
    match number {
        0 => println!("Origin"),//单个值
        1...3 => println!("All"),//范围
        | 5 | 7 | 13 => println!("Bad Luck"),//多个可选值
        n @ 42 => println!("Answer is {}", n),//绑定模式，@将模式中的值绑定给变量，供分支右侧代码使用
        _ => println!("Common"),//通配符,处理剩余情况
    }
}