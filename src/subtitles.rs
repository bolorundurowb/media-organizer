use scraper::{Html, Selector};
use crate::constants::SCRAPER_USER_AGENT;

pub async fn search_subtitles_by_imdb_id(file_name: &str, imdb_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let download_page_url = get_subtitle_download_page_url_by_imdb_id(imdb_id).await?;
    let download_url = get_subtitle_download_url(&download_page_url).await?;
    download_subtitle(file_name, &download_url).await?;

    Ok(())
}

async fn get_subtitle_download_page_url_by_imdb_id(imdb_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://www.opensubtitles.org/en/search/sublanguageid-eng/subformat-srt/imdbid-{}", imdb_id);

    let client = reqwest::Client::builder()
        .user_agent(SCRAPER_USER_AGENT)
        .build()?;
    let resp = client.get(&url).send().await?.text().await?;

    let document = Html::parse_document(&resp);
    let selector = Selector::parse("a.bnone").unwrap();

    for element in document.select(&selector) {
        if let Some(subtitle_download_page_link) = element.attr("href") {
            let download_page_link = normalize_url(&subtitle_download_page_link);
            return Ok(download_page_link);
        }
    }

    Err("Failed to get subtitle download page".into())
}

async fn get_subtitle_download_url(subtitle_download_page_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent(SCRAPER_USER_AGENT)
        .build()?;
    let resp = client.get(subtitle_download_page_url).send().await?.text().await?;

    let document = Html::parse_document(&resp);
    let selector = Selector::parse("a.bt-dwl").unwrap();

    for element in document.select(&selector) {
        if let Some(subtitle_download_link) = element.attr("href") {
            let download_link = normalize_url(&subtitle_download_link);
            return Ok(download_link);
        }
    }

    Err("Failed to get subtitle download link".into())
}

async fn download_subtitle(file_name: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent(SCRAPER_USER_AGENT)
        .build()?;
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        std::fs::write(file_name, bytes)?;
    } else {
        eprintln!("Failed to download subtitle: {}", response.status());
    }

    Ok(())
}

fn normalize_url(url: &str) -> String {
    if url.starts_with("/") {
        format!("https://www.opensubtitles.org{}", url)
    } else {
        url.to_string()
    }
}