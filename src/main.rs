// use arboard::Clipboard;
use clap::Parser;
use hollow::HollowPrompt;

/// SeEk TRuth
#[derive(Debug, Parser)]
struct HollowArgs {
    /// Topic name to any Wikipedia article
    #[arg(default_value_t = String::from("Rumpelstiltskin"))]
    first_topic: String,
    /// Topic name to any Wikipedia article
    #[arg(default_value_t = String::from("Moon landing conspiracy"))]
    second_topic: String,
    /// Language to mix into the output
    #[arg(short = 'l', long = "lang", default_value_t = String::from("ja"))]
    second_language: String,
    // /// Copy the output to the clipboard
    // #[arg(short, long)]
    // clipboard: bool,
}

// TODO: change this to a git branch. 2 commits earlier in master is the older, better working version
#[tokio::main]
async fn main() {
    let args = HollowArgs::parse();
    let prompt = HollowPrompt::new(args.first_topic, args.second_topic, args.second_language);

    let the_spooky = match prompt.run().await {
        Ok(entry) => entry,
        Err(e) => {
            eprintln!("Error while seeking truth: {}", e);
            std::process::exit(1)
        }
    };

    println!("{}", the_spooky);

    // if args.clipboard {
    //     Clipboard::new()
    //         .expect("Could not fetch clipboard")
    //         .set_text(the_spooky)
    //         .unwrap();
    // }
}
