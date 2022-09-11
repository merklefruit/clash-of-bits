use std::collections::HashSet;

pub fn get_all_unique_words_from_sigs(sigs: Vec<String>) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();

    for sig in sigs {
        let mut text_sig = sig.clone();

        // split the text signature into words:
        // 1. remove params (from the first '(' onwards)
        // 2. split on capital letter or underscore,
        //    unless it's the first letter or the letter after it is capital aswell

        if let Some(index) = text_sig.find('(') {
            text_sig = text_sig[..index].to_string();
        }

        let mut word = String::new();
        for (i, c) in text_sig.chars().enumerate() {
            if i == 0 {
                word.push(c);
                continue;
            }

            if c.is_uppercase() {
                if text_sig.chars().nth(i - 1).unwrap().is_lowercase() {
                    words.push(word.clone());
                    word = String::new();
                }
            }

            if c == '_' {
                words.push(word.clone());
                word = String::new();
                continue;
            }

            word.push(c);
        }

        words.push(word);
    }

    println!("Found {} words in signatures", words.len());

    filter_unique(words)
}

fn filter_unique(vec: Vec<String>) -> Vec<String> {
    println!("Filtering unique words...");

    let unique: Vec<String> = vec
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    println!("Found {} unique words", unique.len());

    unique
}
