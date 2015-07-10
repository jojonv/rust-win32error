# rust_win32error

Error like wrapper for GetLastError for Rust.

## Example ##


cargo.toml

**0.8.0**
``` Rust
...
[dependencies]
kernel32-sys = "*"
rust-win32error = "0.8.0"
```

**0.7.0**
``` Rust
...
[dependencies]
winapi = "*"
user32-sys = "*"
kernel32-sys = "*"
rust_win32error = "0.7.0"
```

main.rs
``` Rust
extern crate kernel32;
extern crate rust_win32error;

use rust_win32error::*;
use kernel32::OpenProcess;
// needs to be brought so `description` function can be used
use std::error::Error;

fn main() {
    func();
}

fn func() {
    let process_terminate = 0x0001;
    let h = unsafe { OpenProcess(process_terminate, 0, 4) }; 
    let err = Win32Error::new();
    println!("{}", err); // => 5: Access is denied (or localized):

    let err = Win32Error::from(6); // => 6: Handle is invalid (or localized):
    println!("{}", err);

    println!("{}", err.description()); // => Handle is invalid (or localized)
    println!("Error code is {}", err.get_error_code()); // => Error code is 6

    // pass some crazy error
    let err = Win32Error::from(885848);
    println!("{}", err); // => 885848: Unknown error
}

// From 0.8.0 and above
fn get_result_ok() -> Win32Result<u32> {
    Err(Win32Error::new())
}
```