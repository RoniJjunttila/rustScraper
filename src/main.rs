use reqwest::{Client, Url};
use serde::Deserialize;
use rusqlite::{Connection, Result};

use crate::get_token::get_token;

use yle::categories::{
    RIKOS,
    NUORET,
    DRAAMASARJAT,
    REALITY,
    KOMEDIASARJAT,
    SEKALAISET,
    DOKUMENTIT,
    LUONTO,
    HISTORIA,
};

mod get_token;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<ApiItem>
}

#[derive(Debug, Deserialize)]
struct ApiItem {
    title: String,
    description: String,
    pointer: ApiPointer,
    image: ApiImage,
    labels: Vec<ApiRaw>,
}

#[derive(Debug, Deserialize)]
struct ApiRaw {
    raw: Option<String>
}

/* #[derive(Debug, Deserialize)]
struct ApiPointer {
    #[serde(rename = "Class")]
    class: String,
}
 */

#[derive(Debug, Deserialize)]
struct ApiPointer {
    #[serde(rename = "type")]
    r#type: String,
    _uri: String,
}
 
#[derive(Debug, Deserialize)]
struct ApiImage {
    id: String
}

#[allow(dead_code)]
#[derive(Debug)]
struct OutputData {
    title: String,
    description: String,
    uri_pointer: String,
    image: String,
    class: String
}

struct Category {
    name: String,
    token: Option<String>,
}


async fn make_url(token: &str) -> Url {
  let mut url = Url::parse("https://areena.api.yle.fi/v1/ui/content/list").unwrap();

    url.query_pairs_mut()
        .append_pair("client", "yle-areena-web")
        .append_pair("language", "fi")
        .append_pair("v", "10")
        .append_pair("token", token)
        .append_pair("offset", "0")
        .append_pair("limit", "16")
        .append_pair("country", "FI")
        .append_pair("isPortabilityRegion", "true")
        .append_pair("app_id", "areena-web-items")
        .append_pair("app_key", "wlTs5D9OjIdeS9krPzRQR4I1PYVzoazN");

    return url
}

async fn make_request(client: &Client, url: Url) -> Result<ApiResponse, reqwest::Error> {
    client
        .get(url)
        .send()
        .await?
        .json::<ApiResponse>()
        .await
}

/* ELI EI KOSKAAN LOGITA DATAA  */
async fn fetch_category(url: Url, id: String, client: &Client) {

    let response = match make_request(client, url).await {
        Ok(data) => data,

        Err(_) => {
            /* new fetch */
            let new_token = match get_token(id.to_string()).await {
                Ok(token) => token,
                Err(e) => {
                    println!("{:?}", e); 
                    return;
                }
            };

            let token_ref: &str = &new_token;
            let new_url = make_url(token_ref).await;
    
         //   println!("new url is {:?} ", new_url);

            match make_request(client, new_url).await {
                Ok(data) => data,
                Err(e) => { 
                    println!(" error occured {:?} ", e); /* tulee aina tänne */
                    return 
                }
            }
        }
    };



    let mut new_data: Vec<OutputData> = Vec::new();

    for item in &response.data {
        if let Some(raw) = &item.labels[0].raw { 
            let typ = &item.pointer.r#type; 

            let new_item = OutputData {
                title: String::from(item.title.clone()),
                description: String::from(item.description.clone()),
                uri_pointer: String::from(raw),
                class: String::from(typ),
                image: String::from(item.image.id.clone()),
            };
            new_data.push(new_item);
        }
    }

    println!("{:?}", new_data);

}

async fn get_categories() -> Result<Vec<Category>> {

    let connection = Connection::open("programs.db")?;

    let mut statement = connection.prepare("SELECT name, token FROM categories")?;

    let categories = statement
        .query_map([], |row| {
            Ok(Category {
                name: row.get(0)?,
                token: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(categories)
}

fn update_token_into_db(key: String, new_token: String) -> Result<()> {
    let connection = Connection::open("programs.db")?;

    connection.execute(
        "UPDATE categories SET token = ?1 WHERE name = ?2",
        [&new_token, &key],
    )?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {

    let client = Client::new();
    let categories = match get_categories().await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching categories: {}", e);
            return Err(e);
        }
    };

   for item in &categories {
     let all_categories: Vec<(&str, &str)> = [
            RIKOS,
            NUORET,
            DRAAMASARJAT,
            REALITY,
            KOMEDIASARJAT,
            SEKALAISET,
            DOKUMENTIT,
            LUONTO,
            HISTORIA,
        ]
        .concat();
        
        let id = all_categories.iter()
            .find(|(n, _token)| *n == item.name.clone())
            .map(|(_n, token)| *token);
        
        if let Some(found_id) = id {
            if let Some(token) = &item.token {
                /* TOKEN VAIHTUU AINA SE MÄÄRITTÄÄ GENREN */
                let url = make_url(token).await;

                std::thread::sleep(std::time::Duration::from_millis(15000));

                let _ = fetch_category(url, found_id.to_string(), &client).await; 
            } else {
                let new_token = get_token(found_id.to_string()).await.unwrap();
                let token_ref: &str = &new_token;
                let url = make_url(token_ref).await;
                let _ = fetch_category(url, found_id.to_string(), &client).await; 
                let _ = update_token_into_db(item.name.clone(), new_token);
                std::thread::sleep(std::time::Duration::from_millis(15000));
            }
        } else {
            println!("Womp womp failed");
        }

//tän pitäs palauttaa ok tai ei ok
    }
    Ok(())
}
