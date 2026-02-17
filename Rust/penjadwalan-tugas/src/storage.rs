use std::thread::{sleep};
use std::time::{Duration};

use crate::cli::{clear, input_int, input_scheduling};
use crate::json::{Scheduling, read_json, write_json};
use crate::menu::view_data;

pub fn view() {
  clear();

  println!(" ===============");
  println!(" Jadwal Kegiatan");
  println!(" ===============");

  let schedulings: Vec<Scheduling> = read_json().unwrap_or_else(|_| Vec::new());

  if schedulings.len() == 0 {
    println!("\n > Jadwal belum tersedia!");

    sleep(Duration::from_millis(2000));
    
    return;
  }
  
  let mut index: u32 = 0;
  
  for scheduling in &schedulings {
    index += 1;
    
    view_data(index, scheduling);
  }
  
}

pub fn add() {
  clear();

  println!(" ======================");
  println!(" Tambah Jadwal Kegiatan");
  println!(" ======================");
  
  let scheduling = input_scheduling();

  let mut schedulings = read_json().unwrap_or_else(|_| Vec::new());

  schedulings.push(scheduling);

  if let Err(_e) = write_json(&schedulings) {
    println!(" Gagal menyimpan kegiatan")
  } else {
    println!("\n > Berhasil disimpan!")
  }
}

pub fn edit() {
  clear();

  println!(" =================");
  println!(" == Edit Jadwal ==");
  println!(" =================\n");

  let mut schedulings: Vec<Scheduling> = read_json().unwrap_or_else(|_| Vec::new());

  if schedulings.len() == 0 {
    println!(" > Jadwal belum tersedia!");

    sleep(Duration::from_millis(2000));
    
    return;
  }

  let mut index: u32 = 0;

  for scheduling in &schedulings {
    index += 1;

    view_data(index, scheduling);
  }

  println!(" == Pilih Jadwal ==");

  let selected = input_int("Jadwal Dipilih", 1, schedulings.len() as u8);

  println!("");

  let scheduling = input_scheduling();

  schedulings[(selected - 1) as usize] = scheduling;

  if let Err(_e) = write_json(&schedulings) {
    println!("\n > Jadwal gagal di edit!");
  } else {
    println!("\n > Jadwal berhasil di edit!");
  }

  sleep(Duration::from_millis(2000));
}

pub fn delete() {}