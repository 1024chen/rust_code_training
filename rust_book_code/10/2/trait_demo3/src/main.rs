mod lib;
use crate::lib::notify;
use crate::lib::notify_1;
use crate::lib::returns_summarizable;
use crate::lib::Summary;
use crate::lib::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
    notify_1(&tweet);

    notify(&returns_summarizable());
}
