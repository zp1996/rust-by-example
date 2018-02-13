// trait === interface
pub trait Summaizable {
    fn summery(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summaizable for NewArticle {
    fn summery(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summaizable for Tweet {
    fn summery(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify<T>(item: T) where T: Summaizable {
    println!("{:?}", item.summery());
}

fn main() {
    let article = NewArticle {
        headline: String::from("A rust demo"),
        location: String::from("tianjin"),
        author: String::from("zp1996"),
        content: String::from("rust and fe is good")
    };
    notify(article);

    let tweet = Tweet {
        username: String::from("lyt"),
        content: String::from("really cool"),
        reply: false,
        retweet: false
    };
    notify(tweet);
}
