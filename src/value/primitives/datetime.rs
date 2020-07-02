use serde::{Deserialize, Serialize};

pub(crate) type YearNumber = i32;
pub(crate) type MonthNumber = u8;
pub(crate) type WeekNumber = u8;
pub(crate) type DayNumber = u8;
pub(crate) type DayOrdinal = u16;
pub(crate) type HourNumber = u8;
pub(crate) type MinuteNumber = u8;
pub(crate) type SecondNumber = u8;
pub(crate) type MilliNumber = u16;
pub(crate) type MicroNumber = u16;
pub(crate) type NanoNumber = u16;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeZone {
    Utc,
    Offset { hours: i16, minutes: i16 },
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Date {
    YearDay {
        year: YearNumber,
        day_in_year: DayOrdinal,
    },
    YearMonthDay {
        year: YearNumber,
        month: MonthNumber,
        day: DayNumber,
    },
    YearWeekDay {
        year: YearNumber,
        week_in_year: WeekNumber,
        day_in_week: DayNumber,
    },
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Date::YearMonthDay { year, month, day } => {
                write!(f, "{:0>4}-{:0>2}-{:0>2}", year, month, day)
            }
            Date::YearWeekDay {
                year,
                week_in_year,
                day_in_week,
            } => write!(f, "{:0>4}-W{:0>2}-{}", year, week_in_year, day_in_week),
            Date::YearDay { year, day_in_year } => write!(f, "{:0>4}-{:0>3}", year, day_in_year),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub hour: HourNumber,
    pub minute: MinuteNumber,
    pub second: SecondNumber,
    pub milli: MilliNumber,
    pub micro: MicroNumber,
    pub nano: NanoNumber,
    pub timezone: TimeZone,
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let subseconds: f64 = (self.milli as f64 / 1000.0)
            + (self.micro as f64 / 1000.0 / 1000.0)
            + (self.nano as f64 / 1000.0 / 1000.0 / 1000.0);
        writeln!(
            f,
            "{:0>2}:{:0>2}:{:0>2}.{}",
            self.hour, self.minute, self.second, subseconds
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DateTime {
    Date(Date),
    Time(Time),
    Full { date: Date, time: Time },
}

impl DateTime {
    pub fn ymd(year: YearNumber, month: MonthNumber, day: DayNumber) -> Self {
        Self::Date(Date::YearMonthDay { year, month, day })
    }

    pub fn hms(hour: HourNumber, minute: MinuteNumber, second: SecondNumber) -> Self {
        Self::hms_mmn_tz(hour, minute, second, 0, 0, 0, TimeZone::Utc)
    }

    pub fn hms_mmn(
        hour: HourNumber,
        minute: MinuteNumber,
        second: SecondNumber,
        milli: MilliNumber,
        micro: MicroNumber,
        nano: NanoNumber,
    ) -> Self {
        Self::hms_mmn_tz(hour, minute, second, milli, micro, nano, TimeZone::Utc)
    }

    pub fn hms_mmn_tz(
        hour: HourNumber,
        minute: MinuteNumber,
        second: SecondNumber,
        milli: MilliNumber,
        micro: MicroNumber,
        nano: NanoNumber,
        timezone: TimeZone,
    ) -> Self {
        Self::Time(Time {
            hour,
            minute,
            second,
            milli,
            micro,
            nano,
            timezone,
        })
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTime::Date(d) => writeln!(f, "{}", d.to_string()),
            DateTime::Time(t) => writeln!(f, "{}", t.to_string()),
            DateTime::Full { date, time } => {
                writeln!(f, "{}T{}", date.to_string(), time.to_string())
            }
        }
    }
}
