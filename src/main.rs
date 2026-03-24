use rpassword::read_password;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct PasswordEntry {
    site: String,
    username: String,
    password: String,
}
enum MenuOption {
    Add,
    View,
    Search,
    Exit,
    Invalid,
}

fn parse_choice(input: &str) -> MenuOption {
    match input {
        "1" => MenuOption::Add,
        "2" => MenuOption::View,
        "3" => MenuOption::Search,
        "4" => MenuOption::Exit,
        _ => MenuOption::Invalid,
    }
}

fn add_entry(entries: &mut Vec<PasswordEntry>) {
    let mut site = String::new();
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter site:");
    std::io::stdin().read_line(&mut site).unwrap();

    println!("Enter username:");
    std::io::stdin().read_line(&mut username).unwrap();

    println!("Enter password:");
    std::io::stdin().read_line(&mut password).unwrap();
    let key = "THCY3004"; // master key
    let entry = PasswordEntry {
        site: site.trim().to_string(),
        username: username.trim().to_string(),
        password: encrypt(password.trim(), key),
    };

    entries.push(entry);
}

fn view_entries(entries: &Vec<PasswordEntry>) {
   

    println!("Enter admin key:");
    let key = read_password().unwrap();

    for entry in entries {
        if !key.is_empty() {
            match decrypt(&entry.password, &key) {
                Some(decrypted) => {
                    println!("{} | {} | {}", entry.site, entry.username, decrypted);
                }
                None => println!("Wrong key for {}", entry.site),
            }
        } else {
            println!("{} | {} | [ENCRYPTED]", entry.site, entry.username);
        }
    }
}

fn search_entry<'a>(
    entries: &'a Vec<PasswordEntry>,
    site: &str
) -> Option<&'a PasswordEntry> {
    for entry in entries {
        if entry.site == site {
            return Some(entry);
        }
    }
    None
}

use base64::{encode, decode};

fn encrypt(password: &str, key: &str) -> String {
    let combined = format!("{}{}", key, password);
    encode(combined)
}

fn decrypt(encoded: &str, key: &str) -> Option<String> {
    let decoded = decode(encoded).ok()?;
    let decoded_str = String::from_utf8(decoded).ok()?;

    if decoded_str.starts_with(key) {
        Some(decoded_str[key.len()..].to_string())
    } else {
        None
    }
}

fn load_entries() -> Vec<PasswordEntry> {
    let data = fs::read_to_string("passwords.json");

    match data {
        Ok(content) => serde_json::from_str(&content).unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_entries(entries: &Vec<PasswordEntry>) {
    let json = serde_json::to_string(entries).unwrap();
    fs::write("passwords.json", json).expect("Unable to save file");
}

fn main() {
    let mut entries = load_entries();

    loop {
        println!("\n--- Password Manager ---");
        println!("1. Add password");
        println!("2. View passwords");
        println!("3. Search");
        println!("4. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        let action = parse_choice(choice.trim());

        match action {
            MenuOption::Add => {
            add_entry(&mut entries);
            save_entries(&entries);
            }
            MenuOption::View => view_entries(&entries),
            MenuOption::Search => {
                println!("Enter site to search:");
                let mut site = String::new();
                std::io::stdin().read_line(&mut site).unwrap();

                match search_entry(&entries, site.trim()) {
                    Some(entry) => println!("{:?}", entry),
                    None => println!("Not found"),
                }
            }
            MenuOption::Exit => break,
            MenuOption::Invalid => println!("Invalid choice"),
        }
    }
}