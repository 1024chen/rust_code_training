fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1: User = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2:User = User{
        email:String::from("another@example.com"),
        username:String::from("anotherusername567"),
        active:user1.active,
        sign_in_count:user1.sign_in_count,
    };

    //..语法的使用,不加,
    let user3:User = User{
        email:String::from("another@example.com"),
        username:String::from("anotherusername789"),
        ..user1
    };

    println!("user1.active:{}",user1.active);
    println!("user2.active:{}",user2.active);
    println!("user3.active:{}",user3.active);
}
