#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

#![feature(macro_metavar_expr)]

mod base;

use base::{Base, Log};
use std::{borrow::BorrowMut, ffi::CStr, mem, ptr};
use winit::

const GAME_NAME: &str = "False King";

#[repr(C)]
struct CWinitVideoSystem {
    vfptr: *mut CWinitVideoSystem_VTable,
    vtable: CWinitVideoSystem_VTable,
}

#[repr(C)]
struct CWinitVideoSystem_VTable {
    // ISystem
    dtor: unsafe extern "C" fn(&mut CWinitVideoSystem),
    Initialize: unsafe extern "C" fn(&mut CWinitVideoSystem) -> bool,
    Shutdown: unsafe extern "C" fn(&mut CWinitVideoSystem),
    GetName: unsafe extern "C" fn(&mut CWinitVideoSystem) -> *const i8,
    GetVersion: unsafe extern "C" fn(&mut CWinitVideoSystem) -> u32,

    // IVideoSystem
    Update: unsafe extern "C" fn(&mut CWinitVideoSystem) -> bool,
    GetTitle: unsafe extern "C" fn(&mut CWinitVideoSystem) -> *const i8,
    SetTitle: unsafe extern "C" fn(&mut CWinitVideoSystem, *const i8),
    GetWidth: unsafe extern "C" fn(&mut CWinitVideoSystem) -> u32,
    GetHeight: unsafe extern "C" fn(&mut CWinitVideoSystem) -> u32,
    GetX: unsafe extern "C" fn(&mut CWinitVideoSystem) -> u32,
    GetY: unsafe extern "C" fn(&mut CWinitVideoSystem) -> u32,
    Resized: unsafe extern "C" fn(&mut CWinitVideoSystem) -> bool,
    Focused: unsafe extern "C" fn(&mut CWinitVideoSystem) -> bool,
    GetDpi: unsafe extern "C" fn(&mut CWinitVideoSystem) -> f32,
    GetHandle: unsafe extern "C" fn(&mut CWinitVideoSystem) -> u64,
}

unsafe extern "C" fn CWinitVideoSystem_dtor(_this: &mut CWinitVideoSystem) {}

unsafe extern "C" fn CWinitVideoSystem_Initialize(this: &mut CWinitVideoSystem) -> bool {
    let self_ = this.borrow_mut();
    Log::Info!("Initializing winit video system");
    true
}

unsafe extern "C" fn CWinitVideoSystem_Shutdown(this: &mut CWinitVideoSystem) {
    let self_ = this.borrow_mut();
    Log::Info!("winit video system shut down");
}

unsafe extern "C" fn CWinitVideoSystem_GetName(this: &mut CWinitVideoSystem) -> *const i8 {
    "winit Video\0".as_ptr() as *const i8
}

unsafe extern "C" fn CWinitVideoSystem_GetVersion(this: &mut CWinitVideoSystem) -> u32 {
    1
}

unsafe extern "C" fn CWinitVideoSystem_Update(this: &mut CWinitVideoSystem) -> bool {
    let self_ = this.borrow_mut();
    true
}

unsafe extern "C" fn CWinitVideoSystem_GetTitle(this: &mut CWinitVideoSystem) -> *const i8 {
    let self_ = this.borrow_mut();
    "False King\0".as_ptr() as *const i8
}

unsafe extern "C" fn CWinitVideoSystem_SetTitle(this: &mut CWinitVideoSystem, title: *const i8) {
    let self_ = this.borrow_mut();
}

unsafe extern "C" fn CWinitVideoSystem_GetWidth(this: &mut CWinitVideoSystem) -> u32 {
    let self_ = this.borrow_mut();
    0
}

unsafe extern "C" fn CWinitVideoSystem_GetHeight(this: &mut CWinitVideoSystem) -> u32 {
    let self_ = this.borrow_mut();
    0
}

unsafe extern "C" fn CWinitVideoSystem_GetX(this: &mut CWinitVideoSystem) -> u32 {
    let self_ = this.borrow_mut();
    0
}

unsafe extern "C" fn CWinitVideoSystem_GetY(this: &mut CWinitVideoSystem) -> u32 {
    let self_ = this.borrow_mut();
    0
}

unsafe extern "C" fn CWinitVideoSystem_Resized(this: &mut CWinitVideoSystem) -> bool {
    let self_ = this.borrow_mut();
    false
}

unsafe extern "C" fn CWinitVideoSystem_Focused(this: &mut CWinitVideoSystem) -> bool {
    let self_ = this.borrow_mut();
    true
}

unsafe extern "C" fn CWinitVideoSystem_GetDpi(this: &mut CWinitVideoSystem) -> f32 {
    let self_ = this.borrow_mut();
    1.0
}

unsafe extern "C" fn CWinitVideoSystem_GetHandle(this: &mut CWinitVideoSystem) -> u64 {
    let self_ = this.borrow_mut();
    0
}

#[no_mangle]
unsafe extern "C" fn CreateInterface() -> *mut CWinitVideoSystem {
    let rawSystem: *mut CWinitVideoSystem =
        mem::transmute(Base::Alloc(mem::size_of::<CWinitVideoSystem>(), 8));

    Log::Debug!("Creating CWinitVideoSystem at 0x{:X}", rawSystem as usize);

    let system = &mut (*rawSystem);
    system.vfptr = ptr::addr_of_mut!(system.vtable);
    system.vtable.dtor = CWinitVideoSystem_dtor;
    system.vtable.Initialize = CWinitVideoSystem_Initialize;
    system.vtable.Shutdown = CWinitVideoSystem_Shutdown;
    system.vtable.GetName = CWinitVideoSystem_GetName;
    system.vtable.GetVersion = CWinitVideoSystem_GetVersion;

    system.vtable.Update = CWinitVideoSystem_Update;
    system.vtable.GetTitle = CWinitVideoSystem_GetTitle;
    system.vtable.SetTitle = CWinitVideoSystem_SetTitle;
    system.vtable.GetWidth = CWinitVideoSystem_GetWidth;
    system.vtable.GetHeight = CWinitVideoSystem_GetHeight;
    system.vtable.GetX = CWinitVideoSystem_GetX;
    system.vtable.GetY = CWinitVideoSystem_GetY;
    system.vtable.Resized = CWinitVideoSystem_Resized;
    system.vtable.Focused = CWinitVideoSystem_Focused;
    system.vtable.GetDpi = CWinitVideoSystem_GetDpi;
    system.vtable.GetHandle = CWinitVideoSystem_GetHandle;

    system
}
