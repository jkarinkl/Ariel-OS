use ariel_os::hal::peripherals;

#[cfg(context = "bbc-microbit-v2")]
ariel_os::hal::define_peripherals!(LedPeripherals {
    led_col1: P0_28,
    led: P0_21,
});

#[cfg(context = "nrf52840dk")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_13 });

#[cfg(context = "nrf5340dk")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P0_28 });

#[cfg(context = "particle-xenon")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: P1_12 });

#[cfg(context = "rpi-pico")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PIN_25 });

#[cfg(context = "rpi-pico2")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PIN_25 });

#[cfg(context = "esp")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: GPIO0 });

#[cfg(context = "st-nucleo-f401re")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PA5 });

#[cfg(context = "st-nucleo-h755zi-q")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB0 });

#[cfg(context = "st-nucleo-wb55")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB5 });

#[cfg(context = "st-nucleo-wba55")]
ariel_os::hal::define_peripherals!(LedPeripherals { led: PB4 });
