// use arboard::Clipboard;
use clap::Parser;
use hollow::Prompt;

const DEFAULT_NORMAL_LINK: &str = "https://en.wikipedia.org/wiki/Rumpelstiltskin";
const DEFAULT_CONSPIRACY_LINK: &str =
    "https://en.wikipedia.org/wiki/Moon_landing_conspiracy_theories";

/// SeEk TRuth
#[derive(Debug, Parser)]
struct HollowArgs {
    /// Wikipedia link to any article
    #[arg(default_value_t = String::from(DEFAULT_NORMAL_LINK))]
    normal_link: String,
    /// Wikipedia link to a conspiracy article
    #[arg(default_value_t = String::from(DEFAULT_CONSPIRACY_LINK))]
    conspiracy_link: String,
    /// Language to mix into the output
    #[arg(short, long = "lang", default_value_t = String::from("ja"))]
    language: String,
    // /// Copy the output to the clipboard
    // #[arg(short, long)]
    // clipboard: bool,
}

#[tokio::main]
async fn main() {
    let args = HollowArgs::parse();
    let prompt = Prompt::new(&args.normal_link, &args.conspiracy_link, &args.language);

    let the_spooky = match prompt.run().await {
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
