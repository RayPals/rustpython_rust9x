//! Preserve the original optional impersonation behavior without requiring
//! NT token APIs at process-load time on Windows 98.
use crate::types::{BOOL, HANDLE};
use core::ffi::c_void;

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn SetLastError(error: u32);
}

unsafe fn security_pair() -> Option<(*mut c_void, *mut c_void)> {
    let module = GetModuleHandleA(b"advapi32.dll\0".as_ptr());
    if !module.is_null() {
        let impersonate = GetProcAddress(module, b"ImpersonateAnonymousToken\0".as_ptr());
        let revert = GetProcAddress(module, b"RevertToSelf\0".as_ptr());
        // Never enter impersonation unless the matching cleanup is available.
        if !impersonate.is_null() && !revert.is_null() {
            return Some((impersonate, revert));
        }
    }
    SetLastError(120); // ERROR_CALL_NOT_IMPLEMENTED; do not fabricate success.
    None
}

#[allow(non_snake_case)]
pub unsafe fn ImpersonateAnonymousToken(thread: HANDLE) -> BOOL {
    let Some((proc, _)) = security_pair() else { return 0; };
    let call: unsafe extern "system" fn(HANDLE) -> BOOL = core::mem::transmute(proc);
    call(thread)
}

#[allow(non_snake_case)]
pub unsafe fn RevertToSelf() -> BOOL {
    let Some((_, proc)) = security_pair() else { return 0; };
    let call: unsafe extern "system" fn() -> BOOL = core::mem::transmute(proc);
    call()
}
