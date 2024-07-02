use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {

    
    let mut finger_prints: HashMap<String, Vec<String>> = HashMap::new();
    let arguments = io::stdin();
    // Locks this handle to the standard input stream, returning a readable guard.
    // lines Returns an iterator over the lines of this reader.
    let lines = arguments.lock().lines();
    for line in lines {
        if let Ok(line) = line {
            let trimmed_input = line.trim().to_string();
            if trimmed_input.is_empty() {
                // if input is empty
                continue;
            }
            // .split_whitespace() ->
            // Splits a string slice by whitespace.The iterator returned will return string slices that are sub-slices
            // of the original string slice, separated by any amount of whitespace.
            let parts: Vec<&str> = trimmed_input.split_whitespace().collect();
          
            if parts.len() < 2 {
                eprintln!("Incomplete pair: {}", trimmed_input);
                continue; // Skip incomplete pairs
            }
                let entry = finger_prints.entry(parts[0].to_string()).or_insert(Vec::new());
                entry.push(parts[1..].join(" ")); // push on to the vector
        }
    }
    // for every key in finger_prints it will loop through
    // then as it loops through the hashmap it will loop through
    // the values stored in the vector values
    for (_key, values) in &finger_prints {
        for value in values {
            println!("{}", value);
        }
        // prints a new line after it loops through all values for
        // a specific key
        println!("");
    }
}
