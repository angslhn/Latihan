mod cli;
mod menu;
mod core;
mod json;
mod storage;
mod utils;

use core::{main_menu, scheduling_menu};

fn main() {
    loop {
        let selected: u8 = main_menu();
        
        scheduling_menu(&selected);
        
        if [2, 5].contains(&selected) {
            println!("\n > Keluar dari program");
            break
        }
    }
}