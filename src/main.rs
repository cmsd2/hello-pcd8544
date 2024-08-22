#![no_std]
#![no_main]

use core::cell::RefCell;

use esp_backtrace as _;
use esp_hal::clock::ClockControl;
use esp_hal::delay::Delay;
use esp_hal::gpio::{self, Io, Level, Output};
use esp_hal::peripherals::Peripherals;
use esp_hal::prelude::*;
use esp_hal::spi::master::Spi;
use esp_hal::spi::{SpiBitOrder, SpiMode};
use esp_hal::system::SystemControl;
use pcd8544_spi::{Nokia5110, PCD8544};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    //let mut pcd_clk   = io.pins.gpio4..into_push_pull_output(&mut io.crh);
    let sclk = io.pins.gpio4;
    let mosi = io.pins.gpio5;
    let cs0 = io.pins.gpio1;
    let dc = Output::new(io.pins.gpio22, Level::High);
    let rst = Output::new(io.pins.gpio12, Level::High);

    let spi = RefCell::new(
        Spi::new(peripherals.SPI2, 100.kHz(), SpiMode::Mode0, &clocks)
            .with_pins(
                Some(sclk),
                Some(mosi),
                gpio::NO_PIN,
                Some(cs0),
            )
            .with_bit_order(SpiBitOrder::MSBFirst, SpiBitOrder::MSBFirst),
    );

    let driver = PCD8544::new(&spi, dc, rst, delay).expect("Infallible cannot fail");
    let mut display = Nokia5110::new(driver);

    display.init().expect("init");

    let mut ones = [0u8; 84];
    let mut zeroes = [0u8; 84];
    for i in 0..84 {
        ones[i] = 0b11111111;
        zeroes[i] = 0b00000000;
    }

    let mut lines = 0;
    let mut filling_not_clearing = true;

    loop {
        log::info!("Hello 5110!!");

        //led.toggle();
        delay.delay(500.millis());

        let line = if filling_not_clearing { &ones } else { &zeroes };

        display.write_data(line).expect("write_data");

        lines += 1;

        if lines > 5 {
            filling_not_clearing = !filling_not_clearing;
            lines = 0;
        }
    }
}
