# RustPython for Windows 98 SE via rust9x

An experimental Windows 98 SE port of **[RustPython](https://github.com/RustPython/RustPython)** using **[rust9x](https://github.com/rust9x/rust)**, the Rust toolchain/standard-library port for legacy Windows.

- **RustPython upstream:** https://github.com/RustPython/RustPython
- **rust9x compiler and releases:** https://github.com/rust9x/rust
- **rust9x documentation and limitations:** https://github.com/rust9x/docs
- **Original RustPython README:** [README.upstream.md](README.upstream.md)

This is an independent porting experiment, not an official RustPython or rust9x release. Credit for the interpreter and legacy Rust platform support belongs to their respective upstream contributors.

## Status: experimental, incomplete

The local port produced an x86 Windows executable and passed focused Wine startup, interactive-shell, script, import, and stack/fiber fallback tests. Earlier target feedback indicated that a build reached RustPython on Windows 98 SE but failed during enhanced-shell startup. The current basic-shell fix has been tested under Wine; final native Windows 98 SE prompt/keyboard acceptance is still pending.

**This is not a complete, generally compatible Python distribution for Windows 98.**

Known limits:

- The minimal feature build has no working platform `os` module (`import os` fails). It selects RustPython's basic shell instead of the enhanced shell that depends on `os`.
- The tested target is `i686-rust9x-windows-msvc`, whose default CPU is **Pentium 4 / SSE2**, not a generic i686, Pentium, or i486 target. The i586 linker configuration is not evidence of a tested i586 port.
- Checking 11 known incompatible Windows imports is not a full stock-Win98 export audit.
- Wine success is not proof of Windows 98 runtime compatibility. Python conformance, pip, native extensions, full standard-library coverage, and older CPUs have not been validated.
- The successful local link depended on SDK/CRT/import-library configuration and cached artifacts. A clean end-to-end release builder is not included or claimed.

See [WIN98SE_PORT_STATUS.md](WIN98SE_PORT_STATUS.md) for the detailed development record and [porting/README.md](porting/README.md) for the published tests and evidence.

## Source baseline and compatibility changes

RustPython baseline: [`e634ec61cf9601d69830326e495128434d47e8cf`](https://github.com/RustPython/RustPython/commit/e634ec61cf9601d69830326e495128434d47e8cf).

Toolchain used locally: `rust9x-1.100.0-v2`, `rustc 1.100.0-dev (252d2171a)`, registered as `rust9x-probe`.

The repository contains the upstream source snapshot plus the port changes:

- PE OS/subsystem 4.0 linker settings.
- Local `stacker` and `clipboard-win` compatibility patches under `compat/`, retaining their license terms.
- Legacy x86 stack bounds, dynamic optional Windows API resolution, and process-relative monotonic timing.
- Basic REPL selection for builds without `host_env`.
- CryptoAPI-backed `ProcessPrng` shim source and focused PE/import/REPL regressions under `porting/`.

## Build starting point — not a complete recipe

Follow the [rust9x Linux/MSVC setup documentation](https://github.com/rust9x/docs/blob/main/linux-msvc.md) and supply a genuine Windows SDK plus suitable legacy CRT/import libraries. Register the extracted toolchain without changing your default:

```sh
rustup toolchain link rust9x-probe /path/to/extracted/rust9x
```

The feature selection used for the local experiment was:

```sh
cargo +rust9x-probe build --release \
  --target i686-rust9x-windows-msvc \
  --no-default-features \
  --features stdlib,stdio,importlib,freeze-stdlib -j 4
```

**The command alone does not reproduce the final executable.** Target-scoped Clang/archiver settings, real SDK headers, compatible import libraries, compilation/linking of the CryptoAPI shim, and final-link configuration are additional requirements. Proprietary SDKs, compiler packages, toolchain archives, cached build objects, and executables are deliberately excluded. Upstream GitHub Actions workflows are not activated by this source publication.

## Licensing

RustPython retains its [MIT license](LICENSE) and upstream notices. Vendored `stacker` retains its MIT/Apache-2.0 licenses; vendored `clipboard-win` uses the Boost Software License 1.0. See [THIRD_PARTY_PORT_NOTICES.md](THIRD_PARTY_PORT_NOTICES.md). Nothing here relicenses third-party code or grants rights to proprietary SDK components.
