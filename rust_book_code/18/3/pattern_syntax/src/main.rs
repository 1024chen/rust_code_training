fn matching_literals_function() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables_function() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched,y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges_of_values_with_double_point_and_equal() {
    let x = 5;

    //范围只允许用于数字或 char 值，因为编译器会在编译时检查范围不为空。char 和 数字值是 Rust 仅有的可以判断范围是否为空的类型
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn destructuring_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    //创建了变量 x 和 y 来匹配结构体 p 中的 x 和 y 字段
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

#[allow(unused)]
fn destructuring_enums() {
    enum Message {
        Quit, //对于像 Message::Quit 这样没有任何数据的枚举成员，不能进一步解构其值。只能匹配其字面值 Message::Quit，因此模式中没有任何变量。
        Move { x: i32, y: i32 }, //对于像 Message::Move 这样的类结构体枚举成员，可以采用类似于匹配结构体的模式
        Write(String), //其模式则类似于用于解构元组的模式。模式中变量的数量必须与成员中元素的数量一致。
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

#[allow(unused)]
fn destructuring_nested_structs_and_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
}

fn destructuring_structs_adn_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!(
        "the values is feet: {}  inches: {} x: {} y: {}",
        feet, inches, x, y
    );
}

//大部分情况当你不再需要特定函数参数时，最好修改签名不再包含无用的参数。
//在一些情况下忽略函数参数会变得特别有用，比如实现 trait 时，当你需要特定类型签名但是函数实现并不需要某个参数时。
//此时编译器就不会警告说存在未使用的函数参数，就跟使用命名参数一样。
fn ignoring_an_entire_values_with_() {
    fn foo(_: i32, y: i32) {
        //这段代码会完全忽略作为第一个参数传递的值 3，并会打印出内容
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);
}

#[allow(unused)]
fn ignoring_parts_of_a_value_with_a_nested_() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        //这段代码会打印出 Can't overwrite an existing customized value
        //接着是 setting is Some(5)。
        //在第一个匹配分支，我们不需要匹配或使用任一个 Some 成员中的值；重要的部分是需要测试 setting_value 和 new_setting_value 都为 Some 成员的情况。
        //在这种情况，我们打印出为何不改变 setting_value，并且不会改变它。
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

//有时创建一个还未使用的变量是有用的，比如正在设计原型或刚刚开始一个项目。
//这时希望告诉 Rust 不要警告未使用的变量，为此可以用下划线作为变量名的开头
fn ignoring_an_unused_variable_by_starting_its_ame_with_() {
    let _x = 5; //不警告
    let y = 10; //“未使用”警告

    // let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s);//报错，因为s的值所有权会转移到_s

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s); //不报错，因为之使用_，不会绑定值,所有权也并未发生移动
}

#[allow(unused)]
fn ignoring_remaining_parts_of_a_value_with_double_point() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // let numbers = (2, 4, 8, 16, 32);

    // match numbers {
    //     (.., second, ..) => {//`..` can only be used once per tuple pattern
    //         println!("Some numbers: {}", second)
    //     },
    // }
}
//匹配守卫是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。
//匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。
fn extra_conditionals_with_match_guards() {
    let num = Some(4);

    match num {
        //第一个分支有模式 Some(x) 还有匹配守卫 if x < 5
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        //匹配守卫 if n == y 并不是一个模式所以没有引入新变量
        Some(n) if n == y => println!("Matched, n = {}", n), //第二个匹配分支中的模式不会引入一个覆盖外部 y 的新变量 y，这意味着可以在匹配守卫中使用外部的 y
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;
    match x {
        //这个匹配条件表明此分支值匹配 x 值为 4、5 或 6 同时 y 为 true 的情况
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

//使用 @ 可以在一个模式中同时测试和保存变量值。
fn at_binging() {
    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }


}

fn main() {
    matching_literals_function();
    matching_named_variables_function();
    multiple_patterns();
    matching_ranges_of_values_with_double_point_and_equal();
    destructuring_struct();
    destructuring_enums();
    destructuring_nested_structs_and_enums();
    destructuring_structs_adn_tuples();
    ignoring_an_entire_values_with_();
    ignoring_parts_of_a_value_with_a_nested_();
    ignoring_an_unused_variable_by_starting_its_ame_with_();
    ignoring_remaining_parts_of_a_value_with_double_point();
    extra_conditionals_with_match_guards();
    at_binging();
}
