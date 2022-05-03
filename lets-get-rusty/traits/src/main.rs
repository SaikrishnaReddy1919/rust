fn main() {
    pub struct Article {
        author : String,
        date : String,
        content : String
    }

    pub struct Tweet {
        user : String,
        date : String,
        reply : bool
    }

    impl Summary for Article{
        //own implemenation
        fn summarize(&self) -> String{
            format!("{}: {}", &self.author, &self.content)
        }
    }

    impl Summary for Tweet{
        //own implemenation
        fn summarize(&self) -> String{
            format!("{}: {}", &self.user, &self.date)
        }

        fn default_implementation(&self) -> String {
            "overriding default impl".to_string()
        }
    }


    let article = Article {
        author : "sai".to_string(),
        content : "hack".to_string(),
        date : "20".to_string()
    };

    println!("Article is : {}", article.summarize());
    println!("Default check : {}", article.default_implementation());

    let tweet = Tweet {
        user : "sai-tweet".to_string(),
        reply : false,
        date : "20".to_string()
    };

    println!("Tweet is : {}", tweet.summarize());
    //tweet overrided default impl method
    println!("Default check : {}", tweet.default_implementation());
}


pub trait Summary {
    // withoud default implementation
    fn summarize(&self) -> String;

    fn default_implementation(&self) -> String {
        "hello".to_string()
    }
}
