// use arboard::Clipboard;
use clap::Parser;
use hollow::Prompt;

#[derive(Debug, Parser)]
#[clap(author)]
struct Args {
    /// Wikipedia link to any article
    normal_link: String,
    /// Wikipedia link to a conspiracy article
    conspiracy_link: Option<String>,
    /// Language to mix into the output
    #[arg(short, long = "lang", default_value_t = String::from("ja"))]
    language: String,
    // /// Copy the output to the clipboard
    // #[arg(short, long)]
    // clipboard: bool,
}

const DEFAULT_CONSPIRACY_LINK: &str =
    "https://en.wikipedia.org/wiki/Moon_landing_conspiracy_theories";

fn main() {
    let args = Args::parse();
    let prompt = Prompt::new(
        &args.normal_link,
        &args
            .conspiracy_link
            .unwrap_or(DEFAULT_CONSPIRACY_LINK.to_string()),
        &args.language,
    );

    let the_spooky = match prompt.run() {
        Ok(entry) => entry,
        Err(_) => std::process::exit(1),
    };

    println!("{}", the_spooky);

    // if args.clipboard {
    //     Clipboard::new()
    //         .expect("Could not fetch clipboard")
    //         .set_text(the_spooky)
    //         .unwrap();
    // }
}
