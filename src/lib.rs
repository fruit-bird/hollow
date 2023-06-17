use anyhow::Result;
use rand::{seq::SliceRandom, Rng};
use scraper::{Html, Selector};

const WIKI_URL: &str = "https://en.wikipedia.org/wiki/";

#[derive(Debug)]
pub struct Hollow<'a> {
    first_link: &'a str,
    second_link: &'a str,
    second_language: &'a str,
}

impl<'a> Hollow<'a> {
    pub fn new(first_link: &'a str, second_link: &'a str, second_language: &'a str) -> Self {
        Self {
            first_link,
            second_link,
            second_language,
        }
    }

    async fn get_normal_entries(&self) -> Result<Vec<String>> {
        let response = if self.first_link.starts_with(WIKI_URL) {
            reqwest::get(self.first_link).await?.text().await?
        } else {
            reqwest::get(format!(
                "https://en.wikipedia.org/wiki/{}",
                &self.first_link
            ))
            .await?
            .text()
            .await?
        };

        let document = Html::parse_document(&response);
        let content_selector = Selector::parse("#mw-content-text").unwrap();
        let content_element = document.select(&content_selector).next().unwrap();

        let mut rng = rand::thread_rng();
        let vec_text = content_element
            .text()
            .filter(|s| s.len() < 60)
            .filter_map(|s| {
                match s.contains("\n") || s.contains("\u{a0}") || s.contains("[") || s.contains("]")
                {
                    true => None,
                    false => Some(s.split(' ').take(4).map(|s| s.to_string() + " ").collect()),
                }
            })
            .step_by(rng.gen_range(5..21))
            .take(rng.gen_range(30..60))
            .collect();

        Ok(vec_text)
    }

    async fn get_conspiracy_entries(&self) -> Result<Vec<String>> {
        let response = if self.second_link.starts_with(WIKI_URL) {
            reqwest::get(self.second_link).await?.text().await?
        } else {
            // in case the input is a topic rather than a link
            reqwest::get(format!(
                "https://en.wikipedia.org/wiki/{}",
                &self.second_link
            ))
            .await?
            .text()
            .await?
        };

        let document = Html::parse_document(&response);
        let content_selector = Selector::parse("#mw-content-text").unwrap();
        let content_element = document.select(&content_selector).next().unwrap();

        let mut rng = rand::thread_rng();
        let vec_text = content_element
            .text()
            .filter(|s| s.len() < 60)
            .filter_map(|s| {
                match s.contains("\n") || s.contains("\u{a0}") || s.contains("[") || s.contains("]")
                {
                    true => None,
                    false => Some(s.split(' ').take(4).map(|s| s.to_string() + " ").collect()),
                }
            })
            .step_by(rng.gen_range(5..21))
            .take(rng.gen_range(30..60))
            .collect();

        Ok(vec_text)
    }

    async fn translator(
        &self,
        normal_entries: &[String],
        conspiracy_entries: &[String],
    ) -> Result<(Vec<String>, Vec<String>)> {
        let normal_translation =
            google_translator::translate(normal_entries, "auto", self.second_language)
                .await?
                .output_text
                .iter()
                .filter_map(|v| v.first()) // take first translation, remove alternatives
                .filter(|s| !s.contains("\n"))
                .map(ToString::to_string)
                .collect();

        let conspiracy_translation =
            google_translator::translate(conspiracy_entries, "auto", self.second_language)
                .await?
                .output_text
                .iter()
                .filter_map(|v| v.first()) // take first translation, remove alternatives
                .filter(|s| !s.contains("\n"))
                .map(ToString::to_string)
                .collect();
        Ok((normal_translation, conspiracy_translation))
    }

    pub async fn run(&self) -> Result<String> {
        let normal_entries = self.get_normal_entries().await?;
        let conspiracy_entries = self.get_conspiracy_entries().await?;

        let (normal_translation, conspiracy_translation) = self
            .translator(&normal_entries, &conspiracy_entries)
            .await?;

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

fn combine_original_and_translation(entries: Vec<String>, translation: Vec<String>) -> Vec<String> {
    entries
        .into_iter()
        .zip(translation)
        .fold(vec![], |mut acc, (n, t)| {
            acc.push(n);
            acc.push(t);
            acc
        })
}
