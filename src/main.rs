#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::ClockControl;
use esp_hal::delay::Delay;
use esp_hal::gpio::Io;
use esp_hal::ledc::{channel, timer, LSGlobalClkSource, Ledc, LowSpeed};
use esp_hal::peripherals::Peripherals;
use esp_hal::prelude::*;
use esp_hal::system::SystemControl;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    //let mut led = Output::new(io.pins.gpio8, Level::High);

    esp_println::logger::init_logger_from_env();

    let mut ledc = Ledc::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 4.kHz(),
        })
        .unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel0, io.pins.gpio8);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 50,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    loop {
        log::info!("Hello world!!");
        //led.toggle();
        delay.delay(500.millis());
    }
}
