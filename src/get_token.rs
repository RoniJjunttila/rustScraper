use reqwest::Client;
use anyhow::Result;
use scraper::{Html, Selector};
use serde_json::Value;

pub async fn get_token(id: String) -> Result<String> {

    let client = Client::new();

    let f_url = format!("https://areena.yle.fi/tv/ohjelmat/{}", id);

    let html = client
        .get(f_url)
        .header("User-Agent","Mozilla/5.0")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);


    let selector = Selector::parse(r#"script#__NEXT_DATA__"#).unwrap();

    let json_text = document
        .select(&selector)
        .next()
        .unwrap()
        .inner_html();

    let value: Value = serde_json::from_str(&json_text)?;

    let url = value["props"]["pageProps"]["view"]["tabs"][0]
        ["content"][0]["source"]["uri"]
        .as_str()
        .unwrap();

    let token = url.split("token=").nth(1).unwrap();

    //println!("{}", token);

    Ok(token.to_string())
}
