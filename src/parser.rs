use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!(
        "Please call \"{}\" at the number \"{}\"",
        p.name, p.phones[0]
    );

    Ok(())
}

pub fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

pub fn untyped_error() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name" "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

pub fn grab_from_file_untyped() -> Result<()> {
    // Grab JSON file
    let file_path = "data/test.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&contents)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    Ok(())
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Theme {
    colors: HashMap<String, String>,
    space: Vec<i32>,
    font_sizes: Vec<i32>,
    fonts: HashMap<String, String>,
    font_weights: HashMap<String, i32>,
    line_heights: HashMap<String, f32>,
    breakpoints: HashMap<String, String>,
    animation: HashMap<String, String>,
    gradients: HashMap<String, String>,
}

pub fn get_colors() -> Result<()> {
    // Grab JSON file
    let file_path = "data/test-1.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let p: Theme = serde_json::from_str(&contents)?;
    // Get a single value
    println!("Black: {:?}", p.colors.get("black"));

    // Loop over all the colors
    for (key, color) in p.colors {
        // Create the custom property
        let custom_property = format!("--colors-{}", key);
        let css_rule = format!("{}: {};", &custom_property, color);

        // @TODO: Export a CSS stylesheet file (or return CSS)
        println!("{}", css_rule);
        //stylesheet.push(css_rule);

        // Add the custom property
        //theme_tokens.colors.insert(key, custom_property);
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Aya {
    id: i32,              // "id" : 1,
    jozz: i32,            // "jozz" : 1,
    page: String,         // "page" : "1",
    sura_no: i32,         // "sura_no" : 1,
    sura_name_en: String, // "sura_name_en" : "Al-Fātiḥah",
    line_start: i32,      // "line_start" : 3,
    line_end: i32,        // "line_end" : 3,
    aya_no: i32,          // "aya_no" : 1,
    aya_text: String,     // "aya_text" : "اِ۬لْحَمْدُ لِلهِ رَبِّ اِ۬لْعَٰلَمِينَ ١"
}

pub fn get_ayas() -> Result<()> {
    // Grab JSON file
    let file_path = "data/warshData_v10.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let ayas: Vec<Aya> = serde_json::from_str(&contents)?;

    println!("ayas count: {}", ayas.len());
    println!("last page: {}", ayas.last().unwrap().page);
    println!("last id: {}", ayas.last().unwrap().id);
    let mut word_count = 0;
    for aya in ayas {
        let words: Vec<&str> = aya.aya_text.split_whitespace().collect();
        word_count += words.len() - 1;
    }
    println!("words: {}", word_count);

    Ok(())
}
