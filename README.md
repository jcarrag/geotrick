### Installation

* Install `cargo-hf2`
```
cargo install cargo-hf2
```

* Add the following udev config (so `cargo-hf2` can find the device)
```
ATTRS{idVendor}=="239a", ENV{ID_MM_DEVICE_IGNORE}="1"
SUBSYSTEM=="usb", ATTRS{idVendor}=="239a", MODE="0666"
SUBSYSTEM=="tty", ATTRS{idVendor}=="239a", MODE="0666"
```


### Flashing
* Double click the onboard button to enter bootloader mode

* Run
```
cargo hf2 --release
```

### Helpful resources
* https://github.com/rust-embedded/awesome-embedded-rust
* https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m0
* https://github.com/jonas-schievink/rubble/tree/master/demos/nrf52-beacon
* https://learn.adafruit.com/adafruit-feather-m0-bluefruit-le?view=all
