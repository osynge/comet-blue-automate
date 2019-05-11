use super::app_const;
use clap::App;
use clap::Arg;
use clap::ArgMatches;

pub fn cli_clap<'a>() -> ArgMatches<'a> {
    let application = App::new(app_const::PACKAGE)
        .about("Automates one or more 'Comet Blue' compatible radiator thermostat, by converting the state to and from JSON.")
        .version(app_const::VERSION_CLI)
        .author("Owen Synge <osynge@googlemail.com>")
        .arg(
            Arg::with_name("verbose")
                .help("Increase log output.")
                .short("v")
                .multiple(true)
                .long("verbose"),
        )
        .arg(
            Arg::with_name("quiet")
                .help("Decrease log output.")
                .short("q")
                .multiple(true)
                .long("quiet"),
        )
        .arg(
            Arg::with_name("load")
                .short("l")
                .long("load")
                .value_name("JSON_INPUT")
                .help("Load json file to thermostat.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .value_name("JSON_OUTPUT")
                .help("Save thermostat state to json file.")
                .multiple(true)
                .takes_value(true),
        );
    let matches = application.get_matches();
    return matches;
}
