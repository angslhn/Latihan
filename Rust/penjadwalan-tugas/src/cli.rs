use std::io::{self, Write};
use std::process::Command;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::json::{Scheduling, SchedulingTime};

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
        print!("  {} = ", label);

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
        print!("  {} = ", label);

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

pub fn input_int_with_esc(label: &str, min: u8, max: u8) -> Option<u8> {
    print!("  {} = ", label);

    io::stdout().flush().unwrap();

    let mut buffer = String::new();

    enable_raw_mode().unwrap();

    loop {  
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => {
                            disable_raw_mode().unwrap();
                            println!();
                            return None;
                        }
                        
                        KeyCode::Enter => {
                            disable_raw_mode().unwrap();
                            println!();
                            
                            let value = buffer.trim().parse::<u8>().unwrap_or(0);

                            if value >= min && value <= max {
                                return Some(value);
                            } else {
                                reset();
                                return input_int_with_esc(label, min, max);
                            }
                        }

                        KeyCode::Backspace => {
                            if !buffer.is_empty() {
                                buffer.pop();
                                print!("\x08 \x08"); 
                                io::stdout().flush().unwrap();
                            }
                        }

                        KeyCode::Char(c) => {
                            if c.is_digit(10) {
                                buffer.push(c);
                                print!("{}", c);
                                io::stdout().flush().unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            } 
        }
    }
}

pub fn input_str(label: &str) -> String {
    print!("  {} = ", label);

    io::stdout().flush().unwrap();

    let mut buffer = String::new();

    let _ = io::stdin().read_line(&mut buffer);

    return buffer.trim().to_string();
}

pub fn input_enter() -> String {
    print!(" Klik enter untuk keluar...");

    io::stdout().flush().unwrap();

    let mut buffer = String::new();

    let _ = io::stdin().read_line(&mut buffer);

    return buffer.trim().to_string();
}

pub fn input_scheduling() -> Scheduling {
  println!("  [Informasi Kegiatan]");
  let event_name = input_str(" Nama");

  println!("\n  [Waktu Mulai]");
  let start_hour = input_int(" Jam", 0, 24);
  let start_minute = input_int(" Menit", 0, 59);

  println!("\n  [Waktu Selesai]");
  let end_hour = input_int(" Jam", 0, 24);
  let end_minute = input_int(" Menit", 0, 59);

  return Scheduling {
    event: String::from(&event_name),
    start_time: SchedulingTime { hour: start_hour, minute: start_minute },
    end_time: SchedulingTime { hour: end_hour, minute: end_minute }
  };
} 