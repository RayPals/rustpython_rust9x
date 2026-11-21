# Third-party port notices

- RustPython source baseline: https://github.com/RustPython/RustPython/commit/e634ec61cf9601d69830326e495128434d47e8cf — MIT, see LICENSE and existing source notices.
- rust9x toolchain: https://github.com/rust9x/rust — separately distributed upstream; no compiler/toolchain archive is included here.
- `compat/stacker-win98`: modified stacker 0.1.25, https://github.com/rust-lang/stacker — MIT OR Apache-2.0; LICENSE-MIT and LICENSE-APACHE are retained in that directory.
- `compat/clipboard-win98`: modified clipboard-win 5.4.1, https://github.com/DoumanAsh/clipboard-win — BSL-1.0; see its Cargo metadata and LICENSE.

The compatibility copies modify legacy Windows API handling. Upstream authorship and license terms remain intact. Other upstream dependencies and Python-library files retain their own notices. Proprietary SDKs and CRT/import-library payloads are not included.
