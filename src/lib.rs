//! Tiny crate for drawing pictures on terminal by [Sixel graphics](https://www.vt100.net/docs/vt3xx-gp/chapter14.html)

use std::fmt;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Char(u8);

impl Char {
    pub fn empty() -> Self {
        Self(b'?')
    }

    pub fn all() -> Self {
        Self(b'~')
    }

    pub fn new(bits: [bool; 6]) -> Self {
        let [a, b, c, d, e, f] = bits;
        let mut x = 0x3F;
        if a {
            x += 32;
        }
        if b {
            x += 16;
        }
        if c {
            x += 8;
        }
        if d {
            x += 4;
        }
        if e {
            x += 2;
        }
        if f {
            x += 1;
        }
        Self(x)
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    Repeat { n: usize, c: Char },
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Repeat { n, c } => {
                write!(f, "!{}{}", n, c)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Control {
    seq: Vec<Element>,
}

impl Control {
    pub fn add_repeat(&mut self, n: usize, c: Char) {
        self.seq.push(Element::Repeat { n, c })
    }
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1bPq")?;
        for e in &self.seq {
            write!(f, "{}", e)?;
        }
        write!(f, "\x1b\\")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_new() {
        assert_eq!(
            Char::new([false, false, false, false, false, false]),
            Char::empty()
        );
        assert_eq!(Char::new([true, true, true, true, true, true]), Char::all());
    }
}
