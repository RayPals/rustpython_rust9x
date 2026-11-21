//! Optional post-Win98 APIs. Keep the base thread as a fiber on systems that
//! cannot convert it back; do not pretend that conversion occurred.
use libc::c_void;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
}

unsafe fn optional(name: &'static [u8]) -> *mut c_void {
    if cfg!(feature = "win98-force-legacy") {
        return std::ptr::null_mut();
    }
    let module = GetModuleHandleA(b"kernel32.dll\0".as_ptr());
    if module.is_null() {
        return std::ptr::null_mut();
    }
    GetProcAddress(module, name.as_ptr())
}

pub unsafe fn is_thread_a_fiber() -> bool {
    let proc = optional(b"IsThreadAFiber\0");
    if !proc.is_null() {
        let call: unsafe extern "system" fn() -> i32 = std::mem::transmute(proc);
        return call() != 0;
    }
    // GetCurrentFiber is the x86 TIB macro provided by stacker's C helper.
    // Before XP, an ordinary thread can expose NULL or the 0x1e00 sentinel.
    let fiber = super::__stacker_get_current_fiber() as usize;
    fiber != 0 && fiber != 0x1e00
}

pub unsafe fn convert_back_if_supported() -> Option<i32> {
    let proc = optional(b"ConvertFiberToThread\0");
    if proc.is_null() {
        // Win98: retain the base fiber for reuse until the thread exits.
        // Deleting the currently executing fiber would terminate the thread.
        return None;
    }
    let call: unsafe extern "system" fn() -> i32 = std::mem::transmute(proc);
    Some(call())
}

pub unsafe fn stack_guarantee_if_supported(size: *mut u32) -> Option<i32> {
    let proc = optional(b"SetThreadStackGuarantee\0");
    if proc.is_null() {
        return None;
    }
    let call: unsafe extern "system" fn(*mut u32) -> i32 = std::mem::transmute(proc);
    Some(call(size))
}
