use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types::{c_char, c_void, StatusCode, PERSIST_DATA_MAX_LENGTH};
use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec;
use core::ffi::CStr;

/// Checks whether a value has been set for a given key.
pub fn exists(key: u32) -> bool {
    interface::persist_exists(key)
}

/// Gets the size of a value for a given key.
pub fn get_size(key: u32) -> Result<usize, StatusCode> {
    let size = interface::persist_get_size(key);
    if size < 0 {
        Err(StatusCode::from(size))
    } else {
        Ok(size as usize)
    }
}

/// Deletes the value of a key from persistent storage.
pub fn delete(key: u32) -> Result<(), StatusCode> {
    let status = interface::persist_delete(key);
    let code = StatusCode::from(status);
    if code == StatusCode::True || code == StatusCode::Success {
        Ok(())
    } else {
        Err(code)
    }
}

pub fn read_bool(key: u32) -> bool {
    interface::persist_read_bool(key)
}

pub fn read_int(key: u32) -> i32 {
    interface::persist_read_int(key)
}

/// Reads a blob of data into the provided mutable slice.
/// Returns the number of bytes read.
pub fn read_data(key: u32, buffer: &mut [u8]) -> Result<usize, StatusCode> {
    let bytes_read =
        interface::persist_read_data(key, buffer.as_mut_ptr() as *mut c_void, buffer.len());

    if bytes_read < 0 {
        Err(StatusCode::from(bytes_read))
    } else {
        Ok(bytes_read as usize)
    }
}

/// Safely allocates and returns a String from persistent storage.
pub fn read_string(key: u32) -> Result<String, StatusCode> {
    let size = get_size(key)?;
    if size == 0 {
        return Ok(String::new());
    }

    let mut buffer = vec![0u8; size];

    let bytes_read =
        interface::persist_read_string(key, buffer.as_mut_ptr() as *mut c_char, buffer.len());

    if bytes_read < 0 {
        return Err(StatusCode::from(bytes_read));
    }

    let c_str = CStr::from_bytes_until_nul(&buffer).unwrap_or_default();
    Ok(c_str.to_string_lossy().into_owned())
}

pub fn write_bool(key: u32, value: bool) -> Result<usize, StatusCode> {
    let status = interface::persist_write_bool(key, value);
    if status < 0 {
        Err(StatusCode::from(status))
    } else {
        Ok(status as usize)
    }
}

pub fn write_int(key: u32, value: i32) -> Result<usize, StatusCode> {
    let status = interface::persist_write_int(key, value);
    if status < 0 {
        Err(StatusCode::from(status))
    } else {
        Ok(status as usize)
    }
}

/// Writes a slice of bytes into persistent storage.
/// Returns an error if the slice exceeds PERSIST_DATA_MAX_LENGTH (256 bytes).
pub fn write_data(key: u32, data: &[u8]) -> Result<usize, StatusCode> {
    if data.len() > PERSIST_DATA_MAX_LENGTH {
        return Err(StatusCode::Range);
    }

    let bytes_written =
        interface::persist_write_data(key, data.as_ptr() as *const c_void, data.len());

    if bytes_written < 0 {
        Err(StatusCode::from(bytes_written))
    } else {
        Ok(bytes_written as usize)
    }
}

/// Safely writes a Rust string slice to persistent storage.
pub fn write_string(key: u32, value: &str) -> Result<usize, StatusCode> {
    let c_str = CString::new(value).map_err(|_| StatusCode::InvalidArgument)?;

    let bytes_written = interface::persist_write_string(key, c_str.as_ptr() as *const c_char);

    if bytes_written < 0 {
        Err(StatusCode::from(bytes_written))
    } else {
        Ok(bytes_written as usize)
    }
}
