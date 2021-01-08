fn main() {
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black = Color(1,2,3);
    let origin = Point(1,2,3);

    println!("black_first:{}",black.0);
    println!("origin_first:{}",origin.0);
}
