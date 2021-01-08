#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    //不可变对象
    let user1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1.email:{}", user1.email);

    //可变对象
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("mut user1.email:{}", user1.email);

    //函数创建
    let ema = String::from("123@163.com");
    let nam = String::from("123");
    let user2: User = build_user(ema, nam);
    println!("user2.email:{}", user2.email);

    let ema = String::from("123@163.com");
    let nam = String::from("123");
    let user3: User = build_user1(ema, nam);
    println!("user3.email:{}", user3.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//字段初始化简写语法重写build函数
fn build_user1(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
