fn main() {
    using_if_let();
    using_while_let();
    using_for_loops();
    using_let_statement();

    let point = (3,5);
    using_function_parameters(&point);
}


///如果用户指定了中意的颜色，将使用其作为背景颜色。
/// 如果今天是星期二，背景颜色将是绿色。
/// 如果用户指定了他们的年龄字符串并能够成功将其解析为数字的话，我们将根据这个数字使用紫色或者橙色。
/// 最后，如果没有一个条件符合，背景颜色将是蓝色：
/// 这个条件结构允许我们支持复杂的需求。
/// 使用这里硬编码的值，例子会打印出 Using purple as the background color。
fn using_if_let() {
    println!("now the follows is using_if_let function:");

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn using_while_let() {
    println!("now the follows is using_while_let function:");

    //一个与 if let 结构类似的是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    //使用 vector 作为栈并以先进后出的方式打印出 vector 中的值
    //pop 方法取出 vector 的最后一个元素并返回 Some(value)。如果 vector 是空的，它返回 None
    //while 循环只要 pop 返回 Some 就会一直运行其块中的代码
    while let Some(top) = stack.pop() {
        println!("{}",top);
    }
}

fn using_for_loops() {
    println!("now the follows is using_for_loops function:");

    //for 可以获取一个模式。在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x。
    let v = vec!['a','b','c'];
    for (index,value) in v.iter().enumerate() {//enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引，他们位于一个元组中
        println!("{} is at index {}",value,index);
    }
}

fn using_let_statement() {
    println!("now the follows is using_let_statement function:");

    //例如 let x = 5; 的情况，x 是一个模式代表 “将匹配到的值绑定到变量 x”。同时因为名称 x 是整个模式，
    //这个模式实际上等于 “将任何值绑定到变量 x，不管值是什么”。
    let (x,y,z) = (1,2,3);
    println!("the three numbers in tuples is {} {} {}",x,y,z);
}

//函数参数实际上也是模式,main函数中调用可以看出，值 &(3, 5) 会匹配模式 &(x, y)，如此 x 得到了值 3，而 y得到了值 5
fn using_function_parameters(&(x,y):&(i32,i32)) {
    println!("now the follows is using_function_parameters function:");

    println!("Current location: ({},{})",x,y);
}

