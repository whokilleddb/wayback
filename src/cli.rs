use std::path::PathBuf;
use clap::{Arg, ArgAction, ArgMatches, Command};

#[derive(Debug)]
pub struct CliOpts  {
    pub domains: Vec<String>,               // Domain name to scrape
    pub outfile: Option<PathBuf>,           // Output file to write results to 
    pub subdomains: bool
}

impl CliOpts {
    pub fn new()-> Self {
        let cli_opts: ArgMatches = Command::new("wayback")
            .about("Fetch all endpoints for a domain from the wayback machine")
            .author("DB (whokilleddb@gmail.com)")
            .arg(
                Arg::new("domains").action(ArgAction::Append).help("Comma separated list of domains to enumerate").required(true)
            )
            .arg(
                Arg::new("outfile").short('o').long("outfile").help("File to save output to")
            )
            .arg(
                Arg::new("subdomains").short('s').long("subdomains").help("Enumerate subdomains as well").action(ArgAction::SetTrue),
            )
            .get_matches();
    
        // Get a Vec<> of domain names
        let domains: Vec<String> = cli_opts.get_many::<String>("domains")
            .unwrap_or_default()
            .map(|v| v.clone())
            .collect::<Vec<_>>();
        
        // Get name of outfile
        let outfile: Option<PathBuf> = match cli_opts.get_one::<String>("outfile") {
            Some(v) => Some(PathBuf::from(v)),
            None => None
        };
    
        let subdomains: bool = match cli_opts.get_one::<bool>("subdomains") {
            Some(v) => *v,
            None => false,
        };

        Self {
            domains,
            outfile,
            subdomains
        }
    }
}


