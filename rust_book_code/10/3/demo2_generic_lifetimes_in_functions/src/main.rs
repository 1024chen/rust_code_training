fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

//Rust 并不知道将要返回的引用是指向 x 或 y。
//事实上我们也不知道，因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！
// fn longest(x: &str,y:&str) -> &str {
//     if x.len() > y.len() {
//         x
//     }else {
//         y
//     }
// }
//为了修复这个错误，我们将增加泛型生命周期参数来定义引用间的关系以便借用检查器可以进行分析

//就像泛型类型参数，泛型生命周期参数需要声明在函数名和参数列表间的尖括号中。
//这里我们想要告诉 Rust 关于参数中的引用和返回值之间的限制是他们都必须拥有相同的生命周期，就像示例 10-22 中在每个引用中都加上了 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
