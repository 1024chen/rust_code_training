/**
 * 如果这个 lib.rs 是对应 aggregator crate 的，而别人想要利用我们 crate 的功能为其自己的库作用域中的结构体实现 Summary trait。
 * 首先他们需要将 trait 引入作用域。
 * 这可以通过指定 use aggregator::Summary; 实现，这样就可以为其类型实现 Summary trait 了。
 * Summary 还必须是公有 trait 使得其他 crate 可以实现它
 */

//单词释义 => trait: 特征，突出的特点
//Summary trait 定义，它包含由 summarize 方法提供的行为
//这里使用 trait 关键字来声明一个 trait，后面是 trait 的名字，在这个例子中是 Summary
pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
//不能为外部类型实现外部 trait。
//例如，不能在多媒体聚合库 crate 中为 Vec 实现 Display trait。
//这是因为 Display 和 Vec 都定义于标准库中，它们并不位于多媒体聚合库的 crate 本地作用域中。
//这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。
//这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。
//没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。
