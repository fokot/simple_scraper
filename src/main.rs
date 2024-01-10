use scraper::{Html, Selector};




fn main() {
    let url = "https://books.toscrape.com/";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let body = response.text().unwrap();
    print!("{}",body);

    let document = Html::parse_document(&body);

    let book_selector = Selector::parse("article.product_pod").unwrap();
    let book_name_selector = Selector::parse("h3 a").unwrap();
    let book_price_selector = Selector::parse(".price_color").unwrap();
    let book_link_selector = Selector::parse("h3 a").unwrap();
    for element in document.select(&book_selector) {
        let book_name_element = element.select(&book_name_selector).next().expect("Could not select book name.");
        let book_name = book_name_element.value().attr("title").expect("Could not find title attribute.");
        let price_element = element.select(&book_price_selector).next().expect("Could not find price");
        let price = price_element.text().collect::<String>();
        println!("{:?} - {:?}",book_name, price);

        let book_link_element = element.select(&book_name_selector).next().expect("Could not find book link element.");
        let book_link= book_link_element.value().attr("href").expect("Could not find href attribute");
    }
}

