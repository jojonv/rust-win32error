# rust_win32error

Error like wrapper for GetLastError for Rust.

## Example ##

example.rs
``` Rust
extern crate rust_win32error;
extern crate kernel32;

use rust_win32error::*;
use kernel32::OpenProcess;
use std::error::Error;
use std::fmt;



fn func() {
    let process_terminate = 0x0001;
    let h = unsafe { OpenProcess(process_terminate, 0, 4) };
    let err = Win32Error::new();
    println!("{}", err);
}
```