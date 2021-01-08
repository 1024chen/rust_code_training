#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /*
    方法 与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
    不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文），
    并且它们第一个参数总是 self，它代表调用该方法的结构体实例。
    */
    fn area(&self) -> u32 {
        self.width * self.height
    }
    /*
    这里选择 &self 的理由跟在函数版本中使用 &Rectangle 是相同的：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。
    如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self
    */
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area is:{}", rect1.area());
}
