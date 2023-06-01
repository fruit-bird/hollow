use anyhow::Result;
use google_translator::translate_one_line;
use rand::{seq::SliceRandom, Rng};
use wikipedia::{http::default::Client, Wikipedia};

pub struct HollowPrompt {
    first_topic: String,
    second_topic: String,
    second_language: String,
}

impl HollowPrompt {
    pub fn new(first_topic: &str, second_topic: &str, language: &str) -> HollowPrompt {
        HollowPrompt {
            first_topic: first_topic.to_string(),
            second_topic: second_topic.to_string(),
            second_language: language.to_string(),
        }
    }

    // change the unwraps to ? after changing the crate and adding ? support with thiserror
    pub async fn run(&self) -> Result<String> {
        let wiki = Wikipedia::<Client>::default();

        let content_1 = clean_article_content(
            &wiki
                .page_from_title((&self.first_topic).to_string())
                .get_content()
                .unwrap(),
        );
        let content_2 = clean_article_content(
            &wiki
                .page_from_title((&self.second_topic).to_string())
                .get_content()
                .unwrap(),
        );

        // let normal_translation = translator(&content_1, &self.second_language).await?;
        // let conspiracy_translation = translator(&content_2, &self.second_language).await?;
        let first_translation =
            translate_one_line(content_1.clone(), "auto", &self.second_language).await?;
        let second_translation =
            translate_one_line(content_2.clone(), "auto", &self.second_language).await?;

        let first_mix = combine_original_and_translation(&content_1, &first_translation);
        let conspiracy_mix = combine_original_and_translation(&content_2, &second_translation);

        let mut entries = vec![];
        entries.extend(first_mix.lines());
        entries.extend(conspiracy_mix.lines());
        entries.shuffle(&mut rand::thread_rng());

        Ok(entries.join(" "))
    }
}

fn clean_article_content(content: &str) -> String {
    content
        .split('\n')
        .filter_map(|s| {
            match s.len() < 65
                || s.contains("\n")
                || s.contains("\u{a0}")
                || s.contains("[")
                || s.contains("]")
            {
                true => None,
                false => Some(
                    s.split(' ')
                        .take(4)
                        .map(|s| s.to_string() + " ")
                        .collect::<String>(),
                ),
            }
        })
        .step_by(rand::thread_rng().gen_range(6..21))
        .take(rand::thread_rng().gen_range(30..60))
        .collect()
}

fn combine_original_and_translation(entries: &str, translations: &str) -> String {
    entries
        .to_string()
        .lines()
        .zip(translations.to_string().lines())
        .fold(String::new(), |mut acc, (n, t)| {
            acc.push_str(n);
            acc.push_str(t);
            acc
        })
}
