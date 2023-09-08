### Board

Adafruit Feather nRF52840 Sense

- Feather M4 (`atsamd51j`)

### Installation

1. Add the udev rules necessary for the J-Link EDU mini to be discovered:

```
# This rules are based on the udev rules from the OpenOCD project, with unsupported probes removed.
# See http://openocd.org/ for more details.

ACTION!="add|change", GOTO="probe_rs_rules_end"

SUBSYSTEM=="gpio", MODE="0660", TAG+="uaccess"

SUBSYSTEM!="usb|tty|hidraw", GOTO="probe_rs_rules_end"

# SEGGER J-Link mini
ATTRS{idVendor}=="1366", ATTRS{idProduct}=="0101", MODE="660", TAG+="uaccess"

LABEL="probe_rs_rules_end"
```

### Running

1. Connect the J-Link EDU mini to the laptop
2. Power the nrf52840 (by connecting it to the laptop)
3. Start OpenOCD with:

```
$ sudo openocd
Open On-Chip Debugger 0.11.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : J-Link EDU Mini V1 compiled Feb 18 2021 11:25:23
Info : Hardware version: 1.00
Info : VTarget = 3.285 V
Info : clock speed 1000 kHz
Info : SWD DPIDR 0x2ba01477
Info : nrf52.cpu: hardware has 6 breakpoints, 4 watchpoints
Info : starting gdb server for nrf52.cpu on 3333
Info : Listening on port 3333 for gdb connections
...
```

4. Run `cargo run` which will connect to the debugger and pause execution

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `gdb -q -x openocd.gdb target/thumbv7em-none-eabihf/debug/geotrick`
Reading symbols from target/thumbv7em-none-eabihf/debug/geotrick...
0x00000760 in core::fmt::{impl#0}::write_char<cortex_m_semihosting::hio::HostStream>
    (self=0x2003ff60, c=4294967289 '\xfffffff9')
    at /rustc/75b7e52e92c3b00fc891b47f5b2efdff0a2be55a/library/core/src/fmt/mod.rs:202
202	/rustc/75b7e52e92c3b00fc891b47f5b2efdff0a2be55a/library/core/src/fmt/mod.rs: No such file or directory.
Breakpoint 1 at 0x486: file src/lib.rs, line 1053.
Note: automatically using hardware breakpoints for read-only addresses.
Breakpoint 2 at 0x1ec8: file src/lib.rs, line 1046.
Breakpoint 3 at 0x1d0c: file src/lib.rs, line 71.
Breakpoint 4 at 0x45c: file src/main.rs, line 14.
semihosting is enabled

Loading section .vector_table, size 0x400 lma 0x0
Loading section .text, size 0x1acc lma 0x400
Loading section .rodata, size 0x54c lma 0x1ed0
Start address 0x00000400, load size 9240
Transfer rate: 13 KB/sec, 3080 bytes/write.
cortex_m_rt::DefaultPreInit () at src/lib.rs:1058
1058	pub unsafe extern "C" fn DefaultPreInit() {}
(gdb)
```

5. Use the gdb interpreter to continue execution of the program

```
...
(gdb) continue
Continuing.

Breakpoint 4, geotrick::__cortex_m_rt_main_trampoline () at src/main.rs:14
14	#[entry]
(gdb) next
```

At this point you will see `hprintln` output in the OpenOCD terminal

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
