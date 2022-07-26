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

pub fn clockify(hms_time: HMSTime, character: char) -> String {
    clockify_internals::<5>(hms_time, character)
}

pub fn clockify_with_seconds(hms_time: HMSTime, character: char) -> String {
    clockify_internals::<8>(hms_time, character)
}

fn clockify_internals<const D: usize>(hms_time: HMSTime, character: char) -> String {
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

    let mut result_str = String::new();

    for row_index in 0..5 {
        for digit in result {
            result_str.push_str(digit[row_index].iter().map(|c| if *c { character } else { ' ' }).collect::<String>().as_str())
        }

        result_str.push('\n');
    }

    result_str
}
