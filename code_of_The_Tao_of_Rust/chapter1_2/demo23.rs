//代码清单2-32：neüer类型示例
//`#![feature]` may not be used on the stable release channel
#![feature(nuver_type)]
fn foo() -> i32 {
    let x: ! = { return 123 }; //
    //println!("{}", x);
}

fn main() {
    let num: Option<u32> = Some(42);
    match num {
        Some(num) => num,
        None => panic!("Nothing"),
    };
}
