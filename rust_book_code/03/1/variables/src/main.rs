fn main() {
    //immutable and mutable variables
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is {}", x);

    //const
    const MAX_POINTS: u32 = 100_000;
    println!("the value of MAX_POINTS is {}", MAX_POINTS);

    //Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is:{}", y);

    //当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字。例如，假设程序请求用户输入空格字符来说明希望在文本之间显示多少个空格，然而我们真正需要的是将输入存储成数字（多少个空格）：
    let spaces = "    ";
    let spaces = spaces.len(); //如果是spaces = spaces.len();则会报错
    println!("the length of first spaces:{}", spaces);

    let mut spaces = "    ";
    //spaces = spaces.len();//报错，可变变量类型不可变
}
