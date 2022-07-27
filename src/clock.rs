use chrono::{NaiveTime, Timelike};
use crate::digit::{Digits, to_ascii_digit, DigitsIterator};
use std::iter;

pub struct HMSTime {
    hour: u32,
    minute: u32,
    second: u32,
}

impl From<NaiveTime> for HMSTime {
    fn from(naive_time: NaiveTime) -> Self {
        Self {
            hour: naive_time.hour(),
            minute: naive_time.minute(),
            second: naive_time.second(),
        }
    }
}

impl HMSTime {
    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn second(&self) -> u32 {
        self.second
    }
}

pub fn clockify(hms_time: HMSTime, foreground: char, background: char) -> String {
    clockify_internals::<5>(hms_time, foreground, background)
}

pub fn clockify_with_seconds(hms_time: HMSTime, foreground: char, background: char) -> String {
    clockify_internals::<8>(hms_time, foreground, background)
}

fn clockify_internals<const D: usize>(hms_time: HMSTime, foreground: char, background: char) -> String {
    let mut result = [[[false; 3]; 5]; D];

    Digits::new(hms_time.hour())
        .into_iter()
        .chain(iter::once(10))
        .chain(Digits::new(hms_time.minute()))
        .chain(if D == 8 { Some(10) } else { None })
        .chain(if D == 8 { Digits::new(hms_time.second()).into_iter() } else { DigitsIterator::default() })
        .map(to_ascii_digit)
        .enumerate()
        .for_each(|(idx, array)| result[idx] = array);

    (0..5)
        .flat_map(|row_index| {
            result
                .iter()
                .flat_map(move |digit| 
                    digit[row_index]
                        .map(|c| if c { foreground } else { background }))
                        .chain(Some('\n'))
        })
        .collect::<String>()
}
