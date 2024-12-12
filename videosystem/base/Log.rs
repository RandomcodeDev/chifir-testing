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
extern "C" {
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
    file: CString,
    function: CString,
    message: CString,
) {
    unsafe {
        Write(
            level,
            location.into(),
            is_addr,
            file.as_c_str().as_ptr(),
            function.as_c_str().as_ptr(),
            message.as_c_str().as_ptr(),
        )
    }
}

macro_rules! make_log_function {
    ($level:ident) => {
        #[macro_export]
        macro_rules! $level {
            ($$($$args:tt)*) => {
                use crate::function;
                use crate::Log::{Write, Level};
                Log::WriteSafe(Level::$level, line!(), false, std::ffi::CString::new(file!()).unwrap(), std::ffi::CString::new(function!()).unwrap(), std::ffi::CString::new(format!($$($$args)*)).unwrap())
            };
        }
        #[allow(unused)]
        pub use $level;
    };
}

make_log_function! {Trace}
make_log_function! {Debug}
make_log_function! {Info}
make_log_function! {Warning}
make_log_function! {Error}
make_log_function! {FatalError}
