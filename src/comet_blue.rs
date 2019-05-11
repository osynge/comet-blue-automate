extern crate serde;
extern crate serde_json;

use crate::chrono::{Datelike, Timelike};

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Temperatures {
    current: u8,
    manual: u8,
    target_low: u8,
    target_high: u8,
    offset: u8,
    window_open_detection: u8,
    window_open_minutes: u8,
}

impl TryInto<Vec<u8>> for Temperatures {
    type Error = &'static str;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![
            self.current,
            self.manual,
            self.target_low,
            self.target_high,
            self.offset,
            self.window_open_detection,
            self.window_open_minutes,
        ])
    }
}

impl TryFrom<Vec<u8>> for Temperatures {
    type Error = &'static str;
    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        if input.len() != 7 {
            return Err("wrong length");
        }
        let res = Temperatures {
            current: input[0],
            manual: input[1],
            target_low: input[2],
            target_high: input[3],
            offset: input[4],
            window_open_detection: input[5],
            window_open_minutes: input[6],
        };
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Datetime {
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u8,
}

impl TryFrom<Vec<u8>> for Datetime {
    type Error = &'static str;
    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        if input.len() != 5 {
            return Err("wrong length");
        }
        Ok(Datetime {
            minute: input[0],
            hour: input[1],
            day: input[2],
            month: input[3],
            year: input[4],
        })
    }
}

impl TryFrom<chrono::DateTime<chrono::Local>> for Datetime {
    type Error = &'static str;
    fn try_from(input: chrono::DateTime<chrono::Local>) -> Result<Self, Self::Error> {
        let try_smaller_year = u8::try_from(input.year() - 2000);
        let try_smaller_month = u8::try_from(input.month());
        let try_smaller_day = u8::try_from(input.day());
        let try_smaller_hour = u8::try_from(input.hour());
        let try_smaller_minute = u8::try_from(input.minute());
        let smaller_year = match try_smaller_year {
            Ok(p) => p,
            Err(_) => return Err("year overflow"),
        };
        let smaller_month = match try_smaller_month {
            Ok(p) => p,
            Err(_) => return Err("month overflow"),
        };
        let smaller_day = match try_smaller_day {
            Ok(p) => p,
            Err(_) => return Err("day overflow"),
        };
        let smaller_hour = match try_smaller_hour {
            Ok(p) => p,
            Err(_) => return Err("hour overflow"),
        };
        let smaller_minute = match try_smaller_minute {
            Ok(p) => p,
            Err(_) => return Err("minute overflow"),
        };
        Ok(Datetime {
            minute: smaller_minute,
            hour: smaller_hour,
            day: smaller_day,
            month: smaller_month,
            year: smaller_year,
        })
    }
}

impl TryInto<Vec<u8>> for Datetime {
    type Error = &'static str;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![
            self.minute,
            self.hour,
            self.day,
            self.month,
            self.year,
        ])
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Day {
    #[serde(default = "default_day_field")]
    start_1: u8,
    #[serde(default = "default_day_field")]
    end_1: u8,
    #[serde(default = "default_day_field")]
    start_2: u8,
    #[serde(default = "default_day_field")]
    end_2: u8,
    #[serde(default = "default_day_field")]
    start_3: u8,
    #[serde(default = "default_day_field")]
    end_3: u8,
    #[serde(default = "default_day_field")]
    start_4: u8,
    #[serde(default = "default_day_field")]
    end_4: u8,
}

fn default_day_field() -> u8 {
    255
}

impl TryInto<Vec<u8>> for Day {
    type Error = &'static str;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![
            self.start_1,
            self.end_1,
            self.start_2,
            self.end_2,
            self.start_3,
            self.end_3,
            self.start_4,
            self.end_4,
        ])
    }
}

impl TryFrom<Vec<u8>> for Day {
    type Error = &'static str;
    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        if input.len() != 8 {
            return Err("wrong length");
        }
        Ok(Day {
            start_1: input[0],
            end_1: input[1],
            start_2: input[2],
            end_2: input[3],
            start_3: input[4],
            end_3: input[5],
            start_4: input[6],
            end_4: input[7],
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Holiday {
    #[serde(default = "default_holiday_field")]
    clock_start: u8,
    #[serde(default = "default_holiday_field")]
    day_start: u8,
    #[serde(default = "default_holiday_field")]
    month_start: u8,
    #[serde(default = "default_holiday_field")]
    year_start: u8,
    #[serde(default = "default_holiday_field")]
    clock_end: u8,
    #[serde(default = "default_holiday_field")]
    day_end: u8,
    #[serde(default = "default_holiday_field")]
    month_end: u8,
    #[serde(default = "default_holiday_field")]
    year_end: u8,
    #[serde(default = "default_holiday_field")]
    temperature: u8,
}

fn default_holiday_field() -> u8 {
    128
}

impl TryInto<Vec<u8>> for Holiday {
    type Error = &'static str;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![
            self.clock_start,
            self.day_start,
            self.month_start,
            self.year_start,
            self.clock_end,
            self.day_end,
            self.month_end,
            self.year_end,
            self.temperature,
        ])
    }
}

impl TryFrom<Vec<u8>> for Holiday {
    type Error = &'static str;
    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        if input.len() != 9 {
            return Err("wrong length");
        }
        Ok(Holiday {
            clock_start: input[0],
            day_start: input[1],
            month_start: input[2],
            year_start: input[3],
            clock_end: input[4],
            day_end: input[5],
            month_end: input[6],
            year_end: input[7],
            temperature: input[8],
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Battery {
    #[serde(default = "default_battery_field")]
    charge: u8,
}

fn default_battery_field() -> u8 {
    0
}

impl TryFrom<Vec<u8>> for Battery {
    type Error = &'static str;
    fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
        if input.len() != 1 {
            return Err("wrong length");
        }
        Ok(Battery { charge: input[0] })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pin {
    pub pin_1: u8,
    pub pin_2: u8,
    pub pin_3: u8,
    pub pin_4: u8,
    pub pin_5: u8,
    pub pin_6: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Week {
    pub monday: Day,
    pub tuesday: Day,
    pub wednesday: Day,
    pub thursday: Day,
    pub friday: Day,
    pub saterday: Day,
    pub sunday: Day,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Holidays {
    pub holiday_1: Holiday,
    pub holiday_2: Holiday,
    pub holiday_3: Holiday,
    pub holiday_4: Holiday,
    pub holiday_5: Holiday,
    pub holiday_6: Holiday,
    pub holiday_7: Holiday,
    pub holiday_8: Holiday,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schedule {
    pub week: Week,
    pub holidays: Holidays,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommetBlue {
    pub address: [u8; 6], // 6 byte address used to identify Bluetooth devices
    pub pin: Pin,
    pub clock: Datetime,
    pub identifier: String,
    pub version: String,
    pub firmware_revison: String,
    pub temperatures: Temperatures,
    pub schedule: Schedule,
    pub battery: Battery,
}
