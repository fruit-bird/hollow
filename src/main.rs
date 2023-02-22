//! hell Shallow 走私 毒品 賣淫 春畫 賭博 六合彩 天安門
//! hollow shell 疆維吾爾自治
//! branded like Cattle安門 天安门 法輪功 李洪志

mod prompt;

use google_translator::translate;
use rand::{seq::SliceRandom, Rng};
use scraper::{Html, Selector};

fn get_wiki_article(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?.text()?;

    let document = Html::parse_document(&response);
    let content_selector = Selector::parse("#mw-content-text")?;
    let content_element = document.select(&content_selector).next().unwrap();

    let mut vec_text = content_element
        .text()
        .filter(|s| s.len() < 60)
        .filter_map(|s| {
            match s.contains("\n") || s.contains("\u{a0}") || s.contains("[") || s.contains("]") {
                true => None,
                false => Some(s.split(' ').take(4).map(|s| s.to_string() + " ").collect()),
            }
        })
        .step_by(rand::thread_rng().gen_range(5..21))
        .take(rand::thread_rng().gen_range(30..60))
        .collect::<Vec<_>>();

    let vec_text_trans = translator(&vec_text);
    let vec_text_trans2 = vec_text_trans.clone(); // janky...
    dbg!(&vec_text_trans);

    vec_text.extend(vec_text_trans);
    vec_text.extend(vec_text_trans2); // ...janky
    vec_text.shuffle(&mut rand::thread_rng());

    Ok(vec_text)
}

#[tokio::main]
async fn translator(text_vec: &Vec<String>) -> Vec<String> {
    let translation = translate(text_vec.to_vec(), "auto", "ja").await.unwrap();
    dbg!(&translation.output_text);
    let v = translation
        .output_text
        .iter()
        .filter_map(|v| v.first())
        .map(|s| s.to_owned())
        .filter(|s| !s.contains("\n"))
        .collect();
    v
}

fn main() {
    // let url = "https://en.wikipedia.org/wiki/Moon_landing_conspiracy_theories";
    // let article_text = get_wiki_article(url).unwrap();
    // println!("{}", article_text.join("\n"));

    let language = "ja";
    let normal = "https://en.wikipedia.org/wiki/Shrek_2";
    let conspiracy = "https://en.wikipedia.org/wiki/Moon_landing_conspiracy_theories";

    if let Err(e) = prompt::run(normal, conspiracy, language) {
        eprintln!("{}", e);
    }
}
