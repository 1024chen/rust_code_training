/// 重点中的重点：
/// 只有对象安全（object safe）的 trait 才可以组成 trait 对象
/// 如果一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的：
/// 1.返回值类型不为 Self（一旦有了 trait 对象，就不再知晓实现该 trait 的具体类型是什么了。如果 trait 方法返回具体的 Self 类型，但是 trait 对象忘记了其真正的类型，那么方法不可能使用已经忘却的原始具体类型）
/// 2.方法没有任何泛型类型参数（当使用 trait 对象时其具体类型被抹去了，故无从得知放入泛型参数类型的类型是什么）
/// 当使用 trait 对象时其具体类型被抹去了，故无从得知放入泛型参数类型的类型是什么。
pub mod gui {
    pub trait Draw {
        fn draw(&self);
    }

    //这个 vector 的类型是 Box<dyn Draw>，
    //此为一个 trait 对象：它是 Box 中任何实现了 Draw trait 的类型的替身
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
    //通过使用 trait 对象的方法，
    //一个 Screen 实例可以存放一个既能包含 Box<Button>，也能包含 Box<TextField> 的 Vec<T>

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    //在 Screen 结构体上，定义一个 run 方法，该方法会对其 components 上的每一个组件调用 draw 方法
    //一种 Screen 结构体的替代实现，其 run 方法使用泛型和 trait bound
    // pub struct Screen<T: Draw> {
    //     pub components: Vec<T>,
    // }
    //
    // impl<T> Screen<T>
    // where
    //     T: Draw,
    // {
    //     pub fn run(&self) {
    //         for component in self.components.iter() {
    //             component.draw();
    //         }
    //     }
    // }
    //缺点是Vec<T>类型必须相同，T不能既是Box<Button> 又是Box<TextFiled>,既只能存放相同类型

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("Draw Button,area is {}", self.width * self.height);
        }
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl SelectBox {
        //写的无关代码
        pub fn new(width: u32, height: u32, options: &str) -> SelectBox {
            let mut option = Vec::new();
            option.push(String::from(options));
            SelectBox {
                width,
                height,
                options: option,
            }
        }
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("Draw SelectBox,area is {}", self.width * self.height);
        }
    }
}

fn main() {
    use gui::{Button, Screen, SelectBox};

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox::new(10, 20, "No")),
        ],
    };

    screen.run();
}
