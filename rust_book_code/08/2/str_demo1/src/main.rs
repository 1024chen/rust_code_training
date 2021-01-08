fn main() {
    //从字符串字面值创建字符串
    let data = "initial contents";
    let s = data.to_string();
    println!("s:{}", s);
    let s = "initial contents".to_string();
    println!("s:{}", s);

    //使用from从字符串字面量创建
    let s = String::from("initial contents");
    println!("s:{}", s);

    let hello = String::from("السلام عليكم");
    println!("hello:{}", hello);
    let hello = String::from("Dobrý den");
    println!("hello:{}", hello);
    let hello = String::from("Hello");
    println!("hello:{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("hello:{}", hello);
    let hello = String::from("नमस्ते");
    println!("hello:{}", hello);
    let hello = String::from("こんにちは");
    println!("hello:{}", hello);
    let hello = String::from("안녕하세요");
    println!("hello:{}", hello);
    let hello = String::from("你好");
    println!("hello:{}", hello);
    let hello = String::from("Olá");
    println!("hello:{}", hello);
    let hello = String::from("Здравствуйте");
    println!("hello:{}", hello);
    let hello = String::from("Hola");
    println!("hello:{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    //push_str 方法获取字符串 slice，因为我们并不需要获取参数的所有权
    s1.push_str(s2);
    println!("s2 is {}", s2);

    //push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中
    let mut s = String::from("lo");
    s.push('l');

    //使用+运算或format！宏拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3:{}", s3);
    //format!宏
    /*
    format! 与 println! 的工作原理相同，不过不同于将输出打印到屏幕上，它返回一个带有结果内容的 String。
    这个版本就好理解的多，并且不会获取任何参数的所有权。
    */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s:{}", s);

    let len = String::from("Hola").len();
    println!("the length of Hola is {}", len);
    //len 的值是 4 ，这意味着储存字符串 「Hola」 的 Vec 的长度是四个字节：这里每一个字母的 UTF-8 编码都占用一个字节。

    let len = String::from("Здравствуйте").len();
    println!("the length of Hola is {}", len);
    //24。这是使用 UTF-8 编码 「Здравствуйте」 所需要的字节数，这是因为每个 Unicode 标量值需要两个字节存储。因此一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值。

    //字符串slice
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s:{}", s); //s 会是一个 &str，它包含字符串的头四个字节,这意味着 s 将会是 「Зд」

    //操作单独的 Unicode 标量值，最好的选择是使用 chars 方法
    //对 「नमस्ते」 调用 chars 方法会将其分开并返回六个 char 类型的值，接着就可以遍历其结果来访问每一个元素
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //bytes 方法返回每一个原始字节:
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    //这些代码会打印出组成 String 的 18 个字节

    //从字符串中获取字形簇是很复杂的，所以标准库并没有提供这个功能。crates.io 上有些提供这样功能的 crate。

    //Rust 选择了以准确的方式处理 String 数据作为所有 Rust 程序的默认行为，这意味着程序员们必须更多的思考如何预先处理 UTF-8 数据。
    //这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性，不过也使你在开发生命周期后期免于处理涉及非 ASCII 字符的错误。
}
