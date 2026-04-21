#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    self as _,
    hal::{delay::Delay, twim::Twim},
    hal::uarte::{Baudrate, Parity, Uarte},
};
use microbit::Board;

use lsm303agr::Lsm303agr;
use microbit::pac::twim0::frequency::FREQUENCY_A;

use libm::atan2f;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn heading_to_direction(angle: f32) -> &'static str {
    if angle >= 337.5 || angle < 22.5 {
        "N"
    } else if angle < 67.5 {
        "NE"
    } else if angle < 112.5 {
        "E"
    } else if angle < 157.5 {
        "SE"
    } else if angle < 202.5 {
        "S"
    } else if angle < 247.5 {
        "SW"
    } else if angle < 292.5 {
        "W"
    } else {
        "NW"
    }
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    let twim0 = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut sensor = Lsm303agr::new_with_i2c(twim0);
    sensor.init().unwrap();

    let mut delay = Delay::new(board.SYST);

    sensor
        .set_mag_mode_and_odr(
            &mut delay,
            lsm303agr::MagMode::HighResolution,
            lsm303agr::MagOutputDataRate::Hz10,
        )
        .unwrap();

    let Ok(mut sensor) = sensor.into_mag_continuous() else {
        panic!("Error enabling continuous mode");
    };

    loop {
        if sensor.mag_status().unwrap().xyz_new_data() {
            let data = sensor.magnetic_field().unwrap();

            let x = data.x_nt() as f32;
            let y = data.y_nt() as f32;

            let mut heading = atan2f(y, x) * 180.0 / core::f32::consts::PI;
            if heading < 0.0 {
                heading += 360.0;
            }

            let direction = heading_to_direction(heading);

            write!(
                serial,
                "Magnetic field: x {} y {} z {}\n\r",
                data.x_nt(),
                data.y_nt(),
                data.z_nt()
            )
            .ok();

            write!(
                serial,
                "Direction: {} ({:.1} deg)\n\r",
                direction, heading
            )
            .ok();
        
            delay.delay_ms(500u32);
        }
    }
}