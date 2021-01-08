fn main() {
    let mut s: String = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{},{}", r1, r2);

    //     这个限制允许可变性，不过是以一种受限制的方式允许。新 Rustacean 们经常与此作斗争，因为大部分语言中变量任何时候都是可变的。
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
    // 两个或更多指针同时访问同一数据。
    // 至少有一个指针被用来写入数据。
    // 没有同步数据访问的机制。

    // 数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

    //一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
    let mut s = String::from("hello");

    {
        //let r1 = &mut s;
        let r1 = &mut s;
        r1.push_str(", world");
        println!("r1:{}", r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    r2.push_str(", world");
    println!("r1:{}", r2);
    
    //不能在拥有不可变引用的同时拥有可变引用
    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    */
}
