fn main() {
    let v: Vec<i32> = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //遍历可变 vector 的每一个元素的可变引用以便能改变他们
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    //因为vector只能存储相同类型的值，因为美剧类型的成员都被定义为相同的枚举类型，
    //当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for elem in &row {
        println!("{:?} in row", elem);
        // if let SpreadsheetCell::Int(x) = elem {
        //     println!("{} in row",x);
        // }
        match elem {
            SpreadsheetCell::Int(x) => {
                println!("{} in row", x);
            }
            SpreadsheetCell::Float(x) => {
                println!("{} in row", x);
            }
            SpreadsheetCell::Text(x) => {
                println!("{} in row", x);
            }
        }
    }
}
