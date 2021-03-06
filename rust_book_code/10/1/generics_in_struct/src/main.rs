/**
   struct Point<T, U> {
       x: T,
       y: U,
   }

   fn main() {
       let both_integer = Point { x: 5, y: 10 };
       let both_float = Point { x: 1.0, y: 4.0 };
       let integer_and_float = Point { x: 5, y: 4.0 };
   }
*/

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer:{} and {}",integer.x,integer.y);
    println!("float:{} and {}",float.x,float.y);
}

//以下代码报错：
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let wont_work = Point { x: 5, y: 4.0 };
// }
