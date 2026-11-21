# Windows 98 SE: loader compatibility status

## Latest: basic interactive shell fix
The target screenshot confirms that the loader-fix build reaches RustPython on Windows 98 SE, but a real console launch fails when `_pyrepl.main` imports the unavailable platform `os` module. This is distinct from earlier noninteractive `-c` success.

Changed `src/lib.rs::run_repl` to select the existing basic shell when `host_env` is not compiled in. Builds with `host_env` keep upstream enhanced-shell selection. No fake `os` module, broad catch-and-ignore, or environment workaround was added. The full `os` module remains unsupported by this minimal build.

Regression `../loader-fix/test_basic_repl.py` uses a real Unix PTY under Wine with no PYTHON_BASIC_REPL override. It failed before the patch with the exact screenshot traceback. After rebuilding it reaches `>>>` with no ImportError. A separate `-i` input stream verifies Hello World, persistent state (`42`), multiline loops, recovery after ZeroDivisionError, and clean exit. A real CRLF `.py` file with spaces in its path prints Hello World and exits 0. The test separates TTY startup from fed interactive input; it does not claim native Win98 keyboard input has been tested.

PE version and all 11 known-incompatible-import checks still pass. Earlier loader-fix and fiber-test artifacts are retained. Final Win98 SE prompt/keyboard acceptance awaits the user's test of the new basic-repl executable.

Target: Windows 98 SE (confirmed by user).
Baseline: e634ec61cf9601d69830326e495128434d47e8cf.
Toolchain: rust9x-probe / rustc 1.100.0-dev (252d2171a).
No commits or pushes. Original binaries preserved in ../loader-fix/.

## Current changes
- PE OS/subsystem version 4.0; intended stack reserve restored by final linker wrapper.
- Genuine legacy CryptoAPI randomness replaces the former failing zero-fill ProcessPrng shim.
- Vendored stacker 0.1.25 and clipboard-win 5.4.1 with licenses under compat/. Shared Cargo registry sources are untouched.
- x86 rust9x stacker resolves newer fiber APIs dynamically. When conversion back to an ordinary thread is unavailable, retain/reuse the base fiber instead of claiming a successful conversion or deleting the executing fiber.
- RustPython host_env x86 rust9x stack bounds now use current TIB StackBase/StackLimit plus VirtualQuery reservation information. Exclude a 4 KiB guard cushion. Failed queries use conservative committed bounds; invalid bounds fail closed. No GetCurrentThreadStackLimits or SetThreadStackGuarantee import remains.
- Runtime module identification uses VirtualQuery AllocationBase of the actual runtime function, preserving borrowed-image semantics without GetModuleHandleExW.
- The host_env helper used exclusively by time.monotonic uses a shared Instant epoch on x86 rust9x. Its unspecified monotonic epoch is process-relative; no static GetTickCount64 import or 32-bit wrap workaround is used.
- Wide Toolhelp APIs for optional remote debugging are resolved dynamically. Unsupported exports return failure/ERROR_CALL_NOT_IMPLEMENTED, not fabricated success. This makes remote debugging unavailable on systems lacking those exports without blocking interpreter loading.
- Clipboard impersonation APIs are optional on x86 rust9x. Check both entry and cleanup API availability before impersonating; absence reports failure as already allowed by upstream's optional impersonation guard. Do not substitute successful token operations.
- Other targets retain upstream implementations.

## Verified tests
- RED: original fiber-slice executable fails the stack-import guard with GetCurrentThreadStackLimits and SetThreadStackGuarantee.
- RED: broader known-import regression catches 9 remaining members of the 11-API ban list before this slice.
- GREEN: final executable passes both tests: all 4 newer stack/fiber static imports and all 11 previously identified post-Win98 imports are absent.
- Final PE is x86 PE32, OS/subsystem 4.0. Current static DLLs are kernel32.dll, advapi32.dll, USER32.dll, MSVCRT.dll; 142 imported symbols. This is NOT a complete stock-Win98 export allowlist audit.
- Incremental release rebuild exits 0 (existing resource/dead-code and ProcessPrng duplicate-import warnings remain).
- Standalone probe compiles the exact production stack fallback source and forces stacker's 3 optional newer APIs unavailable under Wine.
- Probe exits 0 after checking valid bounds on main/thread stacks and distinct bounds in nested fibers; 640 callbacks succeed across repeated/nested growth, base-fiber reuse, and thread exit.
- Probe verifies runtime image AllocationBase against GetModuleHandleA(NULL), resolves an existing API, and returns failure/error 120 for a deliberately absent optional Toolhelp export.
- Wine interpreter tests exit 0 for math, random generator operation, monotonic time, sleep, and recursion producing RecursionError rather than a native crash.
- Plain `-c print(...)` and plain `-c import time; ...` also exit 0. Correction to earlier notes: normal command startup does NOT require -S. The earlier failure was from explicitly importing os in the randomness test.
- Explicit `import os` still fails: ImportError: no os specific module found. The minimal feature selection omits host_env support. This is an unresolved library capability gap, not a loader failure.
- Windows 98 SE runtime acceptance remains UNVERIFIED. Wine, even with absent-API simulation, is not a real Win98 test.

## Reproduction / evidence
- python ../loader-fix/rebuild.py uses cached dependency artifacts, SDKs and prior CRT shims. It is an incremental probe, NOT a clean release builder.
- python ../loader-fix/test_stack_imports.py target/i686-rust9x-windows-msvc/release/rustpython.exe
- python ../loader-fix/test_known_win98_imports.py target/i686-rust9x-windows-msvc/release/rustpython.exe
- python ../test_pe_versions.py target/i686-rust9x-windows-msvc/release/rustpython.exe
- ../loader-fix/fiber-test/ contains the fallback probe source.
- ../loader-fix/final-stack-probe-wine.log and final-interpreter-smoke.log contain executed results.
- ../loader-fix/final-imports.txt is the final import inventory.

## Next acceptance boundary
Run the newly named loader-fix executable on Windows 98 SE. Retain the old fiber-test executable for comparison. Confirm actual loader success and REPL/code execution before calling this a working Win98 port. Do not hide the missing os/host-environment capability, potential unicows deployment requirements, or target-only runtime failures behind Wine success.
