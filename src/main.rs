use std::io::{self, Write};
use std::fs;
use hanifx::{encode, decode};
use colored::*; // রঙের জন্য

fn main() {
    loop {
        println!("{}", "=====================".bright_blue().bold());
        println!("{}", " Hanifx Encoder / Decoder ".white().on_bright_blue().bold());
        println!("{}", "=====================".bright_blue().bold());
        println!("{}", "1) Encode Text".green());
        println!("{}", "2) Decode Text".green());
        println!("{}", "3) Encode File".green());
        println!("{}", "4) Decode File".green());
        println!("{}", "5) Exit".red());

        print!("{}", "Enter choice: ".yellow());
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
                    None => eprintln!("{}", "Decoding failed! Wrong key or data.".red()),
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
                    None => eprintln!("{}", "Decoding failed! Wrong key or data.".red()),
                }
            }
            "5" => {
                println!("{}", "Goodbye!".bright_magenta().bold());
                break;
            }
            _ => {
                eprintln!("{}", "Invalid choice, try again.".red());
            }
        }
    }
}

/// Prompt for text and key
fn get_text_and_key() -> (String, String) {
    let mut text = String::new();
    print!("{}", "Enter text: ".cyan());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();

    let mut key = String::new();
    print!("{}", "Enter key: ".cyan());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).unwrap();

    (text.trim().to_string(), key.trim().to_string())
}

/// Prompt for file and key
fn get_file_and_key() -> (String, String) {
    let mut filename = String::new();
    print!("{}", "Enter file path: ".cyan());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut filename).unwrap();

    let mut key = String::new();
    print!("{}", "Enter key: ".cyan());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).unwrap();

    (filename.trim().to_string(), key.trim().to_string())
}

/// Ask user where to save, print or file
fn save_or_print(content: &str) {
    println!("{}", "\nWhat do you want to do with the output?".bright_blue().bold());
    println!("{}", "1) Print on screen".green());
    println!("{}", "2) Save to file".green());
    print!("{}", "Enter choice: ".yellow());
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => {
            println!("{}", "\nOutput:\n".bright_blue());
            println!("{}", content.bright_green());
        }
        "2" => {
            let mut filepath = String::new();
            print!("{}", "Enter file path to save: ".cyan());
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut filepath).unwrap();
            let filepath = filepath.trim();

            fs::write(filepath, content).expect("Failed to write file");
            println!("{}", format!("Saved successfully to {}", filepath).bright_green());
        }
        _ => {
            println!("{}", "Invalid choice, printing on screen by default.".red());
            println!("{}", "\nOutput:\n".bright_blue());
            println!("{}", content.bright_green());
        }
    }
      }
