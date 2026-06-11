#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MenuIndexRef {
    internal: *mut pebble_sys::MenuIndex,
}

impl MenuIndexRef {
    pub fn as_ptr(&self) -> *mut pebble_sys::MenuIndex {
        self.internal
    }

    pub fn section(&self) -> u16 {
        debug_assert!(!self.internal.is_null(), "MenuIndex pointer was null!");
        unsafe { (*self.internal).section }
    }

    pub fn set_section(&mut self, section: u16) {
        debug_assert!(!self.internal.is_null(), "MenuIndex pointer was null!");
        unsafe {
            (*self.internal).section = section;
        }
    }

    pub fn row(&self) -> u16 {
        debug_assert!(!self.internal.is_null(), "MenuIndex pointer was null!");
        unsafe { (*self.internal).row }
    }

    pub fn set_row(&mut self, row: u16) {
        debug_assert!(!self.internal.is_null(), "MenuIndex pointer was null!");
        unsafe {
            (*self.internal).row = row;
        }
    }
}

impl From<*mut pebble_sys::MenuIndex> for MenuIndexRef {
    fn from(internal: *mut pebble_sys::MenuIndex) -> Self {
        Self { internal }
    }
}
