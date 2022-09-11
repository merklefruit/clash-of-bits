use std::path::Path;

mod find;
mod load_sigs;
mod scrape_four_bytes;

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
            let map = load_sigs::fill_hashmap(sigs);
            println!("Loaded {} signatures in the HashMap!", map.len());
        }
        Err(e) => println!("Error loading signatures: {}", e),
    }
}
