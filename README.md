esc_p
======
Tiny crate for drawing pictures in terminal using [Sixel Graphics][Sixel Graphics]

This only implements a wrapper for device control string
(DCS, `ESC P` in [C1 control code](https://en.wikipedia.org/wiki/C0_and_C1_control_codes#C1_controls))
of [Sixel Graphics][Sixel Graphics].
If you are seeking encoder to sixel or decorder from it, go to [libsixel](https://github.com/saitoha/libsixel).

[Sixel Graphics]: https://www.vt100.net/docs/vt3xx-gp/chapter14.html

Example
-------
Setup Sixel-compatible terminal emulators, and run

```shell
cargo run --example main
```

License
--------
© 2023 Toshiki Teramura (@termoshtt)

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
