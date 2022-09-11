use std::io::Write;

use reqwest::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    count: u32,
    results: Vec<FourBytes>,
}

#[derive(Deserialize)]
struct FourBytes {
    id: u32,
    created_at: String,
    text_signature: String,
    hex_signature: String,
    bytes_signature: String,
}

struct SignaturesApi {
    results: <Vec<FourBytes> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl SignaturesApi {
    fn new() -> Result<Self> {
        Ok(SignaturesApi {
            results: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            page: 0,
            per_page: 100,
            total: 0,
        })
    }

    fn try_next(&mut self) -> Result<Option<FourBytes>> {
        if let Some(res) = self.results.next() {
            return Ok(Some(res));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!(
            "https://www.4byte.directory/api/v1/signatures/?page={}",
            self.page
        );

        // print every 10 pages
        if self.page % 100 == 0 {
            println!("page: {}", self.page);
        }

        let response = self.client.get(&url).send()?.json::<ApiResponse>()?;
        self.results = response.results.into_iter();
        self.total = response.count;
        Ok(self.results.next())
    }
}

impl Iterator for SignaturesApi {
    type Item = Result<FourBytes>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(res)) => Some(Ok(res)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

pub fn scrape_four_bytes() -> Result<()> {
    println!("Scraping 4bytes signatures. This may take a while...");

    let file = std::fs::File::create("four_bytes.json").unwrap();
    let mut writer = std::io::BufWriter::new(file);

    writer.write_all(b"[\n").unwrap();

    for res in SignaturesApi::new()? {
        let res = res?;
        let formatted_result = format!(
            "{{
                \"id\": {}, 
                \"created_at\": \"{}\", 
                \"text_signature\": \"{}\", 
                \"hex_signature\": \"{}\" \n}},\n",
            res.id, res.created_at, res.text_signature, res.hex_signature
        );

        match writer.write_all(formatted_result.as_bytes()) {
            Ok(_) => (),
            Err(err) => eprintln!("Error writing to file: {}", err),
        };
    }

    // close json file
    writer.write_all(b"]").unwrap();

    Ok(())
}
