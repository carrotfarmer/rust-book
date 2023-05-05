use core::fmt::Debug;
use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        return format!("(Read more from {}...)", self.summarize_author());
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course as you probably already know"),
        reply: false,
        retweet: false,
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {}

fn some_func<T, U>(t: &T, u: &U) -> () 
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        content: String::from("hello, world!"),
        username: String::from("@carrotfarmer17"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        content: String::from("Russia deleting ukraine ono"),
        headline: String::from("RUSSIA IS BAD"),
        author: String::from("boe jiden"),
    };

    println!("{}", tweet.summarize());
    println!("{}", article.summarize());

    notify(&article);

    println!("{}", returns_summarizable().summarize());
}
