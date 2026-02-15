use std::io;
use std::io::Write;
use std::process::Command;

pub fn clear() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/c", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }

    print!("\x1b[H\x1b[2J\x1b[3J");

    let _ = io::stdout().flush();
}

pub fn reset() {
    print!("\x1b[1A\x1b[2K");

    io::stdout().flush().unwrap();
}

pub fn input_option(label: &str, option: u8) -> u8 {
    let mut buffer = String::new();

    loop {  
        print!(" {} = ", label);

        io::stdout().flush().unwrap();

        buffer.clear();

        if io::stdin().read_line(&mut buffer).is_err() {
            continue;
        }

        let value = buffer.trim().parse().unwrap_or(0);
        
        if value >= 1 && value <= option {
            return value;
        }

        reset();
    }
}

pub fn input_int(label: &str, min: u8, max: u8) -> u8 {
    let mut buffer = String::new();

    loop {  
        print!(" {} = ", label);

        io::stdout().flush().unwrap();

        buffer.clear();

        if io::stdin().read_line(&mut buffer).is_err() {
            continue;
        }

        let value = buffer.trim().parse().unwrap_or(0);
        
        if value >= min && value <= max {
            return value;
        }

        reset();
    }
}

pub fn confirm(label: &str) -> String {
    let mut buffer = String::new();

    loop {  
        print!(" {} (Y/N) = ", label);

        io::stdout().flush().unwrap();

        buffer.clear();

        if io::stdin().read_line(&mut buffer).is_err() {
            continue;
        }

        let value = buffer.trim().to_uppercase();
        
        if ["Y", "N"].contains(&value.as_str()) {
            return value;
        }

        reset();
    }
}

pub fn input_str(label: &str) -> String {
    print!(" {} = ", label);

    io::stdout().flush().unwrap();

    let mut buffer = String::new();

    let _ = io::stdin().read_line(&mut buffer);

    return buffer.trim().to_string();
}