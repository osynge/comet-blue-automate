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
/*
pub struct PeripheralHolder {
    peripheral: rumble::api::Peripheral,
    characteristics: BTreeSet<rumble::api::Characteristic>,
}
*/

fn is_commet_blue<P: rumble::api::Peripheral>(
    peripheral: &P,
    characteristics: &BTreeSet<rumble::api::Characteristic>,
) -> bool {
    let cmd_char = characteristics
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

fn enter_pin<P: rumble::api::Peripheral>(
    peripheral: &P,
    characteristics: &BTreeSet<rumble::api::Characteristic>,
) {
    let cmd_char = characteristics
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
    peripheral.request(&authchar, &pwd).unwrap();
}

fn datetime<P: rumble::api::Peripheral>(
    peripheral: &P,
    characteristics: &BTreeSet<rumble::api::Characteristic>,
) {
    let cmd_char = characteristics
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

    let mut datetimevecraw = peripheral.read(&datetimechar).unwrap();
    let datetimevec: Vec<_> = datetimevecraw.drain(1..).collect();
    let bing = comet_blue::Datetime::try_from(datetimevec).unwrap();
    println!("Now {:?} will print!", bing);
    for i in peripheral.read(&datetimechar).unwrap() {
        println!("datetimechar:{}", i);
    }
    let setdate = getcommetdate().unwrap();
    peripheral.request(&datetimechar, &setdate).unwrap();
}

fn read<T: TryFrom<Vec<u8>>, P: rumble::api::Peripheral>(
    peripheral: &P,
    characteristics: &BTreeSet<rumble::api::Characteristic>,
    btuuid: rumble::api::UUID,
) -> Result<T, &'static str> {
    let cmd_char = characteristics.iter().find(|c| c.uuid == btuuid);
    let datetimechar = match cmd_char {
        Some(p) => p,
        None => {
            return Err("btuuid not found");
        }
    };
    let mut datetimevecraw = peripheral.read(&datetimechar).unwrap();
    let datetimevec: Vec<_> = datetimevecraw.drain(1..).collect();
    match T::try_from(datetimevec) {
        Ok(p) => Ok(p),
        Err(_) => Err("sasdasd"),
    }
}

fn write<T: TryInto<Vec<u8>>, P: rumble::api::Peripheral>(
    peripheral: &P,
    characteristics: &BTreeSet<rumble::api::Characteristic>,
    t: T,
    btuuid: rumble::api::UUID,
) -> Result<(), &'static str> {
    let cmd_char = characteristics.iter().find(|c| c.uuid == btuuid);
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
    match peripheral.request(&datetimechar, &writevec) {
        Ok(_) => Ok(()),
        Err(_p) => Err("failed"),
    }
}

