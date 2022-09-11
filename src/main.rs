use std::fs::File;
use std::{io::Write, path::Path};

mod find;
mod load_sigs;
mod scrape_four_bytes;
mod tokenize;

fn main() {
    if !Path::new("four_bytes.json").exists() {
        println!("four_bytes.json not found. Running lazy scraper...");

        match scrape_four_bytes::scrape_four_bytes() {
            Ok(_) => println!("Scraping complete!"),
            Err(e) => println!("Scraping failed: {}", e),
        }
    }

    match load_sigs::load_sigs() {
        Ok(sigs) => {
            let sigs_map = load_sigs::fill_hashmap(&sigs);
            let text_sigs = load_sigs::get_text_sigs(&sigs);
            let words = tokenize::get_all_unique_words_from_sigs(text_sigs);

            let mut file = File::create("words.txt").unwrap();
            for word in words {
                file.write_all(word.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            }
        }
        Err(e) => println!("Error loading signatures: {}", e),
    }
}
