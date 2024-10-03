pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Display {
    fn display(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // ---- forcing the default implementation by commenting out bellow
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("content: {}", self.content)
    }
}

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

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn display(&self) -> String {
        format!("content: {}", self.content)
    }
}

// for the demo for conditionally implemented methods
pub struct Post<T> {
    pub x: T
}


// conditional method implementation
impl <T: Display + Summary> Post<T>{
        pub fn display_summary(&self) {
            println!("display:{} + summary: {}", self.x.display(), self.x.summarize())
        }
}
