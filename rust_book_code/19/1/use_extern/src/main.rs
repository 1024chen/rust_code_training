///extern 的使用无需 unsafe

fn using_extern_functions_to_call_external_clode() {
    extern "C" {
        //集成 C 标准库中的 abs 函数
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

//访问和修改可变静态变量都是unsafe的
static mut COUNTER: u32 = 0; //多个线程访问 COUNTER 则可能导致数据竞争。
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//一个拥有字符串 slice 值的静态变量
//静态变量只能储存拥有 'static 生命周期的引用，
//这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注
static HELLO_WORLD: &str = "hello,world!";
#[allow(unused)]
fn main() {
    using_extern_functions_to_call_external_clode();
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    //联合体定义及使用
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    fn f(u: MyUnion) {
        unsafe {
            match u {
                MyUnion { f1: 10 } => {
                    println!("ten");
                }
                MyUnion { f2 } => {
                    println!("{}", f2);
                }
            }
        }
    }

    let mut u = MyUnion { f1: 1 };

    u.f1 = 2;
    unsafe {
        println!("f1 is {}", u.f1);
    }
}

//当至少有一个方法中包含编译器不能验证的不变量时 trait 是unsafe的。
//可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，
//同时 trait 的实现也必须标记为 unsafe
unsafe trait Foo {}

unsafe impl Foo for i32 {} //通过 unsafe impl，承诺将保证编译器所不能验证的不变量
