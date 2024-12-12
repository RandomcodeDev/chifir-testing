use std::ffi::{CStr, CString};

#[repr(u32)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    FatalError,
}

#[link(name = "Base")]
unsafe extern "C" {
    #[link_name = "?Log_Write@@YAXW4LogLevel_t@@_K_NPEBD33ZZ"]
    pub fn Write(
        level: Level,
        location: u64,
        isAddress: bool,
        file: *const i8,
        function: *const i8,
        message: *const i8,
    );
}

pub fn WriteSafe(
    level: Level,
    location: u32,
    is_addr: bool,
    file: &str,
    function: &str,
    message: &str,
) {
    unsafe {
        Write(
            level,
            location.into(),
            is_addr,
            CString::new(file).unwrap().as_ptr(),
            CString::new(function).unwrap().as_ptr(),
            CString::new(message).unwrap().as_ptr(),
        )
    }
}
