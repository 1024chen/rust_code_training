fn main() {
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);
            f64::sqrt(x_squared + y_squared)
        }
    }

    let p1 = Point { x: 0.2, y: 0.2 };
    let p2 = Point { x: 5.2, y: 6.7 };
    /*
    Rust 并没有一个与 -> 等效的运算符；
    相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
    方法调用是 Rust 中少数几个拥有这种行为的地方。
    他是这样工作的：当使用 object.something() 调用方法时，
    Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
    也就是说，这些代码是等价的：
    */
    println!(" the distance of p1 and p2 is {}", p1.distance(&p2));
    println!(" the distance of p1 and p2 is {}", (&p1).distance(&p2));
}
