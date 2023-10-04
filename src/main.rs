#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m_rt::entry;
use defmt as _;
use defmt_rtt as _;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use hal::{self as _, prelude::OutputPin};
use panic_probe as _;

#[entry]
fn main() -> ! {
    defmt::println!("init");

    if let Some(periph) = hal::pac::Peripherals::take() {
        let mut timer = hal::timer::Timer::new(periph.TIMER0);

        let p0 = hal::gpio::p0::Parts::new(periph.P0);

        let mut neopixel = p0
            .p0_16
            .degrade()
            .into_push_pull_output(hal::gpio::Level::Low);

        let mut p10 = p0
            .p0_27
            .degrade()
            .into_push_pull_output(hal::gpio::Level::Low);

        loop {
            for value_all in 0..(256 * 3) {
                let r = if value_all < 256 {
                    256 - value_all
                } else if value_all > 512 {
                    value_all - 512
                } else {
                    0
                };
                let g = if value_all < 256 {
                    value_all
                } else if value_all < 512 {
                    512 - value_all
                } else {
                    0
                };
                let b = if value_all > 256 && value_all <= 512 {
                    value_all - 256
                } else if value_all > 512 {
                    768 - value_all
                } else {
                    0
                };

                let value = ((g as u32) << 16) | ((r as u32) << 8) | (b as u32);

                // delay to slow down procession
                timer.delay_ms(10u32);

                // minimum delay for RESET signal
                // timer.delay(60u32);

                // defmt::println!(
                //     "value: {:#03}, colours: r: {:#03}, g: {:#03}, b: {:#03}",
                //     value_all,
                //     r,
                //     g,
                //     b
                // );

                for shift_i in (0..24).rev() {
                    let bit_unmasked = (value >> shift_i) & 1;
                    let bit = bit_unmasked & 1;

                    // defmt::println!("value is: {:#026b}", value);
                    // defmt::println!("bit_u is: {:#026b}", bit_unmasked);
                    // defmt::println!("bit is:   {:#026b}", bit);
                    // defmt::println!("");

                    if bit == 0 {
                        let _ = p10.set_high();
                        let _ = neopixel.set_high();
                        unsafe {
                            // 20 nops for 350~360ns delay
                            asm!(
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop",
                            );
                        }
                        let _ = p10.set_low();
                        let _ = neopixel.set_low();
                        unsafe {
                            // 49 nops for 790~800ns delay
                            asm!(
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop",
                            );
                        }
                    } else {
                        let _ = p10.set_high();
                        let _ = neopixel.set_high();
                        unsafe {
                            // 43 nops is 690ns in isolation
                            // 38 nops is 690~710ns in reality
                            asm!(
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop"
                            );
                        }
                        let _ = p10.set_low();
                        let _ = neopixel.set_low();
                        unsafe {
                            // 30 nops is 600~610ns in isolation
                            // 32 nops is 600~610ns in reality
                            asm!(
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
                                "nop", "nop"
                            );
                        }
                    }
                }
            }
        }
    };
    loop {}
}
