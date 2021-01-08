fn main() {
    //string to u32 将字符串转换成数字
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess:{}",guess);
    //scalar标量类型
    //整型i8,i16,i32,i64,isize
    //u8,u16,u32,u64,usize
    //isize和usize类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。

    //整型字面量
    //Decimal 98_222
    //Hex 0xff
    //Octal 0o77
    //Binary 0b1111_0000
    //Byte(u8 only) b'A'

    //浮点型floating-point numbers
    let x = 2.0;
    print!("x:{}",x);
    let y: f32 = 3.0;
    //浮点遵循IEEE-754
    print!("y:{}",y);

    //caculate
    // 加法
    let sum = 5 + 10;
    println!("sum:{}",sum);
    // 减法
    let difference = 95.5 - 4.3;
    println!("difference:{}",difference);
    // 乘法
    let product = 4 * 30;
    println!("produt:{}",product);
    // 除法
    let quotient = 56.7 / 32.2;
    println!("quotient:{}",quotient);
    // 取余
    let remainder = 43 % 5;
    println!("remainder:{}",remainder);

    //布尔型bool
    let _t = true;
    let f: bool = false; // 显式指定类型注解
    println!("f:{}", f);

    //字符类型char，char 类型代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat: char = '😻';
    println!("c:{}", c);
    println!("z:{}", z);
    println!("heart_eyed_cat:{}", heart_eyed_cat);

    //复合类型Compound types，可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
    //元组类型:将多个其他类型的值组合进一个复合类型的主要方式,使用包含在圆括号中的逗号分隔的值列表来创建一个元组
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    //使用模式匹配（pattern matching）来解构（destructure）元组值
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x,y,z is: {},{},{}", x,y,z);
    //除了使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问它们。
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "The value of 0 is: {}, 1 is: {}, 2 is: {}",
        five_hundred, six_point_four, one
    );

    //数组类型array，数组中的每个元素的类型必须相同，数组是固定长度的
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    //指定类型与长度才不会报警告
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "The value of element is: {} {} {} {} {}",
        a[0], a[1], a[2], a[3], a[4]
    );
}
