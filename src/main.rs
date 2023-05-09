use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_usage();
        return;
    }

    let print_ipv4 = args.contains(&"-4".to_string());
    let print_ipv6 = args.contains(&"-6".to_string());
    let print_public = args.contains(&"-pub".to_string());
    let print_private = args.contains(&"-priv".to_string());
    let silent = args.contains(&"-s".to_string());
    let input_file = args
        .iter()
        .position(|arg| arg == "-f")
        .map(|i| &args[i + 1]);

    let ipv4_regex = Regex::new(r"(?:[0-9]{1,3}\.){3}[0-9]{1,3}").unwrap();
    let ipv6_regex = Regex::new(r"(?:[A-Fa-f0-9]{1,4}:){7}[A-Fa-f0-9]{1,4}").unwrap();

    let input: Box<dyn BufRead> = match input_file {
        Some(file) => Box::new(BufReader::new(File::open(file).unwrap())),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut ipv4s: HashSet<Ipv4Addr> = HashSet::new();
    let mut ipv6s: HashSet<String> = HashSet::new();
    for line in input.lines() {
        let line = line.unwrap();
        for cap in ipv4_regex.captures_iter(&line) {
            if is_valid_ip(&cap[0], print_public, print_private) {
                if let Ok(ipv4) = cap[0].parse() {
                    ipv4s.insert(ipv4);
                }
            }
        }
        for cap in ipv6_regex.captures_iter(&line) {
            if is_valid_ip(&cap[0], print_public, print_private) {
                ipv6s.insert(cap[0].to_string());
            }
        }
    }

    if print_ipv4 || !print_ipv6 {
        let mut ipv4s: Vec<Ipv4Addr> = ipv4s.into_iter().collect();
        ipv4s.sort();
        if !silent {
            println!("IPv4 addresses:");
        }
        for ip in ipv4s {
            println!("{}", ip);
        }
    }

    if print_ipv6 || !print_ipv4 {
        let mut ipv6s: Vec<String> = ipv6s.into_iter().collect();
        ipv6s.sort();
        if !silent && (print_ipv4 || !print_ipv6) {
            println!();
        }
        if !silent {
            println!("IPv6 addresses:");
        }
        for ip in ipv6s {
            println!("{}", ip);
        }
    }
}

fn is_valid_ip(ip: &str, print_public: bool, print_private: bool) -> bool {
    if !print_public && !print_private {
        return true;
    }

    if let Ok(ip) = ip.parse::<IpAddr>() {
        if ip.is_loopback() || ip.is_multicast() || ip.is_unspecified() {
            return false;
        }

        let is_private = match ip {
            IpAddr::V4(ipv4) => ipv4.is_private(),
            IpAddr::V6(ipv6) => {
                let segments = ipv6.segments();
                segments[0] == 0xfc00 || segments[0] == 0xfd00
            }
        };

        if print_public && !is_private {
            return true;
        }

        if print_private && is_private {
            return true;
        }
    }

    false
}

fn print_usage() {
    println!("Usage: ipgr [FLAGS]");
    println!();
    println!("FLAGS:");
    println!("  -h, --help     Prints help information");
    println!("  -f <file>      Specifies the input file to read from");
    println!("  -4             Only prints IPv4 addresses");
    println!("  -6             Only prints IPv6 addresses");
    println!("  -pub           Only prints public IP addresses");
    println!("  -priv          Only prints private IP addresses");
}
