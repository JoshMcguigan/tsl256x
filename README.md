# tsl256x Light-To-Digital Converter [![Docs](https://img.shields.io/crates/v/tsl256x.svg)](https://crates.io/crates/tsl256x) [![Docs](https://docs.rs/tsl256x/badge.svg)](https://docs.rs/tsl256x)

> Platform agnostic driver for TSL256x series of light intensity sensors built using the embedded-hal

## What works

- Getting raw sensor readings for both visible+IR spectrum and IR-only spectrum
- Using one of the three built in slave addresses, or any custom address
- Powering the chip on/off to conserve energy
- Setting sensor gain and integration time

## TODO

- [ ] Add method to perform lux calculation (requires nostd implementation of exponentiation for f32)
- [ ] Add method to setup interrupts
- [ ] Support the TSL2560 (SPI version)

## Example

```rust
    extern crate tsl256x;
    use tsl256x::{Tsl2561, SlaveAddr};
    
    let sensor = Tsl2561::new(&mut i2c, SlaveAddr::default().addr()).unwrap();
    sensor.power_on(&mut i2c); 
    
    // Note sensor readings are zero until one integration period (default 400ms) after power on
    iprintln!(&mut cp.ITM.stim[0], "IR+Visible: {}, IR Only: {}",
                        sensor.visible_and_ir_raw(&mut i2c).unwrap(),
                        sensor.ir_raw(&mut i2c).unwrap());
```
    
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
