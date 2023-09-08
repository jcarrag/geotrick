### Board

Adafruit Feather nRF52840 Sense

- Feather M4 (`atsamd51j`)

### Installation

> Based on https://embedded-trainings.ferrous-systems.com/

1. Install `probe-run` [dependencies](https://embedded-trainings.ferrous-systems.com/installation):

```
$ cargo install probe-run
$ cargo install flip-link
$ cargo install nrfdfu
```

1. Add the udev rules necessary for the J-Link EDU mini to be discovered. `TAG="uaccess"` does not seem to work, so I'm trying `GROUP="users"` atm, which also doesn't seem to work so need to run using `sudo`

```
# This rules are based on the udev rules from the OpenOCD project, with unsupported probes removed.
# See http://openocd.org/ for more details.

# SEGGER J-Link mini
ATTRS{idVendor}=="1366", ATTRS{idProduct}=="0101", MODE="0666", TAG+="uaccess", GROUP="users"
```

### Running

1. Connect the J-Link EDU mini to the laptop
2. Power the nrf52840 (by connecting it to the laptop)
3. Run `sudo cargo run` which will connect via `probe-run` (need `sudo` because of the udev issues mentioned above)

```
$ sudo cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `probe-run --chip nRF52840_xxAA --erase-all target/thumbv7em-none-eabihf/debug/geotrick`
(HOST) INFO  flashing program (4 pages / 16.00 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
Hello, world!
Hello there
Hello there
...
```

### Flashing

- Double click the onboard button to enter bootloader mode

### Debug

- SWD pads on bottom of board

### Helpful resources

- https://github.com/rust-embedded/cortex-m-quickstart
  - A very helpful intro to GDB + OpenOCD for the nrf52840 + usage examples
- https://github.com/rust-embedded/awesome-embedded-rust
- https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m0
- https://learn.adafruit.com/adafruit-feather-sense?view=all
- https://learn.adafruit.com/introducing-the-adafruit-nrf52840-feather?view=all
- Overview: https://www.youtube.com/watch?v=UWgO-x437tg
- Overview [-sense]: https://www.youtube.com/watch?v=LBgu4MghDP4
- PCB: https://github.com/adafruit/Adafruit-Feather-nRF52840-Sense-PCB
- Embedded rust guide: https://www.anyleaf.org/blog/writing-embedded-firmware-using-rust
- embed-rs: https://probe.rs/docs/tools/cargo-embed/
- M4 hal example: https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m4/examples
- Embedded Cortext-M guide: https://interrupt.memfault.com/blog/embedded-async-rust
- Guide to Rust embedded: https://docs.rust-embedded.org/book/start/hardware.html
- Guide to Rust embedded debugging: https://docs.rust-embedded.org/debugonomicon/
- More thorough guide to embedded: https://embedded-trainings.ferrous-systems.com/
