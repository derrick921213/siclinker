use std::{env, fs::File, io::BufRead, io::BufReader};
#[derive(Debug)]
struct Data {
    cs: Option<String>,
    name: Option<String>,
    address: String,
    length: Option<String>,
}
impl Data {
    fn new(cs: Option<&str>, name: Option<&str>, address: &str, length: Option<&str>) -> Self {
        Data {
            cs: cs.and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            }),
            name: name.and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            }),
            address: address.to_string(),
            length: length.and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            }),
        }
    }
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <address> <file1> [file2 ...]", args[0]);
        std::process::exit(1);
    }

    let address = &args[1];
    let files = &args[2..];

    match i32::from_str_radix(address, 16) {
        Ok(start_address) => {
            // println!("Program Name: {}", args[0]);
            // println!("Starting Address: 0x{:X}", start_address);
            let mut data: Vec<Data> = Vec::new();
            let mut address_check: i32 = 0;
            let mut length_check: i32 = 0;
            for file in files {
                let reader = BufReader::new(File::open(file)?);
                for line in reader.lines() {
                    let line = line?;
                    if line.starts_with('H') {
                        let modify = line.trim_start_matches('H').trim_start();
                        let parts = modify.split_whitespace().collect::<Vec<&str>>();
                        let length = &parts[1][6..];
                        if address_check == 0 && length_check == 0 {
                            address_check = start_address;
                            length_check = i32::from_str_radix(length, 16).unwrap();
                        } else {
                            address_check += length_check;
                            length_check = i32::from_str_radix(length, 16).unwrap();
                        }
                        let start_address = &format!("{:04X}", address_check);
                        let length = &format!("{:04X}", i32::from_str_radix(length, 16).unwrap());
                        data.push(Data::new(Some(parts[0]), None, start_address, Some(length)));
                    } else if line.starts_with('D') {
                        let modify = line.trim_start_matches('D').trim_start();
                        let parts: Vec<String> = modify
                            .split_whitespace()
                            .flat_map(|part| {
                                if part.len() > 6 {
                                    let (first, second) = part.split_at(6);
                                    vec![first.to_string(), second.to_string()]
                                } else {
                                    vec![part.to_string()]
                                }
                            })
                            .collect();
                        for i in (0..parts.len()).step_by(2) {
                            let address = format!(
                                "{:04X}",
                                i32::from_str_radix(&parts[i + 1], 16).unwrap() + address_check
                            );
                            data.push(Data::new(None, Some(&parts[i]), &address, None));
                        }
                    }
                }
            }
            println!(
                "{:18}{:16}{:13}{:15}",
                "Control section", "Symbol name", "Address", "Length"
            );
            for d in data {
                println!(
                    "{: >10}{: >16}{: >14}{: >13}",
                    d.cs.unwrap_or("".to_string()),
                    d.name.unwrap_or("".to_string()),
                    d.address,
                    d.length.unwrap_or("".to_string())
                );
            }
        }
        Err(_) => {
            eprintln!("Error: The address provided is not a valid hexadecimal number.");
        }
    }
    Ok(())
}