fn commet_blue_read<P: rumble::api::Peripheral>(
    peripheral: &P,
    characteristics: &BTreeSet<rumble::api::Characteristic>,
) -> Result<comet_blue::CommetBlue, &'static str> {
    let clock: comet_blue::Datetime = read(peripheral, characteristics, characteristics::DATETIME)?;
    let battery: comet_blue::Battery = read(peripheral, characteristics, characteristics::BATTERY)?;
    let temperatures: comet_blue::Temperatures =
        read(peripheral, characteristics, characteristics::TEMPERATURES)?;
    let identifier: comet_blue::Text =
        read(peripheral, characteristics, characteristics::IDENTIFIER)?;
    let version: comet_blue::Text = read(peripheral, characteristics, characteristics::VERSION)?;
    let firmware_revison: comet_blue::Text = read(
        peripheral,
        characteristics,
        characteristics::FIRMWARE_VERSION,
    )?;
    let manufacturer: comet_blue::Text =
        read(peripheral, characteristics, characteristics::MANUFACTURER)?;
    let week = comet_blue::Week {
        monday: read(peripheral, characteristics, characteristics::MONDAY)?,
        tuesday: read(peripheral, characteristics, characteristics::TUESDAY)?,
        wednesday: read(peripheral, characteristics, characteristics::WEDNESDAY)?,
        thursday: read(peripheral, characteristics, characteristics::THURSDAY)?,
        friday: read(peripheral, characteristics, characteristics::FRIDAY)?,
        saturday: read(peripheral, characteristics, characteristics::SATURDAY)?,
        sunday: read(peripheral, characteristics, characteristics::SUNDAY)?,
    };
    let holidays = comet_blue::Holidays {
        holiday_1: read(peripheral, characteristics, characteristics::HOLIDAY_1)?,
        holiday_2: read(peripheral, characteristics, characteristics::HOLIDAY_2)?,
        holiday_3: read(peripheral, characteristics, characteristics::HOLIDAY_3)?,
        holiday_4: read(peripheral, characteristics, characteristics::HOLIDAY_4)?,
        holiday_5: read(peripheral, characteristics, characteristics::HOLIDAY_5)?,
        holiday_6: read(peripheral, characteristics, characteristics::HOLIDAY_6)?,
        holiday_7: read(peripheral, characteristics, characteristics::HOLIDAY_7)?,
        holiday_8: read(peripheral, characteristics, characteristics::HOLIDAY_8)?,
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
        address: peripheral.address().address,
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

fn commet_blue_write<P: rumble::api::Peripheral>(
    peripheral: &P,
    characteristics: &BTreeSet<rumble::api::Characteristic>,
    cb: comet_blue::CommetBlue,
) -> Result<(), &'static str> {
    write(
        peripheral,
        characteristics,
        cb.clock,
        characteristics::DATETIME,
    )?;
    write(
        peripheral,
        characteristics,
        cb.temperatures,
        characteristics::TEMPERATURES,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.week.monday,
        characteristics::MONDAY,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.week.tuesday,
        characteristics::TUESDAY,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.week.wednesday,
        characteristics::WEDNESDAY,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.week.thursday,
        characteristics::THURSDAY,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.week.friday,
        characteristics::FRIDAY,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.week.saturday,
        characteristics::SATURDAY,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.week.sunday,
        characteristics::SUNDAY,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.holidays.holiday_1,
        characteristics::HOLIDAY_1,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.holidays.holiday_2,
        characteristics::HOLIDAY_2,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.holidays.holiday_3,
        characteristics::HOLIDAY_3,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.holidays.holiday_4,
        characteristics::HOLIDAY_4,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.holidays.holiday_5,
        characteristics::HOLIDAY_5,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.holidays.holiday_6,
        characteristics::HOLIDAY_6,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.holidays.holiday_7,
        characteristics::HOLIDAY_7,
    )?;
    write(
        peripheral,
        characteristics,
        cb.schedule.holidays.holiday_8,
        characteristics::HOLIDAY_8,
    )?;
    Ok(())
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

    for peripheral in central.peripherals().into_iter() {
        match peripheral.connect() {
            Ok(_p) => {}
            Err(p) => {
                error!("Failed to connect:{}", p);
                continue;
            }
        }
        // discover characteristics
        peripheral.discover_characteristics().unwrap();

        // find the characteristic we want
        let characteristics = peripheral.characteristics();

        if !is_commet_blue(&peripheral, &characteristics) {
            continue;
        }
        enter_pin(&peripheral, &characteristics);
        let boo = commet_blue_read(&peripheral, &characteristics).unwrap();

        for load_perf in cb_list {
            if load_perf.address == boo.address {
                commet_blue_write(&peripheral, &characteristics, load_perf.clone());
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
    for peripheral in central.peripherals().into_iter() {
        match peripheral.connect() {
            Ok(_p) => {}
            Err(p) => {
                error!("Failed to connect:{}", p);
                continue;
            }
        }
        // discover characteristics
        peripheral.discover_characteristics().unwrap();
        // find the characteristic we want
        let characteristics = peripheral.characteristics();

        if !is_commet_blue(&peripheral, &characteristics) {
            continue;
        }
        enter_pin(&peripheral, &characteristics);
        let boo = commet_blue_read(&peripheral, &characteristics).unwrap();

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
