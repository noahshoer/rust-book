pub mod aggregator {
    // Traits are similar to interfaces. Putting the semicolon
    // requires any type using this trait to implement it.
    // It can also have a default implementation, and you use an empty impl block
    // Can also call other trait methods
    pub trait Summary {
        fn summarize(&self) -> String;
        fn read(&self) -> String {
            self.summarize() + "(Read more...)"
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({}", self.headline, self.author, self.location)
        }
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}

