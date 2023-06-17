// use arboard::Clipboard;
use clap::Parser;
use hollow::Hollow;

/// SeEk TruTh
#[derive(Debug, Parser)]
struct HollowArgs {
    /// Wikipedia topic/link to any article
    #[arg(default_value_t = String::from("Rumpelstiltskin"))]
    first_link: String,
    /// Wikipedia topic/link to another article
    #[arg(default_value_t = String::from("Moon landing conspiracies"))]
    second_link: String,
    /// Language to mix into the output
    #[arg(short = 'l', long = "lang", default_value_t = String::from("ja"))]
    second_language: String,
    // /// Copy the output to the clipboard
    // #[arg(short, long)]
    // clipboard: bool,
}

#[tokio::main]
async fn main() {
    let args = HollowArgs::parse();
    let prompt = Hollow::new(&args.first_link, &args.second_link, &args.second_language);

    let the_spooky = match prompt.run().await {
        Ok(entry) => entry,
        Err(e) => {
            eprintln!("{}", e);
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
