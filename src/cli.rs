use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliOpts {
    pub domains: Vec<String>,     // Domain name to scrape
    pub outfile: Option<PathBuf>, // Output file to write results to
    pub nosubs: bool,             // Do not use subdomains
    pub json: bool,               // Use JSON flags
}

impl CliOpts {
    pub fn new() -> Self {
        let cli_opts: ArgMatches = Command::new("wayback")
            .about("Fetch URLs for a domain from the wayback machine")
            .author("@whokilleddb")
            .arg(
                Arg::new("domains")
                    .action(ArgAction::Append)
                    .help("Space separated list of domains to enumerate")
                    .required(true),
            )
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .help("File to save output to"),
            )
            .arg(
                Arg::new("nosubs")
                    .long("no-subs")
                    .help("Do not enumerate subdomains for the given domain")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("json")
                    .short('j')
                    .long("json")
                    .help("Save output as JSON file")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        // Get a Vec<> of domain names
        let domains: Vec<String> = cli_opts
            .get_many::<String>("domains")
            .unwrap_or_default()
            .map(|v| v.clone())
            .collect::<Vec<_>>();

        // Get name of outfile
        let outfile: Option<PathBuf> = match cli_opts.get_one::<String>("outfile") {
            Some(v) => Some(PathBuf::from(v)),
            None => None,
        };

        let nosubs: bool = match cli_opts.get_one::<bool>("nosubs") {
            Some(v) => *v,
            None => false,
        };

        let json: bool = match cli_opts.get_one::<bool>("json") {
            Some(v) => *v,
            None => false,
        };

        Self {
            domains,
            outfile,
            nosubs,
            json,
        }
    }
}
