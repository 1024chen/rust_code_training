//for...in 迭代器
fn main() {
    //1..101是一个Range类型，也是一个迭代器
    //for每次循环都从迭代器中取值，迭代器中没有值的时候，for循环结束
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
       }else {
           println!("{}", n);
       }
    }
}