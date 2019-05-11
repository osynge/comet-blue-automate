extern crate serde;
extern crate serde_json;

use crate::chrono::{Datelike, Timelike};

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperatures {
    current: u8,
    manual: u8,
    target_low: u8,
    target_high: u8,
    offset: u8,
    window_open_detection: u8,
    window_open_minutes: u8,
}

impl TryFrom<Vec<u8>> for Temperatures {
    type Error = &'static str;
    fn try_from(item: Vec<u8>) -> Result<Self, Self::Error> {
        let res = Temperatures {
            current: item[0],
            manual: item[1],
            target_low: item[2],
            target_high: item[3],
            offset: item[4],
            window_open_detection: item[5],
            window_open_minutes: item[6],
        };
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
            return Err("wrong lenght");
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    start_1: u8,
    end_1: u8,
    start_2: u8,
    end_2: u8,
    start_3: u8,
    end_3: u8,
    start_4: u8,
    end_4: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Holiday {
    temperature: u8,
    start: Datetime,
    end: Datetime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Battery {
    charge: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pin {
    pin_1: u8,
    pin_2: u8,
    pin_3: u8,
    pin_4: u8,
    pin_5: u8,
    pin_6: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Week {
    monday: Day,
    tuesday: Day,
    wednesday: Day,
    thursday: Day,
    friday: Day,
    saterday: Day,
    sunday: Day,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Holidays {
    holiday_1: Holiday,
    holiday_2: Holiday,
    holiday_3: Holiday,
    holiday_4: Holiday,
    holiday_5: Holiday,
    holiday_6: Holiday,
    holiday_7: Holiday,
    holiday_8: Holiday,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schedule {
    week: Week,
    holidays: Holidays,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommetBlue {
    address: [u8; 6], // 6 byte address used to identify Bluetooth devices
    pin: Pin,
    clock: Datetime,
    identifier: String,
    version: String,
    firmware_revison: String,
    temperatures: Temperatures,
    schedule: Schedule,
    battery: Battery,
}
