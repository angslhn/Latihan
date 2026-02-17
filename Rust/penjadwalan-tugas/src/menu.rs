use crate::{json::Scheduling, utils::duration};

pub fn view_main() {
    println!(" == Menu Utama ===");
    println!("  1. Mulai");
    println!("  2. Keluar");
    println!(" =================")
}

pub fn view_scheduling() {
    println!(" == Menu Penjadwalan ===");
    println!("  1. Lihat Jadwal");
    println!("  2. Tambah Jadwal");
    println!("  3. Edit Jadwal");
    println!("  4. Hapus Jadwal");
    println!("  5. Keluar");
    println!(" =======================")
}

pub fn view_data(index: u32, scheduling: &Scheduling) {
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