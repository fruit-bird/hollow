// use arboard::Clipboard;
use clap::Parser;
use hollow::Prompt;

/// SeEk TRuth
#[derive(Debug, Parser)]
struct HollowArgs {
    /// Wikipedia link to any article
    #[arg(default_value_t = String::from("Rumpelstiltskin"))]
    first_topic: String,
    /// Wikipedia link to a conspiracy article
    #[arg(default_value_t = String::from("Moon landing conspiracy"))]
    second_topic: String,
    /// Language to mix into the output
    #[arg(short, long = "lang", default_value_t = String::from("ja"))]
    second_language: String,
    // /// Copy the output to the clipboard
    // #[arg(short, long)]
    // clipboard: bool,
}

#[tokio::main]
async fn main() {
    let args = HollowArgs::parse();
    let prompt = Prompt::new(&args.first_topic, &args.second_topic, &args.second_language);

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
