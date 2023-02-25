/// Returns a vector with the translations
pub async fn translator(text_vec: &[String], language: &str) -> Vec<String> {
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
