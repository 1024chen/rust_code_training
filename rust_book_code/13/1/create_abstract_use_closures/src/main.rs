use std::thread;
use std::time::Duration;
//希望能够在程序的一个位置指定某些代码，并只在程序的某处实际需要结果的时候 执行 这些代码,这正是闭包的用武之地

//定义一个 Cacher 结构体来在 calculation 中存放闭包并在 value 中存放 Option 值
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    //T 的 trait bound 指定了 T 是一个使用 Fn 的闭包。任何我们希望储存到 Cacher 实例的 calculation 字段的闭包必须有一个 u32 参数（由 Fn 之后的括号的内容指定）并必须返回一个 u32（由 -> 之后的内容）。
    calculation: T,
    value: Option(u32),
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    //使用闭包的原因是我们需要在一个位置定义代码，储存代码，并在之后的位置实际调用它；期望调用的代码现在储存在 expensive_result 中。
    let expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        print!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result(intensity));
        }
    }
}

fn main() {
    //对比
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    //一个储存在 equal_to_x 变量中闭包的例子，它使用了闭包环境中的变量 x：
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
