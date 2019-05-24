extern crate chrono;
extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;
extern crate rand;
extern crate rumble;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use crate::chrono::Datelike;
use chrono::{Local, NaiveDate, NaiveDateTime, Timelike, Utc};

use rand::{thread_rng, Rng};
use rumble::api::{Central, Peripheral, UUID};
use rumble::bluez::manager::Manager;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;
mod app_const;
mod characteristics;
mod cli_clap;
mod comet_blue;
mod fern_setup;

use comet_blue::{Datetime, Temperatures};
use rumble::bluez::adapter::ConnectedAdapter;
use std::convert::TryInto;

pub fn on_event(foo: rumble::api::CentralEvent) {
    match foo {
        rumble::api::CentralEvent::DeviceDiscovered(bd_addr) => {
            debug!("DeviceDiscovered:{}", bd_addr);
        }
        rumble::api::CentralEvent::DeviceLost(bd_addr) => {
            debug!("DeviceLost:{}", bd_addr);
        }
        rumble::api::CentralEvent::DeviceUpdated(bd_addr) => {
            debug!("DeviceUpdated:{}", bd_addr);
        }
        rumble::api::CentralEvent::DeviceConnected(bd_addr) => {
            debug!("DeviceConnected:{}", bd_addr);
        }
        rumble::api::CentralEvent::DeviceDisconnected(bd_addr) => {
            debug!("DeviceDisconnected:{}", bd_addr);
        }
    }
}

fn getcommetdate() -> Result<Vec<u8>, ()> {
    //let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let bing = comet_blue::Datetime::try_from(now).unwrap();
    let _foo = match bing.try_into() {
        Ok(p) => return Ok(p),
        Err(_) => return Err(()),
    };
}

pub struct PeripheralHolder {
    peripheral: rumble::bluez::adapter::peripheral::Peripheral,
    characteristics: BTreeSet<rumble::api::Characteristic>,
}

impl PeripheralHolder {
    fn is_commet_blue(&self) -> bool {
        let cmd_char = self
            .characteristics
            .iter()
            .find(|c| c.uuid == characteristics::PASSWORD);
        match cmd_char {
            Some(_) => {
                return true;
            }
            None => {
                return false;
            }
        }
    }

    fn enter_pin(&self) {
        let cmd_char = self
            .characteristics
            .iter()
            .find(|c| c.uuid == characteristics::PASSWORD);
        let authchar = match cmd_char {
            Some(p) => {
                debug!("UUID:{}", characteristics::PASSWORD);
                p
            }
            None => {
                error!("Not found");
                return;
            }
        };
        let pwd = vec![0, 0, 0, 0];
        self.peripheral.request(&authchar, &pwd).unwrap();
    }
    fn datetime(&self) {
        let cmd_char = self
            .characteristics
            .iter()
            .find(|c| c.uuid == characteristics::DATETIME);
        let datetimechar = match cmd_char {
            Some(p) => {
                debug!("UUID:{}", characteristics::DATETIME);
                p
            }
            None => {
                error!("characteristics not found");
                return;
            }
        };

        let mut datetimevecraw = self.peripheral.read(&datetimechar).unwrap();
        let datetimevec: Vec<_> = datetimevecraw.drain(1..).collect();
        let bing = comet_blue::Datetime::try_from(datetimevec).unwrap();
        println!("Now {:?} will print!", bing);
        for i in self.peripheral.read(&datetimechar).unwrap() {
            println!("datetimechar:{}", i);
        }
        let setdate = getcommetdate().unwrap();
        self.peripheral.request(&datetimechar, &setdate).unwrap();
    }

