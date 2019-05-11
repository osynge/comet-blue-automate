use chrono;
use clap::ArgMatches;
use fern;
use log;
use std;

pub fn log_setup(matches: &ArgMatches) -> () {
    let mut verbosity: u64 = 0;
    let mut brevity: u64 = 0;
    if matches.is_present("verbose") {
        verbosity = matches.occurrences_of("verbose");
    }
    if matches.is_present("quiet") {
        brevity = matches.occurrences_of("quiet");
    }
    let i32_log_level: i32 = 3 - verbosity as i32 + brevity as i32;
    let log_level = match i32_log_level {
        n if n <= 0 => log::LevelFilter::Trace,
        1 => log::LevelFilter::Debug,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Warn,
        _ => log::LevelFilter::Error,
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
