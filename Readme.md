# rust_win32error

Error like wrapper for GetLastError for Rust.

## Example ##


cargo.toml
``` Rust
...
[dependencies]
winapi = "*"
user32-sys = "*"
kernel32-sys = "*"
rust_win32error = "0.7.0"
```

example.rs
``` Rust
extern crate kernel32;
extern crate rust_win32error;

use rust_win32error::*;
use kernel32::OpenProcess;

fn main()
{
    func();
}

fn func() {
    let process_terminate = 0x0001;
    let h = unsafe { OpenProcess(process_terminate, 0, 4) };
    let err = Win32Error::new();
    println!("{}", err); // => 5: Access is denied (or localized):

    let err = Win32Error::from(6); // => 6: Handle is invalid(or localized):
    println!("{}", err);
}
```