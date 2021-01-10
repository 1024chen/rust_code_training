///修改 main 来重新赋值 post 使得这个实现不再完全遵守面向对象的状态模式：状态间的转换不再完全封装在 Post 实现中。
/// 然而，得益于类型系统和编译时类型检查，我们得到了的是无效状态是不可能的！这确保了某些特定的 bug
/// 比如显示未发布博文的内容，将在部署到生产环境之前被发现。
/// 在 Rust 中面向对象模式并不总是最好的解决方案，因为 Rust 拥有像所有权这样的面向对象语言所没有的功能
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
