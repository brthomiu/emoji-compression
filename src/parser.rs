use serde::Serialize; // Import serialization utilities
use std::collections::HashMap;
use std::fs; // Import the file system module // Import the HashMap data structure

// Define a structure to hold character data
#[derive(Serialize)]
struct Character {
    unicode: String,     // Field to store the Unicode character
    description: String, // Field to store the description of the character
}

pub fn parser() {
    let contents = fs::read_to_string("src/emojis.txt").expect("Error");

    let mut characters: HashMap<String, String> = HashMap::new();

    // println!("{}", contents);

    for line in contents.lines() {
        if !line.trim().is_empty() && !line.trim().starts_with('#') {
            let parts: Vec<&str> = line.split('#').collect();
            if parts.len() == 2 {
                let key = parts[0].split(';').next().map(str::trim).unwrap_or("");
                let value = parts[1].split(':').next().map(str::trim).unwrap_or("");
                if !key.is_empty() && !value.is_empty() {
                    characters.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    let json_data = serde_json::to_string_pretty(&characters).expect("Failed to serialize to JSON");

    fs::write("emojis.json", json_data).expect("Failed to write JSON to file");

    println!("Parsing completed.");
}
