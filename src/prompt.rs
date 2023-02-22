// have this shit take 2 wiki links
// 1 for the normal topic (say Shrek 2)
// 2 for the conspiracy topic (say 911 conspiracies)
// Then port this to a simple wasm website, after making this code organized and shit
#![allow(unused)]

use anyhow::Result;
use rand::{seq::SliceRandom, Rng};
use scraper::{Html, Selector};
use std::ops::Range;

// #[derive(Debug)]
// pub enum PromptError {
//     InvalidURL,
//     CouldNotFetch,
// }

// impl std::fmt::Display for PromptError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(stringify!(self))
//     }
// }

// impl std::error::Error for PromptError {}

#[derive(Debug, Default)]
struct Prompt {
    normal_link: String,
    conspiracy_link: String,
    second_language: String,
}

impl Prompt {
    /// Links are assumed to be valid wiki article links
    fn new(normal_link: &str, conspiracy_link: &str, language: &str) -> Prompt {
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

pub fn run(normal: &str, conspiracy: &str, language: &str) -> Result<()> {
    let prompt = Prompt::new(normal, conspiracy, language);
    let normal_entries = prompt.get_normal_entries()?;
    let conspiracy_entries = prompt.get_conspiracy_entries()?;

    let normal_translation = translator(&normal_entries, language);
    let conspiracy_translation = translator(&conspiracy_entries, language);

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

    println!("{}", entries.join("\n"));

    Ok(())
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
    dbg!(&translation.output_text);
    let v = translation
        .output_text
        .iter()
        .filter_map(|v| v.first()) // first translation, remove alternatives
        .filter(|&s| !s.contains("\n"))
        .map(|s| s.to_owned())
        .collect();
    v
}
