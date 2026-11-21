//! x86 Win9x stack bounds without post-Win98 imports.
//! Read the current TIB (which Windows updates on fiber switches), and use
//! VirtualQuery to include the uncommitted portion of the reserved stack.
use core::ffi::c_void;

#[repr(C)]
struct MemoryBasicInformation {
    base_address: *mut c_void,
    allocation_base: *mut c_void,
    allocation_protect: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    kind: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualQuery(address: *const c_void, info: *mut MemoryBasicInformation, size: usize) -> usize;
    fn GetModuleHandleA(name: *const u8) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
    fn SetLastError(code: u32);
}

/// Optional remote-debugging APIs: missing exports produce a real unsupported
/// error, rather than preventing every interpreter invocation from loading.
pub unsafe fn toolhelp_call(name: &'static [u8], snapshot: *mut c_void, entry: *mut c_void) -> i32 {
    let Some(proc) = optional_kernel32(name) else {
        unsafe { SetLastError(120) }; // ERROR_CALL_NOT_IMPLEMENTED
        return 0;
    };
    let call: unsafe extern "system" fn(*mut c_void, *mut c_void) -> i32 = unsafe { core::mem::transmute(proc) };
    unsafe { call(snapshot, entry) }
}

pub fn optional_kernel32(name: &'static [u8]) -> Option<usize> {
    // All callers supply static NUL-terminated export names.
    unsafe {
        let module = GetModuleHandleA(b"kernel32.dll\0".as_ptr());
        if module.is_null() { return None; }
        let proc = GetProcAddress(module, name.as_ptr());
        (!proc.is_null()).then_some(proc as usize)
    }
}

pub fn image_base(address: *const c_void) -> Option<usize> {
    let mut info = core::mem::MaybeUninit::<MemoryBasicInformation>::uninit();
    let size = core::mem::size_of::<MemoryBasicInformation>();
    if unsafe { VirtualQuery(address, info.as_mut_ptr(), size) } != size {
        return None;
    }
    let base = unsafe { info.assume_init() }.allocation_base as usize;
    // The caller passes an actual function inside the already-loaded image.
    (base != 0 && base <= address as usize).then_some(base)
}

pub fn current_thread_stack_bounds() -> (usize, usize) {
    let marker = 0usize;
    let sp = &marker as *const usize as usize;
    let high: usize;
    let committed_low: usize;
    // SAFETY: x86 Windows NT_TIB stores StackBase/StackLimit at FS:[4]/FS:[8].
    // No hard-coded stack size or pointer from another thread/fiber is used.
    unsafe {
        core::arch::asm!("mov {0:e}, fs:[4]", out(reg) high,
                         options(nostack, readonly, preserves_flags));
        core::arch::asm!("mov {0:e}, fs:[8]", out(reg) committed_low,
                         options(nostack, readonly, preserves_flags));
    }
    let mut info = core::mem::MaybeUninit::<MemoryBasicInformation>::uninit();
    let size = core::mem::size_of::<MemoryBasicInformation>();
    // SAFETY: marker is live and the output buffer has the exact x86 ABI size.
    let queried = unsafe { VirtualQuery(sp as *const c_void, info.as_mut_ptr(), size) };
    if queried == size {
        let allocation_base = unsafe { info.assume_init() }.allocation_base as usize;
        // x86 Windows pages are 4 KiB. Exclude the bottom guard-page cushion;
        // there is no separate SetThreadStackGuarantee reservation on Win98.
        if let Some(low) = allocation_base.checked_add(4096) {
            if allocation_base != 0 && low < sp && sp < high {
                return (low, high);
            }
        }
    }
    // A failed query must not invent an 8 MiB reservation. The committed TIB
    // range is conservative: recursion may stop early but stays within bounds.
    if committed_low != 0 && committed_low < sp && sp < high {
        return (committed_low, high);
    }
    // Fail closed if neither source is valid; the VM will reject recursion.
    (sp, sp)
}
