use crate::Month::{April, August, December, February, January, July, June, March, May, November, October, September, Sol};
use crate::WeekDay::{Friday, LeapDay, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday, YearDay};

/// Represents an International Fixed Calendar date. The date is monday based, so the first of each month is always monday.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct IFCDate {
    pub day: usize,
    pub week_day: WeekDay,
    pub month: Month,
    pub year: usize,
}

impl IFCDate {
    pub fn new(day: usize, month: usize, year: usize) -> Self {
        let month = Month::from_number(month);

        if month == June && Self::is_leap_year(year) && !(1..=29).contains(&day) {
            panic!("invalid day");
        } else if month == December && !(1..=29).contains(&day) {
            panic!("invalid day")
        } else if !(1..=29).contains(&day) {
            panic!("invalid day")
        }

        let week_day = WeekDay::from_day_and_month(day, month);

        IFCDate {
            day,
            week_day,
            month,
            year,
        }
    }

    /// Increment this date by one day.
    pub fn increment_day(&self) -> Self {
        let mut new_date = *self;

        if self.day < 28 {
            new_date.day += 1;
        } else if self.day == 28 && self.month == June && Self::is_leap_year(self.year) {
            new_date.day += 1;
        } else if self.day == 28 && self.month == December {
            new_date.day += 1;
        } else if self.day == 29 && self.month == June && Self::is_leap_year(self.year) {
            new_date.day = 1;
            new_date.month = Sol;
        } else if self.day == 29 && self.month == December {
            new_date.day = 1;
            new_date.month = January;
            new_date.year += 1;
        } else if self.day == 28 {
            new_date.day = 1;
            new_date.month = Month::from_number(self.month.to_number() + 1);
        }

        new_date.week_day = WeekDay::from_day_and_month(new_date.day, new_date.month);

        new_date
    }

    fn is_leap_year(year: usize) -> bool {
        if year % 400 == 0 {
            true
        } else if year % 100 == 0 {
            false
        } else if year % 4 == 0 {
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    Sol,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn from_number(num: usize) -> Self {
        match num {
            1 => January,
            2 => February,
            3 => March,
            4 => April,
            5 => May,
            6 => June,
            7 => Sol,
            8 => July,
            9 => August,
            10 => September,
            11 => October,
            12 => November,
            13 => December,
            _ => panic!("invalid number {num}")
        }
    }

    fn to_number(&self) -> usize {
        match self {
            January => 1,
            February => 2,
            March => 3,
            April => 4,
            May => 5,
            June => 6,
            Sol => 7,
            July => 8,
            August => 9,
            September => 10,
            October => 11,
            November => 12,
            December => 13
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    /// the 29th of June, which is only present in leap years
    LeapDay,
    /// the 29th of December, which is the last day of the year
    YearDay,
}

impl WeekDay {
    fn from_day_and_month(day: usize, month: Month) -> Self {
        match (day, month) {
            (29, June) => LeapDay,
            (29, December) => YearDay,
            (d, _) if d % 7 == 1 => Monday,
            (d, _) if d % 7 == 2 => Tuesday,
            (d, _) if d % 7 == 3 => Wednesday,
            (d, _) if d % 7 == 4 => Thursday,
            (d, _) if d % 7 == 5 => Friday,
            (d, _) if d % 7 == 6 => Saturday,
            (d, _) if d % 7 == 0 => Sunday,
            (_, _) => panic!("invalid day {day} or month {:?}", month)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IFCDate;

    #[test]
    fn is_leap_year_works() {
        let years_expected = [
            (2023, false),
            (2024, true),
            (2000, true),
            (1940, true),
            (1900, false),
            (1800, false)
        ];

        years_expected.into_iter().for_each(|(year, expected)| assert_eq!(IFCDate::is_leap_year(year), expected))
    }

    #[test]
    fn increment_day_works() {
        let dates_expected = [
            (IFCDate::new(12, 2, 2000), IFCDate::new(13, 2, 2000)),
            (IFCDate::new(28, 2, 2000), IFCDate::new(1, 3, 2000)),
            (IFCDate::new(28, 6, 2000), IFCDate::new(29, 6, 2000)), // 2000 is a leap year, so it has a leap day in june
            (IFCDate::new(29, 6, 2000), IFCDate::new(1, 7, 2000)), // after the leap day follows the first of july
            (IFCDate::new(28, 13, 2000), IFCDate::new(29, 13, 2000)), // after the last day of december follows the year day
            (IFCDate::new(29, 13, 2000), IFCDate::new(1, 1, 2001)), // after year day follows the first day of the new year
        ];

        dates_expected.into_iter().for_each(|(date, expected)| assert_eq!(date.increment_day(), expected, "{:?} incremented by one day was not {:?}", date, expected))
    }
}