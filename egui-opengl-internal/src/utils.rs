use std::ffi::CString;
use std::ptr::null_mut;
use windows::{
    core::PCSTR,
    Win32::{
                Graphics::OpenGL::wglGetProcAddress,
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA, GetProcAddress},
        }, Foundation::HMODULE,
    },
};

pub unsafe fn get_proc_address(function_name: &str) -> *const usize {
    let opengl32 = get_module("opengl32.dll");
    let c = CString::new(function_name).unwrap();
    let process_address = GetProcAddress(opengl32, PCSTR::from_raw(c.as_ptr() as *const u8));

    if let Some(process_address) = process_address {
        return process_address as _;
    }

    let c_proc_name = CString::new(function_name).unwrap();
    let process_address = wglGetProcAddress(PCSTR::from_raw(c_proc_name.as_ptr() as *const u8));

    if let Some(process_address) = process_address {
        return process_address as _;
    }

    // this shouldn't silently error tbh, but im going to copy the old behavior
    std::ptr::null()
}

pub fn get_module(module_name: &str) -> HMODULE {
    unsafe {
        let o = CString::new(module_name).unwrap();
        let module = GetModuleHandleA(PCSTR::from_raw(o.as_ptr() as *const u8));

        if let Ok(module) = module {
            module
        } else {
            // this also shouldn't silently error
            HMODULE(null_mut())
        }
    }
}

pub fn alloc_console() {
    unsafe {
        let _ = AllocConsole();
    }
}

pub fn free_console() {
    unsafe {
        let _ = FreeConsole();
    }
}

pub fn unload() {
    unsafe {
        let module = get_module("example_wnd.dll");
        FreeLibraryAndExitThread(module, 0);
    }
}
