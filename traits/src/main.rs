use std::{fmt::format, iter::Sum};

trait Summary  {
    fn get_author(&self) -> &str;
    fn summarize(&self) -> String {
        format!("{}: Read more .......", self.get_author())
    } //default implementation
}

trait MyTrait {
    fn demo(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let content = format!("news by {}: {}", self.author, self.content);
        content
    }
    
    fn get_author(&self) -> &str {
        self.author.as_str()
    }

    
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = format!("Tweet by {}: {}", self.username, self.content);
        content
    }
    fn get_author(&self) -> &str {
        self.username.as_str()
    }
}

impl MyTrait for Tweet {
    fn demo(&self) -> String {
        todo!()
    }
}

impl MyTrait for NewsArticle
 {
    fn demo(&self) -> String {
        todo!()
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("punit2000"),
        content: String::from("India won the match"),
        reply: true,
        retweet: true
        
    };

    news_aggregator(&tweet);

    let news = NewsArticle {
        headline:  String::from("India won the match against australia by 5 wickets"),
        location: String::from("Australia"),
        author: String::from("Punit Savlesha"),
        content: String::from("India won from a losing position")

    };
    // let a = news.summarize();
    // println!("{a}");
    news_aggregator(&news);

    mix_news(&tweet,&news);

}


fn news_aggregator(source: &(impl Summary + MyTrait) ) {
    println!("there is a new news in the market");
    println!("{}", source.summarize());
}

fn get_news<T: Summary>(source: &impl Summary) {
    println!("{}", source.summarize());
}

// fn mix_news<T: Summary, U: Summary>(primary: &T, other: &U) {
//     println!("{} and {}", primary.summarize(),other.summarize());
// }

//or 

fn mix_news(primary: &impl Summary, other: &impl Summary) {
    println!("{} and {}", primary.summarize(), other.summarize())
}