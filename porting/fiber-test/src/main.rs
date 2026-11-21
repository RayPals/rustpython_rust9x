#[path = "../../../crates/host_env/src/windows_legacy_stack.rs"]
mod legacy_stack;

fn check_stack_bounds() -> (usize, usize) {
    let marker = 0usize;
    let sp = &marker as *const usize as usize;
    let bounds = legacy_stack::current_thread_stack_bounds();
    assert!(bounds.0 < sp && sp < bounds.1, "invalid bounds: {:?}, SP={:x}", bounds, sp);
    assert!(sp - bounds.0 > 32 * 1024, "insufficient measured reserve");
    bounds
}

fn exercise() {
    let outer = check_stack_bounds();
    let mut calls = 0;
    for _ in 0..64 {
        stacker::grow(256 * 1024, || {
            calls += 1;
            let grown = check_stack_bounds();
            assert_ne!(grown, outer, "fiber must report its own stack");
            stacker::grow(256 * 1024, || {
                calls += 1;
                assert_ne!(check_stack_bounds(), grown, "nested fiber must report its own stack");
                assert!(stacker::remaining_stack().unwrap() > 32 * 1024);
            });
        });
    }
    assert_eq!(calls, 128);
}
fn main() {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleA(name: *const u8) -> *mut core::ffi::c_void;
        fn GetLastError() -> u32;
    }
    let image = legacy_stack::image_base(main as *const () as *const _).unwrap();
    assert_eq!(image, unsafe { GetModuleHandleA(core::ptr::null()) } as usize);
    assert!(legacy_stack::optional_kernel32(b"GetLastError\0").is_some());
    assert!(legacy_stack::optional_kernel32(b"RustPythonDefinitelyAbsentExport\0").is_none());
    assert_eq!(unsafe { legacy_stack::toolhelp_call(b"RustPythonDefinitelyAbsentExport\0", core::ptr::null_mut(), core::ptr::null_mut()) }, 0);
    assert_eq!(unsafe { GetLastError() }, 120);
    println!("WIN98_IMAGE_AND_OPTIONAL_API_OK");
    exercise();
    for _ in 0..4 {
        std::thread::spawn(exercise).join().unwrap();
    }
    println!("WIN98_FIBER_FALLBACK_OK: 640 callbacks, nested growth, base-fiber reuse, thread exit");
}
