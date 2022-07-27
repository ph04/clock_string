#![allow(unused_imports)]

use chrono::prelude::*;
use std::{thread, time::Duration};
use clock_string::clock::{clockify, HMSTime, clockify_with_seconds};

fn main() {
    // loop {
        println!("{}", clockify_with_seconds(HMSTime::from(Local::now().time()), '█', ' '));
        // thread::sleep(Duration::from_millis(1000));
        // clearscreen::clear().expect("Failed to clear screen");
    // }
}
