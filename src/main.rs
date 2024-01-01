//! # Wayback 
//! A command line tool to scrape targets from the Wayback Machine
use jsonxf;
use std::thread;
use std::fs::File;
use std::io::Write;

mod cli;
mod fetcher;

use crate::fetcher::*;
use crate::cli::*;

#[tokio::main]
async fn main() {
    let cli_opts = CliOpts::new();
    
    // Iterate through the list of provided domains
    let domains = cli_opts.domains;
   
    // Iterate over each element in the Vec and spawn a thread for each
    let handles: Vec<_> = domains.clone()
        .into_iter()
        .map(|domain| {
            // Spawn a thread for each element
            thread::spawn(move || {
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

    let pretty_json: String = jsonxf::pretty_print(&json_data).unwrap();

    // Print data
    // eprintln!("[#] Enumerated the following domains:");
    // for e_data in data {
    //     // eprintln!("[i] Domain Name:\t\t{}", e_data.domain_name);
    //     // eprintln!("[i] Enumerate Subdomain:\t{}", cli_opts.subdomains);
    //     // eprintln!("[i] Number of URLs:\t\t{}", e_data.url_list.len());
    //     // eprintln!("[i] Request Digest:\t\t{}", e_data.digest);
    //     // eprintln!("[i] Request Timestamp:\t\t{}\n", e_data.timestamp);
    //     println!("\t{},\n", e_data);

    // }
    // println!("}}");

    match cli_opts.outfile {
        Some(v) => {
            eprintln!("[>] Saving output to file:\t{:?}", v);
            // Attempt to create a file, or open it if it already exists
            let mut file = match File::create(v.clone()) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error creating file: {}", e);
                    return;
                }
            };

            // Write the content to the file
            match file.write_all(pretty_json.as_bytes()) {
                Ok(_) => eprintln!("[i] Content successfully written to {}", v.display()),
                Err(e) => eprintln!("[?] Error writing to file: {}", e),
            }
            
        },
        None => {
            eprintln!("[>] Printing output to STDOUT:");
            println!("{}", pretty_json);
        }
    };
}
