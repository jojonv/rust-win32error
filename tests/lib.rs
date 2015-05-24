extern crate rust_win32error;
extern crate kernel32;

use std::error::Error;
use rust_win32error::*;
use self::kernel32::{ OpenProcess };



const UNKNOWN_ERROR_TEXT: &'static str = "Unknown error";


// Cause real error, and test whether there is concrete description
//
#[test]
#[allow(unused_variables)]
fn win32error_new_test_real_error_description()
{
    let terminate = 0x0001;
    let h = unsafe { OpenProcess(terminate, 0, 4) };
    let err = Win32Error::new();
    assert!(err.description() != UNKNOWN_ERROR_TEXT);
}

// Cause real error, and test whether there is right error code
//
#[test]
#[allow(unused_variables)]
fn win32error_new_test_real_error_code()
{
    let terminate = 0x0001;
    let h = unsafe { OpenProcess(terminate, 0, 4) };
    let err = Win32Error::new();
    assert_eq!(5, err.get_error_code());
}

