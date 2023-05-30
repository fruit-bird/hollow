use anyhow::Result;
use rand::{seq::SliceRandom, Rng};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Prompt {
    normal_link: String,
    conspiracy_link: String,
    second_language: String,
}

impl Prompt {
    /// Links are assumed to be valid wiki article links
    pub fn new(normal_link: &str, conspiracy_link: &str, language: &str) -> Prompt {
        Prompt {
            normal_link: normal_link.to_string(),
            conspiracy_link: conspiracy_link.to_string(),
            second_language: language.to_string(),
        }
    }

    async fn get_normal_entries(&self) -> Result<Vec<String>> {
        let response = reqwest::get(&self.normal_link).await?.text().await?;
        let article_body = get_article_body(&response);
        Ok(article_body)
    }

    async fn get_conspiracy_entries(&self) -> Result<Vec<String>> {
        let response = reqwest::get(&self.conspiracy_link).await?.text().await?;
        let article_body = get_article_body(&response);
        Ok(article_body)
    }

    pub async fn run(&self) -> Result<String> {
        let normal_entries = self.get_normal_entries().await?;
        let conspiracy_entries = self.get_conspiracy_entries().await?;

        let normal_translation = translator(&normal_entries, &self.second_language).await?;
        let conspiracy_translation = translator(&conspiracy_entries, &self.second_language).await?;

        let normal_mix = combine_original_and_translation(normal_entries, normal_translation);
        let conspiracy_mix =
            combine_original_and_translation(conspiracy_entries, conspiracy_translation);

        let mut entries = vec![];
        entries.extend(normal_mix);
        entries.extend(conspiracy_mix);
        entries.shuffle(&mut rand::thread_rng());

        Ok(entries.join(" "))
    }
}

fn get_article_body(response: &str) -> Vec<String> {
    let document = Html::parse_document(&response);
    let content_selector = Selector::parse("#mw-content-text").unwrap();
    let content_element = document.select(&content_selector).next().unwrap();

    let body_text = content_element
        .text()
        .filter(|s| s.len() < 65)
        .filter_map(|s| {
            match s.contains("\n") || s.contains("\u{a0}") || s.contains("[") || s.contains("]") {
                true => None,
                false => Some(s.split(' ').take(4).map(|s| s.to_string() + " ").collect()),
            }
        })
        .step_by(rand::thread_rng().gen_range(6..21))
        .take(rand::thread_rng().gen_range(30..60))
        .collect();
    body_text
}

#[derive(Debug, thiserror::Error)]
#[error("Error while fetching translation")]
struct TranslateError;

/// Returns a vector with the translations if it succeeds
async fn translator(text_vec: &[String], language: &str) -> Result<Vec<String>, TranslateError> {
    let translation = google_translator::translate(text_vec.to_vec(), "auto", language).await;

    if let Ok(t) = translation {
        Ok(t.output_text
            .into_iter()
            .filter_map(|v| v.first().map(|s| s.to_owned())) // take first translation, remove alternatives
            .filter(|s| !s.contains("\n"))
            .collect())
    } else {
        Err(TranslateError)
    }
}

fn combine_original_and_translation(
    entries: Vec<String>,
    translations: Vec<String>,
) -> Vec<String> {
    entries
        .into_iter()
        .zip(translations)
        .fold(vec![], |mut acc, (n, t)| {
            acc.push(n);
            acc.push(t);
            acc
        })
}
