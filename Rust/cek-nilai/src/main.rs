use std::io;
use std::io::Write;

fn main() {
    menu();

    let selected = input("Pilih Menu");

    process(&selected);
}

fn process(menu: &str) {
    if menu == "1" {
        let value: u8 = input("Nilai").parse().unwrap_or(0);

        let information = check(value);

        println!(" ---------------------------");
        println!(" {}", information);
        println!(" ---------------------------");
    } else {
        println!(" > Anda Keluar");
    }
}

fn check(nilai: u8) -> &'static str {
    match nilai {
        91..=100 => " > Selamat Anda Mendapat Nilai A",
        81..=90 => " > Anda Mendapat Nilai B",
        71..=80 => " > Anda Mendapat Nilai C",
        61..=70 => " > Anda Mendapat Nilai D",
        0..=60 => " > Anda Mendapat Nilai E",
        _ => " > Nilai Tidak Valid"
    }
}

fn menu() {
    println!(" != Cek Nilai =!");
    println!("  1. Mulai");
    println!("  2. Keluar");
}

fn input(label: &str) -> String {
    print!(" {} = ", label);

    io::stdout().flush().unwrap();

    let mut buffer: String = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        return String::from("");
    };

    buffer.trim().to_string()
}
