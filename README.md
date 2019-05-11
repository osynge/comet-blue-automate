# comet-blue-automate

This Linux application automates one or more "Comet Blue" compatible radiator thermostat, by converting the state to and from JSON. It uses Bluetooth Low Energy (BLE) to control the radiator thermostat Comet Blue. The application is intended to be used with crontab.

# Acknowledgements

This application would not be possible without the work of [Torsten Tränkner for documenting the Comet Blue](https://www.torsten-traenkner.de/wissen/smarthome/heizung.php) and [Micah Wylde for writing rumble a Rust Bluetooth Low Energy (BLE) central module library](https://github.com/mwylde/rumble). Much of this documentation comes from Torsten's blog.