fn main() {
    let article = src::NewsArticle {
        headline: String::from("Rust reaches new heights"),
        location: String::from("Internet"),
        author: String::from("Jane Doe"),
        content: String::from("Rust has become the most popular programming language in the world."),
    };


    let post = src::SocialPost {
        username: String::from("rustacean"),
        content: String::from("I love programming in Rust!"),
        reply: false,
        repost: false,
    };

    println!("Article Summary: {}", article.summarize());
    println!("Post Summary: {}", post.summarize());
}