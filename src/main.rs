use reqwest::Error;
use reqwest::Error;
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "http://books.toscrape.com/catalogue/category/books/science_22/index.html";

    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    let fragment = Html::parse_document(&body);

    // Selector for the book urls
    let book_selector = Selector::parse(".product_pod a").unwrap();

    // Extract book urls
    for book in fragment.select(&book_selector) {
        let book_url = book.value().attr("href").unwrap();
        println!("Book url: {}", book_url);
    }

    Ok(())
}
