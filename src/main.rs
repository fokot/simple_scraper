use std::collections::LinkedList;
use std::fs::File;
use scraper::{Html, Selector};
use std::io::Write;
use std::iter::Map;

fn main() {

    // get current time
    // let now = time::Instant::now();

    let pages = LinkedList::new();

    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();

    let domain = "https://books.toscrape.com";
    let root_page = "/";
    let root_url = concat!(domain,root_page);
    let root_file_name = "index.html";
    let response = client
        .get(root_url)
        .send().expect("Could not load url.");

    // // let response = reqwest::blocking::get(r).expect("Could not load url.");
    let body = response.text().unwrap();


    print!("{}",body);
    let mut file = File::create(root_file_name).expect("Could not create file.");
    file.write_all(body.as_bytes()).expect("Could not write to file.");

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();
    for element in document.select(&selector) {
        let href = element.value().attr("href").expect("Could not find href attribute.");
        println!("{}",href);
    }
}

