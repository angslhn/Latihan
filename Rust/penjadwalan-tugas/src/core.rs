use crate::cli::{clear, input_option};
use crate::menu::{view_main, view_scheduling};
use crate::storage::{view, add, edit, delete};

pub fn main_menu() -> u8 {
    clear();
    view_main();
    
    let selected: u8 = input_option("Pilih Menu", 2);

    if selected == 1 {
        clear();
        view_scheduling();

        let menu_selected: u8 = input_option("Pilih Menu", 5);

        return menu_selected;
    } else {
        return 2;
    }
}

pub fn scheduling_menu(selected: &u8) {
    match selected {
        1 => { view(); }, 
        2 => { add(); }, 
        3 => { edit(); }, 
        4 => { delete(); }, 
        _ => {} 
    }
} 