# tsl256x Light-To-Digital Converter

> Platform agnostic driver for TSL256x series of light intensity sensors built using the embedded-hal

## What works

- Getting raw sensor readings for both visible+IR spectrum and IR-only spectrum

## TODO

- [ ] Support other i2c addresses
- [ ] Add method to power down chip to minimize power consumption
- [ ] Add method to perform lux calculation
- [ ] Add method to control integration time and sensor gain
- [ ] Add method to setup interrupts
- [ ] Support the TSL2560 (SPI version)

## Example

```rust
    extern crate tsl256x;
    use tsl256x::Tsl2561;
    
    let mut sensor = Tsl2561::new(i2c).unwrap();
    
    iprintln!(&mut cp.ITM.stim[0], "IR+Visible: {}, IR Only: {}",
                        sensor.visible_and_ir_raw().unwrap(),
                        sensor.ir_raw().unwrap());
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
