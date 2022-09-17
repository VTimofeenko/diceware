extern crate chbs;
use chbs::{config::BasicConfig, prelude::*, probability::Probability};
use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("diceware")
        .version("0.1.2")
        .author("Vladimir Timofeenko <public@vtimofeenko.com")
        .about("Primitive diceware commandline")
        .arg(
            Arg::with_name("length")
                .short('l')
                .long("length")
                .takes_value(true)
                .help("Amount of words in the passphrase. Default - 6"),
        )
        .arg(
            Arg::with_name("delimeter")
                .short('d')
                .long("delimeter")
                .takes_value(true)
                .help("Separate words by this value. Default - space."),
        )
        .arg(
            Arg::with_name("caps")
                .short('c')
                .long("caps")
                .help("Capitalize words"),
        )
        .get_matches();

    let len = matches.value_of("length");
    let words;

    match len {
        None => words = 6,
        Some(s) => match s.parse::<usize>() {
            Ok(n) => words = n,
            Err(_) => process::exit(1),
        },
    }

    let mut config = BasicConfig::default();
    config.words = words;
    config.separator = matches.value_of("delimeter").unwrap_or(" ").into();
    if matches.is_present("caps") {
        config.capitalize_first = Probability::Always;
    } else {
        config.capitalize_first = Probability::Never;
    }
    let scheme = config.to_scheme();

    println!("{}", scheme.generate());
}
