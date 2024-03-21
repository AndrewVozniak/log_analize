use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let log_path = "src/data/log.txt";

    let file = File::open(log_path).unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    let mut ip_count: HashMap<String, i32> = HashMap::new();
    let mut url_count: HashMap<String, i32> = HashMap::new();
    let mut status_count: HashMap<String, i32> = HashMap::new();
    let mut method_count: HashMap<String, i32> = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" ").collect();
        for i in 0..parts.len() {
            if i == 0 {
                let ip = parts[i];
                let count = ip_count.entry(ip.to_string()).or_insert(0);
                *count += 1;
            }
            if i == 6 {
                let url = parts[i];
                let count = url_count.entry(url.to_string()).or_insert(0);
                *count += 1;
            }
            if i == 8 {
                let status = parts[i];
                let count = status_count.entry(status.to_string()).or_insert(0);
                *count += 1;
            }
            if i == 5 {
                let method = parts[i];
                let count = method_count.entry(method.to_string()).or_insert(0);
                *count += 1;
            }
        }
    }

    println!("IP count: {:?}", ip_count);
    println!("URL count: {:?}", url_count);
    println!("Status count: {:?}", status_count);
    println!("Method count: {:?}", method_count);
}
