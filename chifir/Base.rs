use std::ffi::CString;

#[repr(C)]
pub struct DateTime {
    year: u32,
    month: u32,
    day: u32,

    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
    weekDay: u32,
}

impl DateTime {
    pub fn now(&mut self) {
        unsafe {
            super::Plat::GetDateTime(self);
        }
    }
}

#[link(name = "Base")]
unsafe extern "C" {
    #[link_name = "?Base_Alloc@@YAPEAX_J0@Z"]
    pub fn Alloc(size: usize, align: usize) -> *mut ();
    #[link_name = "?Base_Realloc@@YAPEAXPEAX_J@Z"]
    pub fn Realloc(block: *mut (), newSize: usize) -> *mut ();
    #[link_name = "?Base_Free@@YAXPEAX@Z"]
    pub fn Free(block: *mut ());
    #[link_name = "?Base_AbortSafe@@YAXHPEBD@Z"]
    pub fn AbortSafe(code: u32, msg: *const i8) -> !;
}

pub fn Quit(msg: &str) -> ! {
    FatalError!("{msg}");
    let rawMsg = CString::new(msg).unwrap();
    unsafe { AbortSafe(1, rawMsg.as_ptr()); }
}
