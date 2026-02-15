use crate::json::SchedulingTime;

pub fn duration(start_time: &SchedulingTime, end_time: &SchedulingTime) -> [u8; 2] {
  let start_total = (start_time.hour as u32 * 60) + start_time.minute as u32;
  let end_total = (end_time.hour as u32 * 60) + end_time.minute as u32;

  let diffrence_minutes = if end_total >= start_total {
    end_total - start_total
  } else {
    1440 + end_total - start_total
  };

  [
    (diffrence_minutes / 60) as u8,
    (diffrence_minutes % 60) as u8
  ]
}