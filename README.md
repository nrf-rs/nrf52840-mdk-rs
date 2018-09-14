# Rust for nrf52840-mdk

https://github.com/makerdiary/nrf52840-mdk

## OpenOCD

I was unable to get this working with the openocd that ships with
ubuntu 18.04, but was able to hook up SWD to my JLink.  This thread
is someone else having similar issues:

https://devzone.nordicsemi.com/b/blog/posts/debugging-on-nrf52840-with-gdb-from-cli-on-linux

## JLink

I hooked up my JLink using one of these breakouts:
https://www.adafruit.com/product/2743

| MDK   | SWD Breakout |
| ----- | ------------ |
| 3V3   | Vref         |
| RST   | RST          |
| SWDIO | SWIO         |
| SWCLK | CLK          |
| TDO   | SWO          |
| GND   | GND x 3      |

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