    fn read<T: TryFrom<Vec<u8>>>(&self, btuuid: rumble::api::UUID) -> Result<T, &'static str> {
        let cmd_char = self.characteristics.iter().find(|c| c.uuid == btuuid);
        let datetimechar = match cmd_char {
            Some(p) => p,
            None => {
                return Err("btuuid not found");
            }
        };
        let mut datetimevecraw = self.peripheral.read(&datetimechar).unwrap();
        let datetimevec: Vec<_> = datetimevecraw.drain(1..).collect();
        match T::try_from(datetimevec) {
            Ok(p) => Ok(p),
            Err(_) => Err("sasdasd"),
        }
    }

    fn commet_blue_read(&self) -> Result<comet_blue::CommetBlue, &'static str> {
        let clock: comet_blue::Datetime = self.read(characteristics::DATETIME)?;
        let battery: comet_blue::Battery = self.read(characteristics::BATTERY)?;
        let temperatures: comet_blue::Temperatures = self.read(characteristics::TEMPERATURES)?;
        let identifier: comet_blue::Text = self.read(characteristics::IDENTIFIER)?;
        let version: comet_blue::Text = self.read(characteristics::VERSION)?;
        let firmware_revison: comet_blue::Text = self.read(characteristics::FIRMWARE_VERSION)?;
        let manufacturer: comet_blue::Text = self.read(characteristics::MANUFACTURER)?;
        let week = comet_blue::Week {
            monday: self.read(characteristics::MONDAY)?,
            tuesday: self.read(characteristics::TUESDAY)?,
            wednesday: self.read(characteristics::WEDNESDAY)?,
            thursday: self.read(characteristics::THURSDAY)?,
            friday: self.read(characteristics::FRIDAY)?,
            saturday: self.read(characteristics::SATURDAY)?,
            sunday: self.read(characteristics::SUNDAY)?,
        };
        let holidays = comet_blue::Holidays {
            holiday_1: self.read(characteristics::HOLIDAY_1)?,
            holiday_2: self.read(characteristics::HOLIDAY_2)?,
            holiday_3: self.read(characteristics::HOLIDAY_3)?,
            holiday_4: self.read(characteristics::HOLIDAY_4)?,
            holiday_5: self.read(characteristics::HOLIDAY_5)?,
            holiday_6: self.read(characteristics::HOLIDAY_6)?,
            holiday_7: self.read(characteristics::HOLIDAY_7)?,
            holiday_8: self.read(characteristics::HOLIDAY_8)?,
        };

        let schedule = comet_blue::Schedule {
            week: week,
            holidays: holidays,
        };
        let pin = comet_blue::Pin {
            pin_1: 0,
            pin_2: 0,
            pin_3: 0,
            pin_4: 0,
            pin_5: 0,
            pin_6: 0,
        };

        let commetblue = comet_blue::CommetBlue {
            address: self.peripheral.address().address,
            pin: pin,
            clock: clock,
            identifier: identifier,
            version: version,
            firmware_revison: firmware_revison,
            manufacturer: manufacturer,
            temperatures: temperatures,
            schedule: schedule,
            battery: battery,
        };

        Ok(commetblue)
    }

    fn write<T: TryInto<Vec<u8>>>(
        &self,
        t: T,
        btuuid: rumble::api::UUID,
    ) -> Result<(), &'static str> {
        let cmd_char = self.characteristics.iter().find(|c| c.uuid == btuuid);
        let datetimechar = match cmd_char {
            Some(p) => p,
            None => {
                return Err("btuuid not found");
            }
        };
        let writevec = match T::try_into(t) {
            Ok(p) => p,
            Err(_) => return Err("sasdasd"),
        };
        match self.peripheral.request(&datetimechar, &writevec) {
            Ok(_) => Ok(()),
            Err(_p) => Err("failed"),
        }
    }

    fn commet_blue_write(&mut self, cb: comet_blue::CommetBlue) -> Result<(), &'static str> {
        self.write(cb.clock, characteristics::DATETIME)?;
        self.write(cb.temperatures, characteristics::TEMPERATURES)?;
        self.write(cb.schedule.week.monday, characteristics::MONDAY)?;
        self.write(cb.schedule.week.tuesday, characteristics::TUESDAY)?;
        self.write(cb.schedule.week.wednesday, characteristics::WEDNESDAY)?;
        self.write(cb.schedule.week.thursday, characteristics::THURSDAY)?;
        self.write(cb.schedule.week.friday, characteristics::FRIDAY)?;
        self.write(cb.schedule.week.saturday, characteristics::SATURDAY)?;
        self.write(cb.schedule.week.sunday, characteristics::SUNDAY)?;
        self.write(cb.schedule.holidays.holiday_1, characteristics::HOLIDAY_1)?;
        self.write(cb.schedule.holidays.holiday_2, characteristics::HOLIDAY_2)?;
        self.write(cb.schedule.holidays.holiday_3, characteristics::HOLIDAY_3)?;
        self.write(cb.schedule.holidays.holiday_4, characteristics::HOLIDAY_4)?;
        self.write(cb.schedule.holidays.holiday_5, characteristics::HOLIDAY_5)?;
        self.write(cb.schedule.holidays.holiday_6, characteristics::HOLIDAY_6)?;
        self.write(cb.schedule.holidays.holiday_7, characteristics::HOLIDAY_7)?;
        self.write(cb.schedule.holidays.holiday_8, characteristics::HOLIDAY_8)?;
        Ok(())
    }
}

