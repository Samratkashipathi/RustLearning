use std::fmt;

trait Summary {
    fn summarize(&self) -> String;

    fn default_author(&self) -> String {
        String::from("Samrat")
    }

    fn production_house() -> String {
        String::from("Named under Samrat")
    }
}


struct News {
    article: String,
    author: String,
    date: String,
    likes: i32,
}

impl News {
    fn create_news(article: String, author: String, date: String, likes: i32) -> News {
        News { article: article, author: author, date: date, likes: likes }
    }
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} was written by {} on date {} has {} likes", self.article, self.author, self.date,
                self.likes)
    }
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        String::from("1")
    }
}


fn print_articles<T: Summary>(articles: &[T]) {
    for article in articles {
        let msg = article.summarize();
        println!("{}", msg)
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.owner, self.likes, self.date)
    }
}

trait TweetMsg {
    fn make_tweets(&self) -> Tweet;
}

struct Tweet {
    owner: String,
    likes: i32,
    date: String,
}

impl TweetMsg for Tweet {
    fn make_tweets(&self) -> Tweet {
        Tweet { owner: format!("{}!!", self.owner), likes: self.likes, date: format!("{}!!", self.date) }
    }
}

// Two ways of specifying T

//fn print_tweets<T: TweetMsg + fmt::Display>(tweets: &[T]) {
//    for tweet in tweets {
//        println!("Tweet: {}", tweet.make_tweets())
//    }
//}

fn print_tweets<T>(tweets: &[T]) where T: TweetMsg + fmt::Display {
    for tweet in tweets {
        println!("Tweet: {}", tweet.make_tweets())
    }
}

// Traits as an argument
fn notify(item: impl Summary) {
    println!("Notification!! {}", item.summarize());
}


fn main() {
    println!("Hello world!!");

    let news1 = News::create_news(String::from("Claire underwood becomes new US president"),
                                  String::from("Samrat K S"),
                                  String::from("11-10-2018"),
                                  43);

    println!("{}", news1.summarize());

    let v = 2;

    println!("{}", v.summarize());

    println!("{}", news1.default_author());
    println!("{}", v.default_author());


    println!("{}", News::production_house());
    println!("{}", i32::production_house());

    let tweet_msg = Tweet { owner: String::from("Samrat"), likes: 100, date: String::from("Today") };
    let tweet1 = tweet_msg.make_tweets();
    let tweets = vec![tweet1];
    print_tweets(&tweets);


    // Traits as an argument
    notify(v);

    // This fails as Summary is not imple as trai for f32
    // let f = 44.44;
    // notify(f);
}