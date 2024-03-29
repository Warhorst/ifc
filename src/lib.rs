/// Represents an International Fixed Calendar date
pub struct IFC {
    pub day: u8
}

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
    December
}

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
    YearDay
}