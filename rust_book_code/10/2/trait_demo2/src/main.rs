mod lib;

fn main() {
    //use crate::lib::*;
    use crate::lib::NewArticle;
    use crate::lib::Summary;
    use crate::lib::Tweet;

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course,as you probably already know,people"),
        reply: false,
        retweet: false,
    };

    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());
}
