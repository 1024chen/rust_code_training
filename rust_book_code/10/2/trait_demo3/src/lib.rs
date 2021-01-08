pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Display {
    fn to_display(&self) -> String {
        String::from("a display method of struct")
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//因为实现了 summarize_author，Summary trait 就提供了 summarize 方法的功能，且无需编写更多的代码
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {}

//定义一个函数 notify 来调用其参数 item 上的 summarize 方法，
//该参数是实现了 Summary trait 的某种类型
//pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

//impl Trait 语法适用于短小的例子，它不过是一个较长形式的语法糖。
//trait bound 与泛型参数声明在一起，位于尖括号中分号的后面。
//因为 T 的 trait bound，我们可以传递任何 NewArticle 或 Tweet 的实例调用 notify
//用任何其他类型，比如 String 或 i32，调用该函数的代码将不能编译，因为这些类型没有实现 Summary。
pub fn notify<T: Summary>(item: &T) {
    println!("notify method: {}", item.summarize());
}

//假设notify在两个以上泛型参数，如果希望强制它们都是相同类型，这只有在使用 trait bound 时才有可能：
//pub fn notify<T: Summary>(item1: T, item2: T) {

// 通过 + 指定多个 trait
// 如果 notify_1 需要显示 item 的格式化形式，同时也要使用 summarize 方法，那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现：
//pub fn notify_1(item: &(impl Summary + Display)) {
// 这个语法也适用于泛型的 trait bound：
pub fn notify_1<T: Summary + Display>(item: &T) {
    println!("notify_1 method: {}", item.summarize());
    println!("{}", item.to_display());
}

//每个泛型有其自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，这使得函数签名难以阅读。
//为此，Rust 有另一个在函数签名之后的 where 从句中指定 trait bound 的语法。
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//还可以像这样使用 where 从句:
// fn some_function<T, U>(t: T, u: U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

//可以在返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型：
//这个签名表明，「我要返回某个实现了 Summary trait 的类型，但是不确定其具体的类型」。在例子中返回了一个 Tweet，不过调用方并不知情。
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("hen_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
