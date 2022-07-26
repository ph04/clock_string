#![allow(unused_imports)]

use chrono::prelude::*;
use clock_string::clock::{clockify, HMSTime, clockify_with_seconds};

fn main() {
    println!("{}", clockify_with_seconds(HMSTime::from(Local::now().time()), 'X'));
}
