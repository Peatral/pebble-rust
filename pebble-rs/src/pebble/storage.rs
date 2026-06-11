use alloc::string::String;
use alloc::vec;
use core::ffi::{CStr, c_char, c_void};
use pebble_sys::{
    PERSIST_DATA_MAX_LENGTH, StatusCode, StatusCode_E_RANGE, StatusCode_S_SUCCESS,
    StatusCode_S_TRUE,
};

/// Checks whether a value has been set for a given key.
pub fn exists(key: u32) -> bool {
    unsafe { pebble_sys::persist_exists(key) }
}

/// Gets the size of a value for a given key.
pub fn get_size(key: u32) -> Result<usize, StatusCode> {
    unsafe {
        let size = pebble_sys::persist_get_size(key);
        if size < 0 {
            Err(StatusCode::from(size))
        } else {
            Ok(size as usize)
        }
    }
}

/// Deletes the value of a key from persistent storage.
pub fn delete(key: u32) -> Result<(), StatusCode> {
    unsafe {
        let status = pebble_sys::persist_delete(key);
        let code = StatusCode::from(status);
        if code >= 0 { Ok(()) } else { Err(code) }
    }
}

pub fn read_bool(key: u32) -> bool {
    unsafe { pebble_sys::persist_read_bool(key) }
}

pub fn read_int(key: u32) -> i32 {
    unsafe { pebble_sys::persist_read_int(key) }
}

/// Reads a blob of data into the provided mutable slice.
/// Returns the number of bytes read.
pub fn read_data(key: u32, buffer: &mut [u8]) -> Result<usize, StatusCode> {
    unsafe {
        let bytes_read =
            pebble_sys::persist_read_data(key, buffer.as_mut_ptr() as *mut c_void, buffer.len());

        if bytes_read < 0 {
            Err(StatusCode::from(bytes_read))
        } else {
            Ok(bytes_read as usize)
        }
    }
}

/// Safely allocates and returns a String from persistent storage.
pub fn read_string(key: u32) -> Result<String, StatusCode> {
    let size = get_size(key)?;
    if size == 0 {
        return Ok(String::new());
    }

    let mut buffer = vec![0u8; size];

    unsafe {
        let bytes_read =
            pebble_sys::persist_read_string(key, buffer.as_mut_ptr() as *mut c_char, buffer.len());

        if bytes_read < 0 {
            return Err(StatusCode::from(bytes_read));
        }

        let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
        Ok(c_str.to_string_lossy().into_owned())
    }
}

pub fn write_bool(key: u32, value: bool) -> Result<usize, StatusCode> {
    unsafe {
        let status = pebble_sys::persist_write_bool(key, value);
        if status == StatusCode::S_TRUE || status == StatusCode::S_SUCCESS {
            Err(StatusCode::from(status))
        } else {
            Ok(status as usize)
        }
    }
}

pub fn write_int(key: u32, value: i32) -> Result<usize, StatusCode> {
    unsafe {
        let status = pebble_sys::persist_write_int(key, value);
        if status < 0 {
            Err(StatusCode::from(status))
        } else {
            Ok(status as usize)
        }
    }
}

/// Writes a slice of bytes into persistent storage.
/// Returns an error if the slice exceeds PERSIST_DATA_MAX_LENGTH (256 bytes).
pub fn write_data(key: u32, data: &[u8]) -> Result<usize, StatusCode> {
    if data.len() > PERSIST_DATA_MAX_LENGTH as usize {
        return Err(StatusCode::E_RANGE);
    }

    unsafe {
        let bytes_written =
            pebble_sys::persist_write_data(key, data.as_ptr() as *const c_void, data.len());

        if bytes_written < 0 {
            Err(StatusCode::from(bytes_written))
        } else {
            Ok(bytes_written as usize)
        }
    }
}

/// Safely writes a Rust string slice to persistent storage.
pub fn write_string(key: u32, value: &CStr) -> Result<usize, StatusCode> {
    unsafe {
        let bytes_written = pebble_sys::persist_write_string(key, value.as_ptr());

        if bytes_written < 0 {
            Err(StatusCode::from(bytes_written))
        } else {
            Ok(bytes_written as usize)
        }
    }
}
