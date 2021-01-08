fn main() {
    //let v: Vec<i32> = Vec::new();

    //vec宏会根据我们提供的值来创建一个新的 Vec
    //let v: Vec<i32> = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    //访问 vector 中一个值的两种方式，索引语法或者 get 方法：

    //索引语法,使用 & 和 [] 返回一个引用
    let third: &i32 = &v[2];
    println!("the third element is {}", third);

    //get方法以索引作为参数来返回一个 Option<&T>
    match v.get(2) {
        Some(third) => println!("third element is {}", third),
        None => println!("no third element."),
    }
    /*
        get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None。
        当偶尔出现超过 vector 范围的访问属于正常情况的时候可以考虑使用它。
        接着你的代码可以有处理 Some(&element) 或 None 的逻辑，如第六章讨论的那样。
        例如，索引可能来源于用户输入的数字。
        如果它们不慎输入了一个过大的数字那么程序就会得到 None 值，你可以告诉用户当前 vector 元素的数量并再请求它们输入一个有效的值。
        这就比因为输入错误而使程序崩溃要友好的多！
    */
}
