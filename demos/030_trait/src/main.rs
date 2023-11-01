use trait_example::{NewsArticle, Summary, Tweet}; // trait_example 是 Cargo.toml 中的 package name

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanly Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best\
        hockey team in the NHL.",
        ),
    };

    println!("new article available. {}", article.summarize());
}
