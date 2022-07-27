pub const COLON: [[bool; 3]; 5] = [
    [false, false, false],
    [false, true, false],
    [false, false, false],
    [false, true, false],
    [false, false, false],
];

pub const ZERO: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, true],
    [true, false, true],
    [true, false, true],
    [true, true, true],
];

pub const ONE: [[bool; 3]; 5] = [
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [false, true, false],
    [false, true, false],
];

pub const TWO: [[bool; 3]; 5] = [
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [true, false, false],
    [true, true, true],
];
pub const THREE: [[bool; 3]; 5] = [
    [true, true, true],
    [false, false, true],
    [true, true, true],
    [false, false, true],
    [true, true, true],
];
pub const FOUR: [[bool; 3]; 5] = [
    [true, false, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true],
];
pub const FIVE: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [false, false, true],
    [true, true, true],

];
pub const SIX: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, false],
    [true, true, true],
    [true, false, true],
    [true, true, true],
];
pub const SEVEN: [[bool; 3]; 5] = [
    [true, true, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
    [false, false, true],
];
pub const EIGHT: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [true, false, true],
    [true, true, true],
];
pub const NINE: [[bool; 3]; 5] = [
    [true, true, true],
    [true, false, true],
    [true, true, true],
    [false, false, true],
    [false, false, true],
];

pub fn to_ascii_digit(digit: u32) -> [[bool; 3]; 5] {
    match digit {
        0 => ZERO,
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        10 => COLON,
        _ => unreachable!(),
    }
}

pub struct DigitsIterator {
    to_divide: i32,
    curr_number: u32,
    leading_zero: bool,
}

impl DigitsIterator {
    pub fn new(number: u32) -> Self {
        if number == 0 {
            return Self {
                to_divide: 0,
                curr_number: number,
                leading_zero: true,
            }
        }

        let mut to_divide = -1;

        let mut n = number;

        while n != 0 {
            n /= 10;
            to_divide += 1;
        }

        Self {
            to_divide,
            curr_number: number,
            leading_zero: to_divide == 0,
        }
    }
}

impl Default for DigitsIterator {
    fn default() -> Self {
        Self {
            to_divide: -1,
            curr_number: 0,
            leading_zero: false
        }
    }
}

impl Iterator for DigitsIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.leading_zero {
            self.leading_zero = false;
            Some(0)
        } else if self.to_divide != -1 {
            let digit = self.curr_number / 10_u32.pow(self.to_divide.try_into().unwrap());
            self.curr_number %= 10_u32.pow(self.to_divide.try_into().unwrap());
            self.to_divide -= 1;

            Some(digit)
        } else {
            None
        }
    }
}

pub struct Digits {
    number: u32
}

impl Digits {
    pub fn new(number: u32) -> Self {
        Self { number }
    }
}

impl IntoIterator for Digits {
    type Item = u32;
    type IntoIter = DigitsIterator;

    fn into_iter(self) -> Self::IntoIter {
        DigitsIterator::new(self.number)
    }
}
