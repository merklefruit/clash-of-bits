use rand::seq::SliceRandom;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct MarkovEngine {
    order: usize,
    filename: String,
    ngrams_map: HashMap<String, Vec<(String, u32)>>,
}

impl MarkovEngine {
    // create a new struct with the given order
    // and a corpus of words to generate ngrams from
    pub fn new(order: usize, filename: &str) -> Self {
        let mut corpus = String::new();
        let mut ngrams_map = HashMap::new();

        // read the corpus from the file
        match std::fs::read_to_string(filename.clone()) {
            Ok(s) => corpus = s.replace("\n", ""),
            Err(e) => println!("Error reading corpus: {}", e),
        }
        println!("Corpus length: {} bytes", corpus.len());

        // split the corpus into ngrams of the given order
        let all_ngrams = corpus
            .chars()
            .collect::<Vec<_>>()
            .windows(order)
            .map(|w| w.iter().collect::<String>())
            .collect::<Vec<_>>();

        // create a map of ngrams to their following ngrams
        // by looking at every occurrence of an ngram in the corpus
        for ngram in all_ngrams {
            let ngram_positions = find_all_occurrences(&corpus, &ngram);
            let mut following_ngrams = Vec::new();

            for position in ngram_positions {
                if (position + order * 2) < corpus.len() {
                    let following_ngram = &corpus[position + order..position + order + order];
                    following_ngrams.push(following_ngram.to_string());
                }
            }

            // build probabilities for each following ngram
            let following_ngrams_map = following_ngrams
                .iter()
                .fold(HashMap::new(), |mut acc, ngram| {
                    let count = acc.entry(ngram.clone()).or_insert(0);
                    *count += 1;
                    acc
                })
                .into_iter()
                .map(|(k, v)| (k, v as u32))
                .collect::<HashMap<_, _>>();

            // keep only the top 10 following ngrams by highest probability
            let mut following_ngrams = following_ngrams_map.into_iter().collect::<Vec<_>>();
            following_ngrams.sort_by(|(_, a), (_, b)| b.cmp(a));
            let following_ngrams = following_ngrams.into_iter().take(10).collect::<Vec<_>>();

            ngrams_map.insert(ngram, following_ngrams);
        }

        println!("Created n-grams map with {} entries", ngrams_map.len());

        Self {
            order,
            filename: filename.to_string(),
            ngrams_map,
        }
    }

    // generate a new function name
    pub fn generate_one(&self) -> String {
        let mut fn_name = String::new();

        // pick a random ngram from the map
        let mut ngram = self
            .ngrams_map
            .keys()
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        // generate a new name by following the ngrams
        for _ in 0..self.order * 2 {
            fn_name.push_str(&ngram);

            let following_ngrams = self.ngrams_map.get(&ngram).unwrap();
            let following_ngram = following_ngrams
                .choose(&mut rand::thread_rng())
                .unwrap()
                .0
                .to_string();

            ngram = following_ngram;
        }

        fn_name
    }
}

fn find_all_occurrences(haystack: &str, needle: &str) -> Vec<usize> {
    let mut occurrences = Vec::new();
    let mut start = 0;

    while let Some(index) = haystack[start..].find(needle) {
        occurrences.push(start + index);
        start += index + 1;
    }

    occurrences
}
