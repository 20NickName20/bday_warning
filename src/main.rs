extern crate user32;
extern crate winapi;

use std::cmp::Ordering;
use std::ptr;
use std::fs;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use chrono::Date;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use user32::MessageBoxW;
use winapi::{MB_ICONINFORMATION, MB_OK};

struct BDay<'a> {
    day: u32,
    month: u32,
    text: &'a str
}

impl BDay<'_> {
    fn new(day: u32, month: u32, text: &str) -> BDay {
        BDay {day, month, text}
    }

    fn cmp(&self, day: u32, month: u32) -> Ordering {
        if month > self.month {
            return Ordering::Greater;
        } else if month == self.month {
            if day > self.day {
                return Ordering::Greater;
            } else if day == self.day {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        } else {
            return Ordering::Less;
        }
    }
}

fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect()
}

fn main() {
    let contents = fs::read_to_string("C:/Users/User/Desktop/Дни рождения.txt")
        .expect("Should have been able to read the file");
    let local: DateTime<Local> = Local::now();
    let local_day = local.day();
    let local_month = local.month();
    println!("{local_day}.{local_month}");
    let mut bdays = Vec::new();

    for line in contents.split("\n") {
        let splitted_line: Vec<&str> = line.split(|c| c == ':' || c == '.').collect();
        bdays.push(BDay::new(splitted_line[0].parse::<u32>().unwrap(), splitted_line[1].parse::<u32>().unwrap(), splitted_line[2]));
    }

    let mut text = String::new();

    for i in 0..bdays.len() {
        if bdays[i].cmp(local_day, local_month) == Ordering::Less && bdays[(i - 1) % bdays.len()].cmp(local_day, local_month) == Ordering::Greater {
            text.push_str(bdays[i].text);
            break;
        }
    }

    let lp_text = to_wide_string(&text);
    let lp_caption = to_wide_string("Ближайший день рождения:");

    unsafe {
        MessageBoxW(ptr::null_mut(), lp_text.as_ptr(), lp_caption.as_ptr(), MB_OK | MB_ICONINFORMATION);
    }
}