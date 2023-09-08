#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt as _;
use defmt_rtt as _; // global logger
use hal as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    loop {
        defmt::println!("Hello there");
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
