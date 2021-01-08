fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;
    println!(
        "the area of the rectangle0 is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "the area of the rectangle1 is {} square pixels.",
        area1(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle2 is {} square pixels.",
        area2(&rect1)
    );

    //
    let rect1 = Rectangle {
        width: 32,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

//原始版本
fn area(width: u32, height: u32) -> u32 {
    width * height
}

//使用元组进行重构
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//结构体版本
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
//函数 area 现在被定义为接收一个名叫 re 的参数，
//其类型是一个结构体 Rectangle 实例的不可变借用
fn area2(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
