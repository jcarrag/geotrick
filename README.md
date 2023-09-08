### Board

Adafruit Feather nRF52840 Sense

- Feather M4 (`atsamd51j`)

### Installation

- Install `cargo-hf2`

```
cargo install cargo-hf2
```

- Add the following udev config (so `cargo-hf2` can find the device)

```
ATTRS{idVendor}=="239a", ENV{ID_MM_DEVICE_IGNORE}="1"
SUBSYSTEM=="usb", ATTRS{idVendor}=="239a", MODE="0666"
SUBSYSTEM=="tty", ATTRS{idVendor}=="239a", MODE="0666"
```

### Flashing

- Double click the onboard button to enter bootloader mode

- Run

```
cargo embed --probe 1366:0101
```

### Debug

- SWD pads on bottom of board

### Helpful resources

- https://github.com/rust-embedded/awesome-embedded-rust
- https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m0
- https://github.com/jonas-schievink/rubble/tree/master/demos/nrf52-beacon
- https://learn.adafruit.com/adafruit-feather-sense?view=all
- https://learn.adafruit.com/introducing-the-adafruit-nrf52840-feather?view=all
- Overview: https://www.youtube.com/watch?v=UWgO-x437tg
- Overview [-sense]: https://www.youtube.com/watch?v=LBgu4MghDP4
- PCB: https://github.com/adafruit/Adafruit-Feather-nRF52840-Sense-PCB
- Embedded rust guide: https://www.anyleaf.org/blog/writing-embedded-firmware-using-rust
- embed-rs: https://probe.rs/docs/tools/cargo-embed/
- M4 hal example: https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m4/examples
- Embedded Cortext-M guide: https://interrupt.memfault.com/blog/embedded-async-rust
