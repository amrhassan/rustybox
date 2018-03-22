use enum_traits::*;
use std::cmp::Ordering;

// 19

pub fn count() {

    let count = Date::from_without_weekday(1, 1, 1901)
        .into_iter()
        .take_while(|d| d.year < 2001)
        .filter(|d| d.day == 1 && d.day_of_week == DayOfWeek::Sunday)
        .count();

    println!(
        "The number of first-of-the-month sundays from 1-1-1901 to 31-12-2000 is {}",
        count
    )
}

#[derive(EnumIndex, EnumToIndex, EnumFromIndex, EnumEnds, Debug, Copy, Clone, Eq, PartialEq)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    fn next(&self) -> DayOfWeek {
        DayOfWeek::from_index(self.index() + 1).unwrap_or(DayOfWeek::first())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Date {
    pub day: u8,   // 1..31
    pub month: u8, // 1..12
    pub year: u16,
    pub day_of_week: DayOfWeek,
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = if self.year != other.year {
            self.year.cmp(&other.year)
        } else {
            if self.month != other.month {
                self.month.cmp(&other.month)
            } else {
                self.day.cmp(&other.day)
            }
        };
        Some(ord)
    }
}

impl Date {
    pub fn from(day: u8, month: u8, year: u16, day_of_week: DayOfWeek) -> Date {
        Date {
            day,
            month,
            year,
            day_of_week,
        }
    }

    pub fn from_without_weekday(day: u8, month: u8, year: u16) -> Date {
        assert!(year >= 1900);
        Date::from(1, 1, 1900, DayOfWeek::Monday)
            .into_iter()
            .find(|d| d.year == year && d.month == month && d.day == day)
            .expect("Failed to find a date on the timeline")
    }

    pub fn next(&self) -> Date {
        if (self.day + 1) > days_in_month(self.month, self.year as u32) {
            if self.month == 12 {
                Date {
                    day: 1,
                    month: 1,
                    year: self.year + 1,
                    day_of_week: self.day_of_week.next(),
                }
            } else {
                Date {
                    day: 1,
                    month: self.month + 1,
                    year: self.year,
                    day_of_week: self.day_of_week.next(),
                }
            }
        } else {
            Date {
                day: self.day + 1,
                month: self.month,
                year: self.year,
                day_of_week: self.day_of_week.next(),
            }
        }
    }

    pub fn into_iter(self) -> DateIter {
        DateIter(self)
    }
}

pub struct DateIter(Date);

impl Iterator for DateIter {
    type Item = Date;

    fn next(&mut self) -> Option<Date> {
        let n = self.0.next();
        let c = self.0;
        self.0 = n;
        Some(c)
    }
}

fn days_in_month(month: u8, year: u32) -> u8 {
    match month {
        1 => 31,
        2 => if is_leap_year(year) {
            29
        } else {
            28
        },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        v => panic!("Invalid month value: {}", v),
    }
}

fn is_leap_year(year: u32) -> bool {
    divides(year, 4) || (divides(year, 100) && divides(year, 400))
}

fn divides(x: u32, y: u32) -> bool {
    x % y == 0
}
