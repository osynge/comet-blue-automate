extern crate rand;
extern crate rumble;

extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use crate::chrono::Datelike;
use chrono::{Local, NaiveDate, NaiveDateTime, Timelike, Utc};

use rand::{thread_rng, Rng};
use rumble::api::{Central, Peripheral, UUID};
use rumble::bluez::manager::Manager;
use std::convert::TryFrom;
use std::thread;
use std::time::Duration;
mod comet_blue;

pub fn on_event(foo: rumble::api::CentralEvent) {
    match foo {
        rumble::api::CentralEvent::DeviceDiscovered(BDAddr) => {
            println!("DeviceDiscovered:{}", BDAddr);
        }
        rumble::api::CentralEvent::DeviceLost(BDAddr) => {
            println!("DeviceLost:{}", BDAddr);
        }
        rumble::api::CentralEvent::DeviceUpdated(BDAddr) => {
            println!("DeviceUpdated:{}", BDAddr);
        }
        rumble::api::CentralEvent::DeviceConnected(BDAddr) => {
            println!("DeviceConnected:{}", BDAddr);
        }
        rumble::api::CentralEvent::DeviceDisconnected(BDAddr) => {
            println!("DeviceDisconnected:{}", BDAddr);
        }
    }
}

fn getcommetdate() -> Result<Vec<u8>, ()> {
    //let now: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();

    let try_smaller_year = u8::try_from(now.year() - 2000);
    let try_smaller_month = u8::try_from(now.month());
    let try_smaller_day = u8::try_from(now.day());
    let try_smaller_hour = u8::try_from(now.hour());
    let try_smaller_minute = u8::try_from(now.minute());
    let setdateyear = try_smaller_year.unwrap();
    let setdatemonth = try_smaller_month.unwrap();
    let setdateday = try_smaller_day.unwrap();
    let setdatehour = try_smaller_hour.unwrap();
    let setdateminute = try_smaller_minute.unwrap();
    println!("setdateyear:{}:{}", setdateyear, setdatemonth);
    let setdate = vec![
        setdateminute,
        setdatehour,
        setdateday,
        setdatemonth,
        setdateyear,
    ];
    Ok(setdate)
}

pub fn main() {
    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state
    println!("Hello, world!");
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();
    println!("Hello, world!");

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

    for num in foo {
        println!("address:{}", num.properties().address);
        match num.properties().local_name {
            Some(name) => println!("name:{}", name),
            None => {}
        }
    }
    let light = central.peripherals().into_iter().nth(1).unwrap();
    match light.properties().local_name {
        Some(name) => println!("name:{}", name),
        None => {}
    }
    // connect to the device
    light.connect().unwrap();
    // discover characteristics
    light.discover_characteristics().unwrap();

    // find the characteristic we want
    let chars = light.characteristics();
    //    let authcharuuid : [u8; 16] = [ 0x47 , 0xE9, 0xEE, 0x30, 0x47, 0xE9, 0x11, 0xE4, 0x89, 0x39, 0x16, 0x42, 0x30, 0xD1, 0xDF, 0x67 ];
    //    let authcharuuid : [u8; 16] = [ 0x67, 0xDF , 0xD1, 0x30, 0x42, 0x16, 0x39, 0x89, 0xE4, 0x11, 0xE9, 0x47, 0x30, 0xEE , 0xE9, 0x47];
    //  let cmd_char = chars.iter().find(|c| c.uuid == UUID::B128(authcharuuid)).unwrap();
    println!("len:{}", chars.len());
    for char in chars.iter() {
        println!("char:{}", char);
    }

    let temperatureschar = chars.iter().nth(28).unwrap();
    println!("temperatureschar:{}", temperatureschar);
    let datetimechar = chars.iter().nth(11).unwrap();
    println!("datetimechar:{}", datetimechar);
    let authchar = chars.iter().nth(32).unwrap();
    println!("authchar:{}", authchar);
    let pwd = vec![0, 0, 0, 0];
    light.request(&authchar, &pwd).unwrap();

    light.discover_characteristics().unwrap();
    let chars = light.characteristics();
    for i in light.read(&temperatureschar).unwrap() {
        println!("tmp:{}", i);
    }
    for i in light.read(&datetimechar).unwrap() {
        println!("datetimechar:{}", i);
    }
    let setdate = getcommetdate().unwrap();
    light.request(&datetimechar, &setdate).unwrap();
}
