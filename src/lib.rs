use anyhow::Result;
use google_translator::translate;
use rand::{seq::SliceRandom, Rng};
use wikipedia::{http::default::Client, Wikipedia};

pub struct HollowPrompt {
    first_topic: String,
    second_topic: String,
    second_language: String,
}

impl HollowPrompt {
    pub fn new(first_topic: String, second_topic: String, second_language: String) -> HollowPrompt {
        HollowPrompt {
            first_topic,
            second_topic,
            second_language,
        }
    }

    // change the unwraps to ? after changing the wikipedia crate and adding ? support with thiserror
    pub async fn run(&self) -> Result<String> {
        let wiki = Wikipedia::<Client>::default();

        let (content_1, content_2) = self.prepare_article_content(&wiki);
        // let (content_1, content_2) = (
        // clean_article_content(&content_1),
        // clean_article_content(&content_2),
        // );

        let (content_1, content_2) = (
            content_1
                .lines()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
            content_2
                .lines()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        );

        let (c1, c2) = (content_1.clone(), content_2.clone());

        let translation_1 = translate(c1, "auto", &self.second_language)
            .await?
            .output_text;
        let translation_2 = translate(c2, "auto", &self.second_language)
            .await?
            .output_text;

        let mix_1 = combine_original_and_translation(&content_1, &translation_1[0]);
        let mix_2 = combine_original_and_translation(&content_2, &translation_2[0]);

        let mut entries = vec![];
        entries.extend(mix_1);
        entries.extend(mix_2);
        entries.shuffle(&mut rand::thread_rng());

        Ok(entries.join(" "))
        // Ok(content_1.join("\n"))
    }

    /// Divides article body into a string with many lines
    fn prepare_article_content(&self, wiki: &Wikipedia<Client>) -> (String, String) {
        let content_1 = wiki
            .page_from_title(self.first_topic.to_string())
            .get_content()
            .unwrap()
            .replace(".", ",")
            .split(",")
            .map(|s| s.trim().to_string() + "\n")
            .collect();

        let content_2 = wiki
            .page_from_title(self.second_topic.to_string())
            .get_content()
            .unwrap()
            .replace(".", ",")
            .split(",")
            .map(|s| s.trim().to_string() + "\n")
            .collect();

        (content_1, content_2)
    }
}

fn clean_article_content(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|s| match /* s.len() > 100 || */  s.contains("==") {
            true => None,
            false => Some(
                s.split(' ')
                    .take(4)
                    .map(|s| s.to_string() + " ")
                    .collect::<String>(),
            ),
        })
        // .step_by(rand::thread_rng().gen_range(6..10)) // 6..21
        // .take(rand::thread_rng().gen_range(30..60)) // 30..60
        .collect()
}

fn combine_original_and_translation(
    entries: &Vec<String>,
    translations: &Vec<String>,
) -> Vec<String> {
    entries
        .iter()
        .zip(translations)
        .fold(vec![], |mut acc, (n, t)| {
            acc.push(n.clone());
            acc.push(t.clone());
            acc
        })
}
