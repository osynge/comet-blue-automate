# comet-blue-automate

This Linux application automates one or more "Comet Blue" compatible radiator thermostat, by converting the state to and from JSON. It uses Bluetooth Low Energy (BLE) to control the radiator thermostat Comet Blue from the manufacturer EUROtronic GmbH. It depends on [rumble](https://github.com/mwylde/rumble) and [BlueZ](http://www.bluez.org/) and runs on Linux.

# Compiling

comet-blue-automate is written in [rust](https://www.rust-lang.org/) and uses [cargo](https://doc.rust-lang.org/cargo/guide/) to build.

    $ cargo build --release

`cargo build --release` puts the resulting binary in `target/release` instead of `target/debug`.

You can run the application in the target directory, or copy it to a system path.

    $ ./target/debug/comet-blue-automate -h

# Quickstart

You probably do not have permission as a normal linux UID to run comet-blue-automate and will get a `PermissionDenied` error, as it interfaces with BlueZ to control the Bluetooth interface so it maybe easiest to test running with higher privileges. In this quickstart we run as root, but running this application in the `bluetooth` group would be production practice with cron.

First record the state of all your comet-blue thermostats in each room. For example:

    # comet-blue-automate --save kitchen.json
    # comet-blue-automate --save bedroom.json
    # comet-blue-automate --save livingroom.json
    # comet-blue-automate --save bathroom.json

These json files are lists of thermostat states. These can be written back to the thermostats listed to restore the state.

These files may have duplicates thermostats, so it is easier to make 1 new file for each thermostat:

    # ls
    kitchen-thermostat.json
    bedroom-thermostat.json
    livingroom-thermostat.json
    bathroom-thermostat.json

Alternatively you can put all thermostat state in a single json file.

Next delete the `clock` from each thermostat's json file. This will then allow `comet-blue-automate` to set the thermostat's clock to local system time.

    "clock": {
          "minute": 54,
          "hour": 20,
          "day": 11,
          "month": 5,
          "year": 19
    },

The command `comet-blue-automate --load json` will attempt to apply the json file to thermostats which match the Bluetooth address. It will not error if the device is not found. This allows us apply all state with a single run of the application.

    # comet-blue-automate --load kitchen-thermostat.json --load bedroom-thermostat.json --load livingroom-thermostat.json --load bathroom-thermostat.json

# Usage of comet-blue-automate

comet-blue-automate is a command line application and intended to be used with crontab.

## Show command line help

To show the command line help:

    $ comet-blue-automate --help

## Save current state to a json file

To save the current state of the thermostat to file

    $ comet-blue-automate --save state.json

This will save a file which looks like:

    [
      {
        "address": [
          84,
          56,
          46,
          22,
          74,
          84
        ],
        "pin": {
          "pin_1": 0,
          "pin_2": 0,
          "pin_3": 0,
          "pin_4": 0,
          "pin_5": 0,
          "pin_6": 0
        },
        "clock": {
          "minute": 54,
          "hour": 20,
          "day": 11,
          "month": 5,
          "year": 19
        },
        "identifier": "Comet Blue",
        "version": "COBL0124",
        "firmware_revison": "0.0.5-beta1",
        "manufacturer": "EUROtronic GmbH",
        "temperatures": {
          "current": 40,
          "manual": 21,
          "target_low": 28,
          "target_high": 15,
          "offset": 0,
          "window_open_detection": 4,
          "window_open_minutes": 10
        },
        "schedule": {
          "week": {
            "monday": {
              "start_1": 42,
              "end_1": 132,
              "start_2": 255,
              "end_2": 255,
              "start_3": 255,
              "end_3": 255,
              "start_4": 255,
              "end_4": 255
            },
            "tuesday": {
              "start_1": 42,
              "end_1": 132,
              "start_2": 255,
              "end_2": 255,
              "start_3": 255,
              "end_3": 255,
              "start_4": 255,
              "end_4": 255
            },
            "wednesday": {
              "start_1": 42,
              "end_1": 132,
              "start_2": 255,
              "end_2": 255,
              "start_3": 255,
              "end_3": 255,
              "start_4": 255,
              "end_4": 255
            },
            "thursday": {
              "start_1": 42,
              "end_1": 132,
              "start_2": 255,
              "end_2": 255,
              "start_3": 255,
              "end_3": 255,
              "start_4": 255,
              "end_4": 255
            },
            "friday": {
              "start_1": 42,
              "end_1": 132,
              "start_2": 255,
              "end_2": 255,
              "start_3": 255,
              "end_3": 255,
              "start_4": 255,
              "end_4": 255
            },
            "saterday": {
              "start_1": 42,
              "end_1": 132,
              "start_2": 255,
              "end_2": 255,
              "start_3": 255,
              "end_3": 255,
              "start_4": 255,
              "end_4": 255
            },
            "sunday": {
              "start_1": 42,
              "end_1": 132,
              "start_2": 255,
              "end_2": 255,
              "start_3": 255,
              "end_3": 255,
              "start_4": 255,
              "end_4": 255
            }
          },
          "holidays": {
            "holiday_1": {
              "clock_start": 128,
              "day_start": 128,
              "month_start": 128,
              "year_start": 128,
              "clock_end": 128,
              "day_end": 128,
              "month_end": 128,
              "year_end": 128,
              "temperature": 128
            },
            "holiday_2": {
              "clock_start": 128,
              "day_start": 128,
              "month_start": 128,
              "year_start": 128,
              "clock_end": 128,
              "day_end": 128,
              "month_end": 128,
              "year_end": 128,
              "temperature": 128
            },
            "holiday_3": {
              "clock_start": 128,
              "day_start": 128,
              "month_start": 128,
              "year_start": 128,
              "clock_end": 128,
              "day_end": 128,
              "month_end": 128,
              "year_end": 128,
              "temperature": 128
            },
            "holiday_4": {
              "clock_start": 128,
              "day_start": 128,
              "month_start": 128,
              "year_start": 128,
              "clock_end": 128,
              "day_end": 128,
              "month_end": 128,
              "year_end": 128,
              "temperature": 128
            },
            "holiday_5": {
              "clock_start": 128,
              "day_start": 128,
              "month_start": 128,
              "year_start": 128,
              "clock_end": 128,
              "day_end": 128,
              "month_end": 128,
              "year_end": 128,
              "temperature": 128
            },
            "holiday_6": {
              "clock_start": 165,
              "day_start": 165,
              "month_start": 165,
              "year_start": 165,
              "clock_end": 165,
              "day_end": 165,
              "month_end": 165,
              "year_end": 165,
              "temperature": 165
            },
            "holiday_7": {
              "clock_start": 128,
              "day_start": 128,
              "month_start": 128,
              "year_start": 128,
              "clock_end": 128,
              "day_end": 128,
              "month_end": 128,
              "year_end": 128,
              "temperature": 128
            },
            "holiday_8": {
              "clock_start": 128,
              "day_start": 128,
              "month_start": 128,
              "year_start": 128,
              "clock_end": 128,
              "day_end": 128,
              "month_end": 128,
              "year_end": 128,
              "temperature": 128
            }
          }
        },
        "battery": {
          "charge": 16
        }
      }
    ]

## Load desired state into the thermostat

To set the current state of the thermostat from file

    $ comet-blue-automate --load state.json

This will take the file you saved.

### Note:
If you do not want to specify the time in the json file the thermostat will be set as the system time on the computer.


# Acknowledgements

This application would not be possible without the work of [Torsten Tränkner for documenting the Comet Blue](https://www.torsten-traenkner.de/wissen/smarthome/heizung.php) and [Micah Wylde for writing rumble a Rust Bluetooth Low Energy (BLE) central module library](https://github.com/mwylde/rumble). Much of this documentation comes from Torsten's blog.