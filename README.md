Important notes:
i.MX RT1050 EVKB onboard LEDs are connected to GPIO pins on GPIO1_IO03 (usually LED1).

The board uses the i.MXRT1050 chip with ARM Cortex-M7.

We will do a simple GPIO toggle with delays, no RTOS, no HAL (bare-metal).

You must have a Rust toolchain installed targeting thumbv7em-none-eabihf.

You should have a linker script and startup for cortex-m7 (usually provided by a crate or example BSP).

We’ll create a minimal example you can build and get a .bin or .elf to flash with J-Flash.

Step 1: Create Cargo project
bash
Copy
Edit
cargo new imxrt1050_blink --bin
cd imxrt1050_blink
Step 2: Replace src/main.rs with this code
rust
Copy
Edit
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
Step 3: Add dependencies in Cargo.toml
Add these lines inside [dependencies]:

toml
Copy
Edit
cortex-m = "0.7.6"
cortex-m-rt = "0.7.1"
panic-halt = "0.2.0"
And also add in [profile.release]:

toml
Copy
Edit
[profile.release]
codegen-units = 1
debug = false
lto = true
Step 4: Set Rust target
Create a memory.x file for the linker script or get one for iMXRT1050 (usually you can get from imxrt-rt repo). A minimal one could be:

ld
Copy
Edit
MEMORY
{
  FLASH (rx) : ORIGIN = 0x60000000, LENGTH = 1024K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 512K
}

_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .text : {
    *(.text*)
    *(.rodata*)
    _etext = .;
  } > FLASH

  .data : {
    _sdata = .;
    *(.data*)
    _edata = .;
  } > RAM AT > FLASH

  .bss : {
    _sbss = .;
    *(.bss*)
    *(COMMON)
    _ebss = .;
  } > RAM

  . = ALIGN(4);
  _end = .;
}
Step 5: Build the code
bash
Copy
Edit
rustup target add thumbv7em-none-eabihf
cargo build --release --target thumbv7em-none-eabihf
You get .elf file in

target/thumbv7em-none-eabihf/release/imxrt1050_blink

Step 6: Flash with J-Flash
Convert elf to bin if needed (J-Flash supports elf, better).

Connect J-Link and iMXRT1050 EVKB.

Use J-Flash GUI or CLI to flash the elf file.

Bonus: If you want a full ready example, I can generate the complete project folder and you just compile and flash.
Would you like me to generate a full ready-to-compile project ZIP for you? Or do you want me to help with linker scripts and flashing instructions?







You said:
there is no extension of the target file


ChatGPT said:
Right, when you build a Rust project for an embedded target like thumbv7em-none-eabihf, the output file usually does not have a file extension by default. The file is an ELF binary, but it just appears as the project name without .elf unless you rename or specify otherwise.

Explanation:
The file will be at:

bash
Copy
Edit
target/thumbv7em-none-eabihf/release/imxrt1050_blink
This is an ELF binary (Executable and Linkable Format), but on Linux/Windows it shows no extension by default.

J-Flash can directly load ELF files, so you can point it to that file.

If you want, you can rename it to imxrt1050_blink.elf for clarity — it is still the same file.

How to confirm it's ELF?
Run this command in your terminal (Linux/macOS or Windows WSL/Git Bash):

bash
Copy
Edit
file target/thumbv7em-none-eabihf/release/imxrt1050_blink
You should see output like:

arduino
Copy
Edit
target/thumbv7em-none-eabihf/release/imxrt1050_blink: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), ...
How to generate a .bin if you want raw binary:
If J-Flash or your flashing tool needs raw .bin (flat binary), you can generate it using objcopy:

bash
Copy
Edit
arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/imxrt1050_blink target/thumbv7em-none-eabihf/release/imxrt1050_blink.bin
You need the ARM GNU toolchain installed (arm-none-eabi-objcopy).

TL;DR
The target file is the ELF binary without extension.

You can use it as is with J-Flash.

Or convert to .bin with objcopy if needed.

