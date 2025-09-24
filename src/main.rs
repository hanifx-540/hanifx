use std::io::{self, Write};
use std::fs;
use hanifx::{encode, decode};

fn main() {
    loop {
        println!("\n=====================");
        println!("hanifx Encoder/Decoder");
        println!("=====================");
        println!("1) Encode Text");
        println!("2) Decode Text");
        println!("3) Encode File");
        println!("4) Decode File");
        println!("5) Exit");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                let (text, key) = get_text_and_key();
                let encoded = encode(&text, &key);
                save_or_print(&encoded);
            }
            "2" => {
                let (text, key) = get_text_and_key();
                match decode(&text, &key) {
                    Some(decoded) => save_or_print(&decoded),
                    None => println!("Decoding failed! Wrong key or data."),
                }
            }
            "3" => {
                let (filename, key) = get_file_and_key();
                let content = fs::read_to_string(&filename).expect("Could not read file");
                let encoded = encode(&content, &key);
                save_or_print(&encoded);
            }
            "4" => {
                let (filename, key) = get_file_and_key();
                let content = fs::read_to_string(&filename).expect("Could not read file");
                match decode(&content, &key) {
                    Some(decoded) => save_or_print(&decoded),
                    None => println!("Decoding failed! Wrong key or data."),
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, try again.");
            }
        }
    }
}

/// Prompt for text and key
fn get_text_and_key() -> (String, String) {
    let mut text = String::new();
    print!("Enter text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();

    let mut key = String::new();
    print!("Enter key: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).unwrap();

    (text.trim().to_string(), key.trim().to_string())
}

/// Prompt for file and key
fn get_file_and_key() -> (String, String) {
    let mut filename = String::new();
    print!("Enter file path: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut filename).unwrap();

    let mut key = String::new();
    print!("Enter key: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).unwrap();

    (filename.trim().to_string(), key.trim().to_string())
}

/// Ask user where to save, print or file
fn save_or_print(content: &str) {
    println!("\nWhat do you want to do with the output?");
    println!("1) Print on screen");
    println!("2) Save to file");
    print!("Enter choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => {
            println!("\nOutput:\n{}", content);
        }
        "2" => {
            let mut filepath = String::new();
            print!("Enter file path to save: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filepath).unwrap();
            let filepath = filepath.trim();

            fs::write(filepath, content).expect("Failed to write file");
            println!("Saved successfully to {}", filepath);
        }
        _ => {
            println!("Invalid choice, printing on screen by default.");
            println!("\nOutput:\n{}", content);
        }
    }
                                     }
