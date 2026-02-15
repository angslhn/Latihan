use crate::cli::{clear, input_int, input_str};
use crate::json::{Scheduling, SchedulingTime, read_json, write_json};
use crate::utils::{duration};

pub fn view() {
  clear();

  println!(" ===============");
  println!(" Jadwal Kegiatan");
  println!(" ===============");
  
  match read_json() {
    Ok(schedulings) => {
      
      let mut index: u32 = 0;
      
      for scheduling in &schedulings {
        index += 1;
        
        let space = " ".repeat(index.to_string().len());

        let start_time = &scheduling.start_time;
        let end_time = &scheduling.end_time;
        let [hour, minute] = duration(start_time, end_time);

        let start_time_formatted = format!("{}:{:02}", start_time.hour, start_time.minute);
        let end_time_formatted = format!("{}:{:02}", end_time.hour, end_time.minute);
        let duration_formatted = if hour > 0 && minute > 0 {
            format!("{} Jam {} Menit", hour, minute)
        } else if hour > 0 && minute == 0 {
            format!("{} Jam", hour)
        } else {
          format!("{} Menit", minute)
        };
          
        println!(" [{}]. Kegiatan : {}", index, scheduling.event);
        println!("    {} Waktu    : {} - {}", space, start_time_formatted, end_time_formatted);
        println!("    {} Durasi   : {}\n", space, duration_formatted);
      }
    },
    Err(e) => {
      println!(" {}", e)
    }
  }
}

pub fn add() {
  clear();

  println!(" ======================");
  println!(" Tambah Jadwal Kegiatan");
  println!(" ======================");

  println!(" [Informasi Kegiatan]");
  let event_name = input_str(" Nama");

  println!("\n [Waktu Mulai]");
  let start_hour = input_int(" Jam", 0, 24);
  let start_minute = input_int(" Menit", 0, 59);

  println!("\n [Waktu Selesai]");
  let end_hour = input_int(" Jam", 0, 24);
  let end_minute = input_int(" Menit", 0, 59);
  
  let scheduling = Scheduling {
    event: String::from(&event_name),
    start_time: SchedulingTime { hour: start_hour, minute: start_minute },
    end_time: SchedulingTime { hour: end_hour, minute: end_minute }
  };

  let mut schedulings = read_json().unwrap_or_else(|_| Vec::new());

  schedulings.push(scheduling);

  if let Err(_e) = write_json(&schedulings) {
    println!(" Gagal menyimpan kegiatan")
  } else {
    println!("\n > Berhasil disimpan!")
  }
}

pub fn edit() {}

pub fn delete() {}