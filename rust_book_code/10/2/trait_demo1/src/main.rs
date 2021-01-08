mod lib;

fn main() {
    //use crate::lib::*;
    use crate::lib::Summary;
    use crate::lib::Tweet;

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course,as you probably already know,people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
