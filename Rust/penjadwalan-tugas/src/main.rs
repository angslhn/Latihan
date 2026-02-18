mod cli;
mod menu;
mod core;
mod json;
mod utils;

use cli::{clear, input_option}; 
use core::{view, add, edit, delete};
use menu::{view_main, view_scheduling};

fn main() {
    let mut selected: u8;

    loop {
        clear();
        view_main();
        
        selected = input_option("Pilih Menu", 2);

        if selected == 1 {
            loop {   
                clear();
                view_scheduling();

                selected = input_option("Pilih Menu", 5);

                match selected {
                    1 => { view(); }, 
                    2 => { add(); }, 
                    3 => { edit(); }, 
                    4 => { delete(); }, 
                    _ => { break; } 
                }
            }
        } else {
            break;
        }
    }
}