fn load(cb_list: &Vec<comet_blue::CommetBlue>) {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state

    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = adapter.connect().unwrap();
    let boo = Box::new(on_event);
    central.on_event(boo);
    // start scanning for devices
    central.start_scan().unwrap();

    // instead of waiting, you can use central.on_event to be notified of
    // new devices
    thread::sleep(Duration::from_secs(10));
    let foo = central.peripherals().into_iter();
    //asdfg(&central);
    for num in foo {
        debug!("address:{}", num.properties().address);
        match num.properties().local_name {
            Some(name) => info!("name:{}", name),
            None => {}
        }
        //connect(&num);
        //get_temps(&num);
    }

    for item in central.peripherals().into_iter() {
        match item.connect() {
            Ok(_p) => {}
            Err(p) => {
                error!("Failed to connect:{}", p);
                continue;
            }
        }
        // discover characteristics
        item.discover_characteristics().unwrap();

        // find the characteristic we want
        let chars = item.characteristics();

        let mut jil = PeripheralHolder {
            peripheral: item,
            characteristics: chars,
        };
        if !jil.is_commet_blue() {
            continue;
        }
        jil.enter_pin();
        let boo = jil.commet_blue_read().unwrap();

        for load_perf in cb_list {
            if load_perf.address == boo.address {
                jil.commet_blue_write(load_perf.clone());
            }
        }

        //jil.commet_blue_write(foo);
    }
}

fn save(save_path: &str) {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state

    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to the adapter
    let central = adapter.connect().unwrap();
    let boo = Box::new(on_event);
    central.on_event(boo);
    // start scanning for devices
    central.start_scan().unwrap();

    // instead of waiting, you can use central.on_event to be notified of
    // new devices
    thread::sleep(Duration::from_secs(10));
    let foo = central.peripherals().into_iter();
    //asdfg(&central);
    for num in foo {
        debug!("address:{}", num.properties().address);
        match num.properties().local_name {
            Some(name) => info!("name:{}", name),
            None => {}
        }
        //connect(&num);
        //get_temps(&num);
    }
    let mut all_peripherals = Vec::new();
    for item in central.peripherals().into_iter() {
        match item.connect() {
            Ok(_p) => {}
            Err(p) => {
                error!("Failed to connect:{}", p);
                continue;
            }
        }
        // discover characteristics
        item.discover_characteristics().unwrap();
        // find the characteristic we want
        let chars = item.characteristics();

        let jil = PeripheralHolder {
            peripheral: item,
            characteristics: chars,
        };
        if !jil.is_commet_blue() {
            continue;
        }
        jil.enter_pin();
        let boo = jil.commet_blue_read().unwrap();

        let _foo = boo.clone();
        all_peripherals.push(boo.clone());
        //jil.commet_blue_write(foo);
    }
    let serialized = serde_json::to_string_pretty(&all_peripherals).unwrap();
    let path = Path::new(save_path);
    let rsult_file = File::create(path);
    let mut file = match rsult_file {
        Ok(p) => p,
        Err(_) => return,
    };

    file.write_all(serialized.as_bytes());
}

fn deserialise(load_paths: &Vec<String>) -> Result<Vec<comet_blue::CommetBlue>, &'static str> {
    let mut output = Vec::new();
    for path_string in load_paths {
        let path = Path::new(path_string);
        let display = path.display();
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => {
                error!("couldn't open {}: {}", display, why.description());
                continue;
            }
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut serialized = String::new();
        match file.read_to_string(&mut serialized) {
            Err(why) => {
                error!("couldn't read {}: {}", display, why.description());
                continue;
            }
            Ok(_) => debug!("{} contains:\n{}", display, serialized),
        }
        let mut deserialized: Vec<comet_blue::CommetBlue> =
            serde_json::from_str(&serialized).unwrap();
        output.append(&mut deserialized);
    }
    Ok(output)
}

pub fn main() {
    let clap_matches = cli_clap::cli_clap();
    fern_setup::log_setup(&clap_matches);

    let mut load_path_list = Vec::new();

    if let Some(load_it) = clap_matches.values_of("load") {
        for load_str in load_it {
            load_path_list.push(load_str.to_string());
        }

        let comet_blue_list = match deserialise(&load_path_list) {
            Ok(p) => p,
            Err(_) => {
                return;
            }
        };

        load(&comet_blue_list);
    }

    if let Some(mut load_it) = clap_matches.values_of("save") {
        let save_path = load_it.nth(0).unwrap();
        save(&save_path)
    }
}
