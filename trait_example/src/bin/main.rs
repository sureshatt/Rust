use trait_example::{Display, NewsArticle, Summary, Tweet, Post};

fn main() {
    println!("jello");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    print_news(&tweet);
    print_news(&article);

    // trait bounds
    print_news2(&article);

    // trait bounds with where
    display(&tweet);

    // return a trait type
    print_news(&returns_summarizable_news_article());
    print_news(&returns_summarizable_tweet());

    // conditional method implementation
    let post = Post {
        x: tweet,
    };

    post.display_summary();
    
}

// this method can only be called by a type that has the Summary trait
fn print_news(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bounds
fn print_news2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// trait bounds with where
fn display<T>(item: &T)
where
    T: Display + Summary,
{
    // note '+' for multiple traits
    println!("Breaking news! {}", item.display());
}

// this function will retun a 'type' that implements the 'summary' trait
fn returns_summarizable_news_article() -> impl Summary {
    // note: -- you cannot return either NewsArticle or Tweet inside, it always has to be just ONE type. 
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    }
}

// above returns_summarizable_news_article() cannot return Tweet or NewsArticle. Therefore this function only returns Tweet.
fn returns_summarizable_tweet() -> impl Summary { 
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
