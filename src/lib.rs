//! Tiny crate for drawing pictures on terminal by [Sixel graphics](https://www.vt100.net/docs/vt3xx-gp/chapter14.html)

pub const DSC: u8 = 0x90;
pub const ST: u8 = 0x9C;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn print() {
        let ctl = vec![
            DSC, ';' as u8, 0, /* P2 */
            0, /* P3 */
            'q' as u8, 1, 1, 1, ST,
        ];
        std::io::stdout().write_all(&ctl).unwrap();
    }
}
