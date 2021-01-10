///对于状态模式来说，Post 的方法和使用 Post 的位置无需 match 语句，
/// 同时增加新状态只涉及到增加一个新 struct 和为其实现 trait 的方法。

//State trait 定义了所有不同状态的博文所共享的行为
trait State {
    //这里使用了 self: Box<Self>。这个语法意味着这个方法调用只对这个类型的 Box 有效
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    //这个语法获取了 Box<Self> 的所有权，使老状态无效化以便 Post 的状态值可以将自身转换为新状态

    fn approve(self: Box<Self>) -> Box<dyn State>;

    //获取 post 的引用作为参数，并返回 post 一部分的引用，所以返回的引用的生命周期与 post 参数相关
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}
//PendingReview状态
struct PendingReview {}
//Draft状态
struct Draft {}
//Published状态
struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {}) //返回一个新的，装箱的 PendingReview 结构体的实例，用来代表博文处于等待审核状态
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self //返回自身，因为请求审核已经处于 PendingReview 状态的博文应该保持 PendingReview 状态
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self //返回自身，因为请求审核已经处于 PendingReview 状态的博文应该保持 PendingReview 状态
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub struct Post {
    state: Option<Box<dyn State>>, //状态
    content: String,               //内容
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        //调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权
        self.state.as_ref().unwrap().content(self)
        //state 是一个 Option<Box<State>>，调用 as_ref 会返回一个 Option<&Box<State>>
    }

    //为 Post 增加一个获取 self 可变引用的公有方法 request_review
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            //调用 take 方法将 state 字段中的 Some 值取出并留下一个 None
            self.state = Some(s.request_review()) //第二个 request_review 方法会消费当前的状态并返回一个新状态
        }
        //不同于像 self.state = self.state.request_review(); 这样的代码直接设置 state 字段，来获取 state 值的所有权.确保了当 Post 被转换为新状态后其不再能使用老的 state 值。
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}
