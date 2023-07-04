extern crate chbs;
use chbs::{config::BasicConfig, prelude::*, probability::Probability};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Amount of words in the passphrase
    #[arg(short, long, default_value_t = 6)]
    length: usize,

    /// Separate words by this value. Default - space.
    #[arg(short, long, default_value_t = ' ')]
    delimeter: char,

    /// Capitalize words
    #[arg(short, long, default_value_t = true)]
    caps: bool,
}


fn main() {
    let args = Args::parse();

    let mut config = BasicConfig::<chbs::word::WordSampler> { words: args.length, separator: args.delimeter.to_string(), ..Default::default() };
    if args.caps {
        config.capitalize_first = Probability::Always;
    } else {
        config.capitalize_first = Probability::Never;
    }
    let scheme = config.to_scheme();
    println!("{}", scheme.generate());
}
