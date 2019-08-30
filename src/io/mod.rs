pub mod uart;

use crate::kernel;
use core::fmt;
#[derive(Clone, Copy)]
pub enum KernelStdio {
    MiniUart(uart::MiniUart),
    None
}

impl Read for KernelStdio {
    fn get_char(&self) -> Option<char> {
        match self {
            KernelStdio::None => Option::None,
            KernelStdio::MiniUart(u) => u.get_char(),
            _ => Option::None,
        }
    }
    fn get_line(&self) -> (usize, [u8; 128]) {
        match self {
            KernelStdio::None => (0, [0; 128]),
            KernelStdio::MiniUart(u) => u.get_line(),
            _ => (0, [0; 128]),
        }
    }
}
impl Write for KernelStdio {
    fn put_char(&self, c: char) -> Result<(), WriteError> {
        match self {
            KernelStdio::None => Ok(()),
            KernelStdio::MiniUart(u) => u.put_char(c),
            _ => Ok(()),
        }
    }
    fn put_string(&self, string: &str) -> Result<(), WriteError> {
        match self {
            KernelStdio::None => Ok(()),
            KernelStdio::MiniUart(u) => u.put_string(string),
            _ => Ok(()),
        }
    }
}
impl fmt::Write for KernelStdio {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.put_string(s).unwrap();
        Ok(())
    }
}
pub trait Read {
    fn get_char(&self) -> Option<char>;
    fn get_line(&self) -> (usize, [u8; 128]);
}
pub trait Write {
    fn put_char(&self, c: char) -> Result<(), WriteError>;
    fn put_string(&self, string: &str) -> Result<(), WriteError>;
}

#[derive(Debug)]
pub enum WriteError {
    
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::print!("\x1b[31m{}\x1b[0m", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    
    let mut stdio = kernel::get_kernel_ref().get_stdio();
    stdio.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! scanln {
    ($( $x:ty ),+ ) => {{
        print!("\x1B[38;2;100;255;255m(No Filesystem  )/ \x1B[38;2;200;255;100m❯\x1B[0m");
        let res;

        let stdio = kernel::get_kernel_ref().get_stdio();
        res = stdio.get_line();
        
        let string = core::str::from_utf8( &res.1).unwrap();
        let mut iter = string.split_ascii_whitespace();
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}
