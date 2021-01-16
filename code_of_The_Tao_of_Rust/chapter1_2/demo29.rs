//代码清单2-39：无参数枚举体示例
enum Number {
    _Zero,
    One,
    _Two,
}

//代码清单2-40：类C枚举体示例
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

//代码清单2-41：带参数枚举体示例
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//代码清单2-42 枚举体应用
enum Option {
    Some(i32),
    None,
}

impl Option {
    #[inline]
    #[track_caller]
    pub const fn unwrap(&self) -> i32 {
        match self {
            Option::Some(val) => *val,
            Option::None => -1,
        }
    }
}

//代码清单2-43：Option＜T＞示例
fn list2_44() {
    let s = &Some("hello".to_string());
    match s {
        Some(s) => println!("s is: {}",s),
        _ => (),
    }
}

fn main() {
    //39
    let a = Number::One;
    match a {
        Number::_Zero => println!("0"),
        Number::One => println!("1"),
        Number::_Two => println!("2"),
    }

    //40
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("leaf are #{:06x}", Color::Green as i32);

    //41
    let x = IpAddr::V4;
    let y = IpAddr::V6;
    let home = IpAddr::V4(127, 0, 0, 1);
    let a = x(127, 0, 0, 2);
    let b = y(String::from("127,0,0,3"));
    match home {
        IpAddr::V4(_, _, _, _) => println!("home V4"),
        IpAddr::V6(_) => println!("home V6"),
    }
    match a {
        IpAddr::V4(_, _, _, m) => println!("{}", m),
        IpAddr::V6(_) => println!("home V6"),
    }
    match b {
        IpAddr::V4(_, _, _, m) => println!("{}", m),
        IpAddr::V6(n) => println!("{}", n),
    }

    //42
    let s = crate::Option::Some(42);
    let num = s.unwrap();
    match s {
        crate::Option::Some(n) => println!("num is {}",n),
        crate::Option::None => (),
    }
    println!("num:{}",num);

    list2_44();
}
