use scraper::{Html, Selector};
use std::error::Error;
use crate::constants::SCRAPER_USER_AGENT;
use crate::utils::url_encode;

#[derive(Debug)]
pub struct ImdbResult {
    pub title: String,
    pub id: String,
}

pub async fn get_imdb_result(movie_name: &str) -> Result<ImdbResult, Box<dyn Error>> {
    let url = format!(
        "https://www.imdb.com/find?q={}&s=tt&ttype=ft&ref_=fn_ft",
        url_encode(&movie_name)
    );

    let client = reqwest::Client::builder()
        .user_agent(SCRAPER_USER_AGENT)
        .build()?;
    let response = client.get(&url).send().await?.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse(".ipc-metadata-list-summary-item__tc a").unwrap();
    if let Some(element) = document.select(&selector).next() {
        let title = element.text().collect::<String>().trim().to_string();
        if let Some(href) = element.value().attr("href") {
            // extract the ID from the href
            if let Some(id_start) = href.find("/title/") {
                let id_end = href[id_start + 7..].find('/').unwrap_or(href.len());
                let id = &href[id_start + 7..id_start + 7 + id_end];
                return Ok(ImdbResult {
                    title,
                    id: id.to_string(),
                });
            }
        }
    }

    Err("IMDb result not found".into())
}
