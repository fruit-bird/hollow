use crate::random::random_range;
use crate::utils::translator;

use anyhow::Result;
use rand::seq::SliceRandom;
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
        let response = reqwest::get(self.normal_link.to_owned());
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = rt.block_on(async { response.await.unwrap().text().await.unwrap() });

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
            .step_by(random_range(5, 21))
            .take(random_range(30, 60))
            .collect::<Vec<_>>();

        Ok(vec_text)
    }

    fn get_conspiracy_entries(&self) -> Result<Vec<String>> {
        let response = reqwest::get(self.conspiracy_link.to_owned());
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = rt.block_on(async { response.await.unwrap().text().await.unwrap() });

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
            .step_by(random_range(5, 21))
            .take(random_range(30, 60))
            .collect::<Vec<_>>();

        Ok(vec_text)
    }

    pub fn run(&self) -> Result<String> {
        let normal_entries = self.get_normal_entries()?;
        let conspiracy_entries = self.get_conspiracy_entries()?;
    
        let normal_translation = translator(&normal_entries, &self.second_language);
        let conspiracy_translation = translator(&conspiracy_entries, &self.second_language);
    
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let normal_translation = rt.block_on(async { normal_translation.await });
        let conspiracy_translation = rt.block_on(async { conspiracy_translation.await });
    
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
