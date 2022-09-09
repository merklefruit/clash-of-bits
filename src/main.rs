use ethers::utils::{hex, keccak256};
use rand;
use rand::seq::SliceRandom;
use std::env;
use std::thread;
mod data;

struct Args {
    target: String,
    threads: usize,
    limit: usize,
}

fn main() {
    let args = Args {
        target: env::args()
            .nth(1)
            .expect("Please specify a target selector in hex"),
        threads: env::args()
            .nth(2)
            .map(|s| s.parse().expect("Please specify a number of threads"))
            .unwrap_or(1),
        limit: env::args()
            .nth(3)
            .expect("Please specify a max limit of lookups")
            .parse()
            .expect("Please specify a number for the max limit"),
    };

    hex::decode(&args.target).expect("Please specify a valid hex string");
    assert_eq!(args.target.len(), 8, "Please specify a 4 byte hex string");

    fn get_selector_from_slice(slice: &[u8]) -> String {
        hex::encode(&keccak256(slice))[..8].to_string()
    }

    fn generate_random_solidity_function_name() -> String {
        let gcw = data::get_common_words();
        let gcat = data::get_common_argument_types();

        let words = gcw
            .choose_multiple(&mut rand::thread_rng(), 2)
            .collect::<Vec<_>>();

        let args = gcat
            .choose_multiple(&mut rand::thread_rng(), 2)
            .collect::<Vec<_>>();

        // generate 4 random numbers to the function name
        let mut random_numbers = String::new();
        for _ in 0..2 {
            random_numbers.push_str(&rand::random::<u8>().to_string());
        }

        format!(
            "{}_{}{}({},{})",
            words[0], words[1], random_numbers, args[0], args[1]
        )
    }

    fn check_selector(selector: &str, target: &str) -> bool {
        selector == target
    }

    // loop generating random function names, hashing them
    // until we find a match with the target
    let mut counter = 0;
    let mut threads = vec![];

    for _ in 0..args.threads {
        let target = args.target.clone();

        threads.push(thread::spawn(move || loop {
            let fn_name = generate_random_solidity_function_name();
            let selector = get_selector_from_slice(fn_name.as_bytes());

            // println!("{}: {}", fn_name, selector);

            if check_selector(&selector, &target) {
                println!("Found match: {}", selector);
                println!("Function name: {}", fn_name);
                break;
            }

            counter += 1;
            if counter % 100000 == 0 {
                println!("{} lookups", counter);
            }

            if counter > args.limit {
                println!("Reached limit of {} lookups", counter);
                break;
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Done");
}
