use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    author: String,
    location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

struct SocialPost {
    username: String,
    content: String,
}

trait DefaultSummary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")   // it is called default implimentation
    }
}

impl DefaultSummary for NewsArticle {}

impl DefaultSummary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

trait AuthorSummary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl AuthorSummary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

fn notify_with_bound<T: Summary>(item: &T) {
    println!("Notification: {}", item.summarize());
}

fn notify_two_different(
    item1: &impl Summary,
    item2: &impl Summary,
) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

fn notify_two_same<T: Summary>(
    item1: &T,
    item2: &T,
) {
    println!("Item 1: {}", item1.summarize());
    println!("Item 2: {}", item2.summarize());
}

fn print_and_summarize<T: Summary + Display>(item: &T) {
    println!("Display: {}", item);
    println!("Summary: {}", item.summarize());
}

fn print_and_summarize_where<T, U>(item1: &T, item2: &U)
where
    T: Summary + Display,
    U: Summary + Display,
{
    println!("Item 1: {}", item1);
    println!("Item 1 summary: {}", item1.summarize());

    println!("Item 2: {}", item2);
    println!("Item 2 summary: {}", item2.summarize());
}

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("grah4na"),
        content: String::from("learning rust traits"),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest member is x = {}", self.x);
        } else {
            println!("Largest member is y = {}", self.y);
        }
    }
}

trait Printable {
    fn print(&self);
}

impl<T: Display> Printable for T {
    fn print(&self) {
        println!("Printable: {}", self);
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust is learning"),
        author: String::from("Alice"),
        location: String::from("India"),
    };

    let post = SocialPost {
        username: String::from("grah4na"),
        content: String::from("learning rust traits"),
    };

    println!("{}", article.summarize());

    println!("{}", article.summarize());

    println!("{}", post.summarize());

    println!("{}", post.summarize());

    notify(&article);

    notify_with_bound(&article);

    notify_two_different(&article, &post);

    notify_two_same(&article, &article);

    let result = returns_summarizable();
    println!("{}", result.summarize());

    let numbers = Pair::new(10, 20);
    numbers.cmp_display();

    let number = 42;
    number.print();

    let text = String::from("hello");
    text.print();

    let number_string = 3.to_string();
    println!("{}", number_string);
}