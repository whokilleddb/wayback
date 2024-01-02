# Wayback 
[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/whokilleddb)

A multithreaded approach to fetching URLs from the wayback machine! 

## Command Line Arguments

The tool supports the following command line options

```
$ wayback --help 

Fetch all endpoints for a domain from the wayback machine

Usage: wayback [OPTIONS] <domains>...

Arguments:
  <domains>...  Space separated list of domains to enumerate

Options:
  -o, --outfile <outfile>  File to save output to
  -s, --subdomains         Enumerate subdomains as well
  -j, --json               Save output as JSON file
  -h, --help               Print help
```

For example, to enumerate domains for `google.com` and `youtube.com`, and store them as a JSON file, the command would be:

```
$ wayback --json --outfile example.json google.com youtube.com
```

## Notes

- All the printing which does not include the URL endpoints happens out to `stderr` so you can pipe the output from `stdout` to be used in scripts as well!
