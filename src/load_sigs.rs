use crate::scrape_four_bytes::FourBytes;
use serde_json;
use std::collections::HashMap;

pub fn load_sigs() -> Result<Vec<FourBytes>, serde_json::Error> {
    let file = std::fs::File::open("four_bytes.json").unwrap();
    let reader = std::io::BufReader::new(file);

    let signatures: Vec<FourBytes> = serde_json::from_reader(reader)?;
    println!("Loaded {} signatures", signatures.len());

    Ok(signatures)
}

pub fn fill_hashmap(sigs: &Vec<FourBytes>) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for sig in sigs {
        map.insert(sig.text_signature.clone(), sig.hex_signature.clone());
    }

    map
}

pub fn get_text_sigs(sigs: &Vec<FourBytes>) -> Vec<String> {
    let mut text_sigs: Vec<String> = Vec::new();

    for sig in sigs {
        text_sigs.push(sig.text_signature.clone());
    }

    text_sigs
}
