//! # Wayback 
//! A command line tool to scrape targets from the Wayback Machine
use jsonxf;
use std::thread;
use std::fs::File;
use std::io::Write;
use colored::*;

mod cli;
mod fetcher;

use crate::fetcher::*;
use crate::cli::*;

#[tokio::main]
async fn main() {
    let cli_opts = CliOpts::new();
    
    // Iterate through the list of provided domains
    let domains = cli_opts.domains;

    eprintln!("==============>> {} by {} <<==============\n", "Wayback".bold().green(), "@whokilleddb".magenta().bold());

    // Iterate over each element in the Vec and spawn a thread for each
    let handles: Vec<_> = domains.clone()
        .into_iter()
        .map(|domain| {
            // Spawn a thread for each element
            thread::spawn(move || {
                eprintln!("[{}] Fetching URLs for:\t{}", "i".cyan() , domain.italic().yellow());
                let mut url_data = UrlData::new(domain);
                url_data.fetch(cli_opts.subdomains);
                url_data
            })
        })
        .collect();

    // Wait for all threads to complete
    let mut data = Vec::new();
    for handle in handles {
       data.push(handle.join().unwrap());
    }

    // Write format
    let text_to_write = match cli_opts.json {
        true => {
            // Convert into one json
            let mut json_data = String::from("{\n");
            let _h_len = domains.clone().len();
            for i in 0.._h_len {
                if i == _h_len - 1 {
                    json_data = format!("{}\t{}\n}}", json_data, data[i]);
                } else {
                    json_data = format!("{}\t{},\n", json_data, data[i]);
                }
            }
            jsonxf::pretty_print(&json_data).unwrap()
        },

        false => {
            let mut __first = true;
            let mut entry = String::new();
            for d in data {
                entry = match __first {
                    true=>{
                        format!("\n# {} | {}\n\n", d.domain_name, d.timestamp)
                    },
                    false =>{
                        format!("{}\n# {} | {}\n\n", entry, d.domain_name, d.timestamp)
                    }
                };

                for u in d.url_list {
                    entry = format!("{}{}\n", entry, u)
                }
                __first = false;
            }
            entry
        }
    };
    
    match cli_opts.outfile {
        Some(v) => {
            let mut __file_path = format!("{:?}", v.clone());
            let mut chars: Vec<char> = __file_path.chars().collect();
            chars.pop();
            chars.remove(0);
            __file_path = chars.into_iter().collect();
           
            // Attempt to create a file, or open it if it already exists
            let mut file = match File::create(v.clone()) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("[!] Error creating file: {}", e);
                    return;
                }
            };

            // Write the content to the file
            file.write_all(text_to_write.as_bytes()).unwrap(); 
            eprintln!("[{}] Saved output to:\t{}", "i".cyan(), __file_path.red().italic());
        },
        None => {
            eprintln!("[{}] Printing output to {}", "i".cyan(), "stdout".red().italic());
            print!("{}", text_to_write);
        }
    };
    eprint!("\n=========================================================");
}
