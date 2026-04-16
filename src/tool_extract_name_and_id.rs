use std::{io};
use reqwest::Client;
use anyhow::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {

    let client = Client::new();

    loop {
        let mut input = String::new();


        io::stdin()
            .read_line(&mut input)
            .expect("input");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let html = client
            .get(input)
            .header("User-Agent","Mozilla/5.0")
            .send()
            .await?
            .text()
            .await?;

        let document = Html::parse_document(&html);

        let selector = Selector::parse("h1[class^='PackageHeader_title']").unwrap();

        let header_text = document
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();

        let id = input.split("ohjelmat/").nth(1).unwrap();

        println!("({:?}, {:?})", header_text, id);
    }

    Ok(())
}
