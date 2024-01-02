use chrono::Local;
use md5::Context;
use reqwest::blocking::Client;
use reqwest::{self, header};
use std::fmt;

// Store fetch data, it's hash, and a timestamp
#[derive(Clone, Debug)]
pub struct UrlData {
    pub domain_name: String,
    pub url_list: Vec<String>,
    pub digest: String,
    pub timestamp: String,
}

impl fmt::Display for UrlData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = format!(
            "\"{}\": {{\n\t\"timestamp\":\t\"{}\",\n\t\"digest\":\t\"{}\",\n\t\"urls\":{:?}\n\t}}",
            self.domain_name, self.timestamp, self.digest, self.url_list
        );
        write!(f, "{}", msg)
    }
}

impl UrlData {
    pub fn new(d_name: String) -> Self {
        Self {
            domain_name: d_name,
            url_list: Vec::new(),
            digest: String::new(),
            timestamp: String::new(),
        }
    }

    pub fn fetch(&mut self, no_subs: bool) {
        let dt = Local::now();
        let mut ctx: Context = Context::new();

        // URL to fetch data from
        let url = match no_subs {
            false => format!("http://web.archive.org/cdx/search/cdx?url=*.{}/*&output=text&fl=original&collapse=urlkey", self.domain_name),
            true => format!("http://web.archive.org/cdx/search/cdx?url={}/*&output=text&fl=original&collapse=urlkey", self.domain_name)
        };

        // Headers to send
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
        );

        let client = Client::builder().default_headers(headers).build().unwrap();
        let resp = client.get(url.as_str()).send().unwrap();
        if !resp.status().is_success() {
            panic!("Got status code: {}", resp.status());
        }

        // println!("[i] Status: {}", resp.status());

        let resp_text: Vec<String> = resp
            .text()
            .unwrap()
            .split_whitespace()
            .map(|x| String::from(x))
            .collect();
        let b_payload: Vec<u8> = resp_text.concat().into_bytes(); // Concatenate all strings in the vector
        ctx.consume(&b_payload);

        self.url_list = resp_text.clone();
        self.digest = format!("{:#?}", ctx.compute());
        self.timestamp = format!("{}", dt.format("%H:%M %d/%m/%Y"));
    }
}
