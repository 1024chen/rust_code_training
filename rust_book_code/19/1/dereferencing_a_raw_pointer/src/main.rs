fn main() {
    let mut num = 5;

    //可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;//若通过可变指针修改数据，则可能潜在造成数据竞

    //创建一个指向任意内存地址的裸指针。尝试使用任意内存是未定义行为：此地址可能有数据也可能没有，
    //编译器可能会优化掉这个内存访问，或者程序可能会出现段错误（segmentation fault）
    //let address = 0x012345usize;
    //let r = address as *const i32;

    //可以在safe 代码中创建裸指针，不过不能解引用裸指针和读取其指向的数据
    //unsafe块中对裸指针使用解引用运算符*
    unsafe {
        println!("r1 is: {}",*r1);
        println!("r2 is: {}",*r2);
    }
}
