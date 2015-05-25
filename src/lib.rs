
//! Error-like wrapper around win32 GetLastError and FormatMessage
extern crate kernel32;

use std::ptr;
use std::slice;
use std::fmt;
use std::error::Error;

use self::kernel32::{ GetLastError, FormatMessageW};


const FORMAT_MESSAGE_FROM_STRING: u32 = 0x00000400;
const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x00000100;
const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;
const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
const FORMAT_MESSAGE_ARGUMENT_ARRAY: u32 = 0x00002000;

const UNKNOWN_ERROR_TEXT: &'static str = "Unknown error";


pub type Win32Result<T> = Result<T, Win32Error>;

#[derive(Debug, Clone)]
pub struct Win32Error
{
    // Error code returned by GetLastError or passed as an arg
    //
    error_code: u32,
    // Message returned by FormatMessage
    //
    description: Option<String>,
}

fn init_from_error_code(errno: u32) -> Win32Error
{
    unsafe
    {
        let buff_size = 256;
        let mut buff: Vec<u16> = Vec::with_capacity(buff_size);
        for x in 0 .. buff_size
        {
            buff.push(0);
        }

        // Should be zero or num of chars copied
        //
        let chars_copied = FormatMessageW(
            FORMAT_MESSAGE_IGNORE_INSERTS
            | FORMAT_MESSAGE_FROM_SYSTEM
            | FORMAT_MESSAGE_ARGUMENT_ARRAY
            , ptr::null()
            , errno
            , 0
            , buff.as_mut_ptr()
            , (buff_size + 1) as u32
            , ptr::null_mut());

        // Very likely wrong err number was passed, and no message exists
        //
        if chars_copied == 0
        {
            return Win32Error { error_code: errno, description: None };
        }


        // Remove newline - "\r\n" and punctuation or space from the message
        //
        let mut curr_char: usize = chars_copied as usize;
        while curr_char > 0
        {
            let ch = buff[curr_char];

            if ch >= ' ' as u16 { break; }
            curr_char -= 1;
        }
        let sl = slice::from_raw_parts(buff.as_ptr(), curr_char);
        let err_msg = String::from_utf16(sl);

        let description = match err_msg { Ok(s) => Some(s), _ => None };
        Win32Error { error_code: errno, description: description }
    }
}


macro_rules! impl_from_trait
{
    ($($t:ty), *) =>
    {
        $(
            impl From<$t> for Win32Error
            {
                fn from(errno: $t) -> Self
                {
                    init_from_error_code(errno as u32)
                }
            }
        )*
    };
}

macro_rules! impl_into_trait
{
    ($($t:ty), *) =>
    {
        $(
            impl Into<$t> for Win32Error
            {
                fn into(self)  -> $t
                {
                    self.error_code as $t
                }
            }
        )*
    };
}

impl_from_trait!(i32, i16, i8, u32, u16, u8);
impl_into_trait!(i32, i16, i8, u32, u16, u8);

impl fmt::Display for Win32Error
{
    /// Prints an error description in the following format:
    /// **Error code**: **Error message**, eg. 5: Access denied
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_win32error::*;
    /// let err = Win32Error::new();
    /// println!("{}", err);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.description.as_ref()
        {
            Some(s) => format!("{}: {}", self.error_code, s),
            None => format!("{}: {}", self.error_code, UNKNOWN_ERROR_TEXT)
        }.fmt(f)
    }
}

impl Win32Error
{
    /// Initializes new Win32Error instance.
    /// Function behind the scenes calls native GetLastError, and uses error code it returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_win32error::*;
    /// let err = Win32Error::new();
    /// ```
    pub fn new() -> Self { Self::from(unsafe { GetLastError() }) }

    /// Retrieves error code returned by GetLastError, or one that was passed through `std::convert::From::from` call
    /// # Examples
    ///
    /// ```
    /// use rust_win32error::*;
    /// let err = Win32Error::new();
    /// assert_eq!(err.get_error_code(), 0);
    /// ```
    pub fn get_error_code(&self) -> u32 { self.error_code }
}


/// Retrieves localized description of the error, with one exception that's description
/// of the error couldn't be retrieved, in which case *Unknown error* (in english) is returned.
impl Error for Win32Error
{
    fn description(&self) -> &str
    {
        match self.description.as_ref()
        {
            Some(s) => s,
            None => UNKNOWN_ERROR_TEXT
        }
    }
    fn cause(&self) -> Option<&Error> { None }
}


#[cfg(test)]
mod test
{
    use std::error::Error;
    use super::*;

    // ugly duplication
    //
    const UNKNOWN_ERROR_TEXT: &'static str = "Unknown error";


    #[test]
    fn win32error_new_test()
    {
        let err = Win32Error::new();
        assert_eq!(0, err.get_error_code());
    }

    // Test whether passed error code is returned from get_error_code
    //
    #[test]
    #[allow(unused_variables)]
    fn win32error_from_test_unknown_error_code()
    {
        let errno = 99999;
        let err = Win32Error::from(errno);
        assert_eq!(err.get_error_code(), errno);
    }

    #[test]
    #[allow(unused_variables)]
    fn win32error_from_test_unknown_error_description()
    {
        let errno = 99999;
        let err = Win32Error::from(errno);
        assert_eq!(err.description(), UNKNOWN_ERROR_TEXT);
    }
}

