#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]

use chifir::{Base, Debug, Info, Log};
use std::{borrow::BorrowMut, ffi::CStr, mem, ptr};
use winit::{application::ApplicationHandler, event_loop::EventLoop, window::Window};

const GAME_NAME: &str = "False King";

#[repr(C)]
struct CWinitVideoSystem {
    vfptr: *mut CWinitVideoSystem_VTable,
    vtable: CWinitVideoSystem_VTable,

    event_loop: EventLoop<CWinitVideoSystem>,
    window: Window,
}

impl ApplicationHandler for CWinitVideoSystem {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        todo!()
    }
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

impl CWinitVideoSystem_VTable {
    unsafe extern "C" fn dtor(this: &mut CWinitVideoSystem) {}

    unsafe extern "C" fn Initialize(this: &mut CWinitVideoSystem) -> bool {
        Info!("Initializing winit video system");

        Debug!("Creating event loop");
        this.event_loop = match EventLoop::with_user_event().build() {
            Ok(event_loop) => event_loop,
            Err(err) => Base::Quit(format!("Failed to create event loop: {err}").as_str()),
        };

        //Debug!("Creating window");

        true
    }

    unsafe extern "C" fn Shutdown(this: &mut CWinitVideoSystem) {
        // TODO: drop members properly
        Info!("winit video system shut down");
    }

    unsafe extern "C" fn GetName(this: &mut CWinitVideoSystem) -> *const i8 {
        "winit Video\0".as_ptr() as *const i8
    }

    unsafe extern "C" fn GetVersion(this: &mut CWinitVideoSystem) -> u32 {
        1
    }

    unsafe extern "C" fn Update(this: &mut CWinitVideoSystem) -> bool {
        true
    }

    unsafe extern "C" fn GetTitle(this: &mut CWinitVideoSystem) -> *const i8 {
        "False King\0".as_ptr() as *const i8
    }

    unsafe extern "C" fn SetTitle(this: &mut CWinitVideoSystem, title: *const i8) {}

    unsafe extern "C" fn GetWidth(this: &mut CWinitVideoSystem) -> u32 {
        0
    }

    unsafe extern "C" fn GetHeight(this: &mut CWinitVideoSystem) -> u32 {
        0
    }

    unsafe extern "C" fn GetX(this: &mut CWinitVideoSystem) -> u32 {
        0
    }

    unsafe extern "C" fn GetY(this: &mut CWinitVideoSystem) -> u32 {
        0
    }

    unsafe extern "C" fn Resized(this: &mut CWinitVideoSystem) -> bool {
        false
    }

    unsafe extern "C" fn Focused(this: &mut CWinitVideoSystem) -> bool {
        true
    }

    unsafe extern "C" fn GetDpi(this: &mut CWinitVideoSystem) -> f32 {
        1.0
    }

    unsafe extern "C" fn GetHandle(this: &mut CWinitVideoSystem) -> u64 {
        0
    }
}

#[unsafe(no_mangle)]
extern "C" fn CreateInterface() -> *mut CWinitVideoSystem {
    let rawSystem: *mut CWinitVideoSystem =
        unsafe { mem::transmute(Base::Alloc(mem::size_of::<CWinitVideoSystem>(), 8)) };

    Debug!("Creating CWinitVideoSystem at 0x{:X}", rawSystem as usize);

    let system = unsafe { &mut (*rawSystem) };
    system.vfptr = ptr::addr_of_mut!(system.vtable);
    system.vtable.dtor = CWinitVideoSystem_VTable::dtor;
    system.vtable.Initialize = CWinitVideoSystem_VTable::Initialize;
    system.vtable.Shutdown = CWinitVideoSystem_VTable::Shutdown;
    system.vtable.GetName = CWinitVideoSystem_VTable::GetName;
    system.vtable.GetVersion = CWinitVideoSystem_VTable::GetVersion;

    system.vtable.Update = CWinitVideoSystem_VTable::Update;
    system.vtable.GetTitle = CWinitVideoSystem_VTable::GetTitle;
    system.vtable.SetTitle = CWinitVideoSystem_VTable::SetTitle;
    system.vtable.GetWidth = CWinitVideoSystem_VTable::GetWidth;
    system.vtable.GetHeight = CWinitVideoSystem_VTable::GetHeight;
    system.vtable.GetX = CWinitVideoSystem_VTable::GetX;
    system.vtable.GetY = CWinitVideoSystem_VTable::GetY;
    system.vtable.Resized = CWinitVideoSystem_VTable::Resized;
    system.vtable.Focused = CWinitVideoSystem_VTable::Focused;
    system.vtable.GetDpi = CWinitVideoSystem_VTable::GetDpi;
    system.vtable.GetHandle = CWinitVideoSystem_VTable::GetHandle;

    system
}
