#![no_std]
#![no_main]

use core::ptr;

use cortex_m_rt::entry;
use panic_halt as _; // panic handler

// iMXRT1050 GPIO1 base address (from Reference Manual)
const GPIO1_BASE: u32 = 0x401B_8000;
const GPIO_DR: u32 = 0x00;       // Data Register offset
const GPIO_GDIR: u32 = 0x04;     // Direction Register offset

// GPIO pin for LED1 is GPIO1_IO03 (pin 3)
const LED_PIN: u32 = 3;

fn delay(count: u32) {
    for _ in 0..count {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    // Configure pin as GPIO output
    unsafe {
        let gdir = (GPIO1_BASE + GPIO_GDIR) as *mut u32;
        let dr = (GPIO1_BASE + GPIO_DR) as *mut u32;

        // Set pin 3 as output
        let mut val = ptr::read_volatile(gdir);
        val |= 1 << LED_PIN;
        ptr::write_volatile(gdir, val);

        loop {
            // Set pin high (turn LED on)
            let mut data = ptr::read_volatile(dr);
            data |= 1 << LED_PIN;
            ptr::write_volatile(dr, data);

            delay(5_000_000);

            // Set pin low (turn LED off)
            let mut data = ptr::read_volatile(dr);
            data &= !(1 << LED_PIN);
            ptr::write_volatile(dr, data);

            delay(5_000_000);
        }
    }
}
