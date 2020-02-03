// #![no_std]
#![feature(format_args_nl)]

use core::fmt;
use std::io::Write;

mod write_to;

#[cfg(test)]
mod tests {
    use super::write_to::*;

    #[test]
    pub fn write_to_works() {
        let mut buf = [0u8; 64];
        let s: &str = show(
            &mut buf,
            format_args!("write some stuff {:?}: {}", "foo", 42),
        )
        .unwrap();
        assert_eq!(s, "write some stuff \"foo\": 42");
        assert_eq!(s.as_ptr(), buf.as_ptr());
    }
}

// https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! myprintln {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        _print(format_args_nl!($($arg)*));
    })
}

pub fn _print(args: fmt::Arguments) {
    let mut buf = [0u8; 512];
    let s = write_to::show(&mut buf, args).unwrap();
    std::io::stdout()
        .lock()
        .write_all(b"Converted to str\n")
        .unwrap();
    std::io::stdout().lock().write_all(s.as_bytes()).unwrap();
}

fn main() {
    myprintln!("Hello, world!");
}
