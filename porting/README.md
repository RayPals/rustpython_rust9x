# Porting tests and evidence

Run from the repository root against an existing compiled interpreter:

```sh
python3 porting/test_pe_versions.py /path/to/rustpython.exe
python3 porting/test_stack_imports.py /path/to/rustpython.exe
python3 porting/test_known_win98_imports.py /path/to/rustpython.exe
python3 porting/test_basic_repl.py /path/to/rustpython.exe
```

The first three checks parse PE headers/imports using Python's standard library. The REPL regression requires Linux, Wine, and a writable `TMPDIR`; it uses an isolated Wine prefix under that directory. It checks real PTY startup, fed interactive commands, and a CRLF script with spaces in its filename. Wine output is not native Windows 98 acceptance.

`fiber-test/` is a standalone cross-target probe that forces the vendored stacker's optional newer fiber APIs unavailable. It checks the actual production legacy-stack implementation. It still needs the rust9x SDK/link environment. `process_prng.c` supplies the real legacy CryptoAPI implementation used at final link; it must be compiled to x86 COFF and linked with the appropriate import libraries, not merely placed in this directory.

`evidence/` contains historical local execution logs and the import inventory, not freshly generated CI results. No binary is supplied in this publication.

`../WIN98SE_PORT_STATUS.md` is the preserved development record. Its `../loader-fix/`, `../test_pe_versions.py`, and cached-rebuild paths refer to the original experiment workspace; published equivalents are here where included. The old machine-specific incremental rebuild wrappers are deliberately not advertised as a portable clean builder.
