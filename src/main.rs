use std::fs::File;
use std::{io::Write, path::Path};

mod markov;
mod sigs;
mod utils;

use markov::{markov as mkv, tokenize};
use sigs::{load_sigs, scrape_four_bytes, utils as sig_utils};

fn main() {
    let args = utils::cli::get_args();
    let target = args.target_function;
    let target_selector = sig_utils::get_selector(&target);

    if !Path::new("four_bytes.json").exists() {
        println!("four_bytes.json not found. Running lazy scraper...");

        match scrape_four_bytes::scrape_four_bytes() {
            Ok(_) => println!("Scraping complete!"),
            Err(e) => println!("Scraping failed: {}", e),
        }
    } else {
        println!("four_bytes.json found. Proceeding.");
    }

    match load_sigs::load_sigs() {
        Ok(sigs) => {
            let text_sigs = load_sigs::get_text_sigs(&sigs);
            let unique_words = tokenize::get_all_unique_words_from_sigs(text_sigs);

            // check if the words file exists
            if !Path::new("words.txt").exists() {
                println!("corpus words.txt not found. Creating it now...");

                let mut file = File::create("words.txt").unwrap();
                for word in unique_words {
                    file.write_all(word.as_bytes()).unwrap();
                    file.write_all(b"\n").unwrap();
                }
                println!("words.txt created!");
            } else {
                println!("words.txt found. Proceeding.");
            }
        }
        Err(e) => println!("Error loading signatures: {}", e),
    }

    let markov = mkv::MarkovEngine::new(3, "words.txt");
    let mut fn_name = markov.generate_one();
    println!("Generated function name: {}", fn_name);

    // generate random function names until the selector matches the cli argument
    let mut count = 0;
    while sig_utils::get_selector(&format!("{}()", fn_name)) != target_selector {
        fn_name = markov.generate_one();
        count += 1;

        if count % 1000000 == 0 {
            print!("\rGenerated {}M function names so far...", count / 1000000);
        }
    }

    println!("Match found: {}", fn_name);
    println!("Generated {} function names before finding a match.", count);
}
