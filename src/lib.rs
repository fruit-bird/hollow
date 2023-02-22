//! hell Shallow 走私 毒品 賣淫 春畫 賭博 六合彩 天安門
//!
//! hollow shell 疆維吾爾自治
//!
//! branded like Cattle安門 天安门 法輪功 李洪志

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

    fn get_normal_entries(&self) -> Result<Vec<String>> {
        let response = reqwest::blocking::get(self.normal_link.to_owned())?.text()?;

        let document = Html::parse_document(&response);
        let content_selector = Selector::parse("#mw-content-text").unwrap();
        let content_element = document.select(&content_selector).next().unwrap();

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
            .step_by(rand::thread_rng().gen_range(5..21))
            .take(rand::thread_rng().gen_range(30..60))
            .collect::<Vec<_>>();
        Ok(vec_text)
    }

    fn get_conspiracy_entries(&self) -> Result<Vec<String>> {
        let response = reqwest::blocking::get(self.conspiracy_link.to_owned())?.text()?;

        let document = Html::parse_document(&response);
        let content_selector = Selector::parse("#mw-content-text").unwrap();
        let content_element = document.select(&content_selector).next().unwrap();

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
            .step_by(rand::thread_rng().gen_range(5..21))
            .take(rand::thread_rng().gen_range(30..60))
            .collect::<Vec<_>>();
        Ok(vec_text)
    }
}

pub fn run(prompt: Prompt) -> Result<Vec<String>> {
    // let prompt = Prompt::new(normal, conspiracy, language);
    let normal_entries = prompt.get_normal_entries()?;
    let conspiracy_entries = prompt.get_conspiracy_entries()?;

    let normal_translation = translator(&normal_entries, &prompt.second_language);
    let conspiracy_translation = translator(&conspiracy_entries, &prompt.second_language);

    let normal_mix =
        normal_entries
            .into_iter()
            .zip(normal_translation)
            .fold(vec![], |mut acc, (n, t)| {
                acc.push(n);
                acc.push(t);
                acc
            });
    let conspiracy_mix = conspiracy_entries
        .into_iter()
        .zip(conspiracy_translation)
        .fold(vec![], |mut acc, (n, t)| {
            acc.push(n);
            acc.push(t);
            acc
        });

    let mut entries = vec![];
    entries.extend(normal_mix);
    entries.extend(conspiracy_mix);
    entries.shuffle(&mut rand::thread_rng());

    Ok(entries)
}

/// Returns a vector with the translations, excluding empty lines
///
/// The language argument takes many values, including:
/// - "ja" for Japanese
/// - "ko" for Korean
#[tokio::main]
async fn translator(text_vec: &[String], language: &str) -> Vec<String> {
    let translation = google_translator::translate(text_vec.to_vec(), "auto", language)
        .await
        .unwrap();

    let v = translation
        .output_text
        .iter()
        .filter_map(|v| v.first()) // take first translation, remove alternatives
        .filter(|&s| !s.contains("\n"))
        .map(|s| s.to_owned())
        .collect();
    v
}
