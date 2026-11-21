"""Wine regression: real TTY startup, interactive input, and script output.
A PTY is essential: pipe-only -c tests miss the enhanced-REPL import failure.
"""
from pathlib import Path
import os
import pty
import re
import select
import subprocess
import sys
import time

EXE = str(Path(sys.argv[1]).resolve())
ENV = os.environ.copy()
ENV.update(WINEPREFIX=str(Path(os.environ['TMPDIR']) / 'rustpython-loader-wine64'),
           WINEARCH='win64', WINEDEBUG='-all')
ENV.pop('PYTHON_BASIC_REPL', None)
ENV.pop('PYTHONSTARTUP', None)
ENV.pop('PYTHONINSPECT', None)
ANSI = re.compile(rb'\x1b\[[0-?]*[ -/]*[@-~]')

# No flags/environment workaround: the same launch path as double-clicking.
master, slave = pty.openpty()
child = subprocess.Popen(['wine', EXE], env=ENV, stdin=slave, stdout=slave,
                         stderr=slave, start_new_session=True)
os.close(slave)
output = b''
try:
    deadline = time.monotonic() + 25
    while time.monotonic() < deadline:
        if select.select([master], [], [], 0.1)[0]:
            try:
                chunk = os.read(master, 65536)
            except OSError:
                break
            if not chunk:
                break
            output += chunk
            if b'>>>' in ANSI.sub(b'', output):
                break
        if child.poll() is not None:
            break
finally:
    if child.poll() is None:
        child.terminate()
    child.wait(timeout=5)
    os.close(master)
plain = ANSI.sub(b'', output).decode(errors='replace')
print('TTY STARTUP OUTPUT:\n' + plain)
assert 'Could not import _pyrepl' not in plain, 'enhanced REPL selected without its os dependency'
assert 'ImportError:' not in plain, plain
assert '>>>' in plain, 'basic prompt never appeared'
print('PASS: bare TTY launch reaches basic prompt without importing _pyrepl')

# Feed the actual interactive parser, including state, multiline syntax, recovery.
commands = ('x = 40\nprint("Hello, World!")\nprint(x + 2)\n'
            'for i in range(2):\n    print("LOOP", i)\n\n'
            '1 / 0\nprint("AFTER_ERROR")\nimport sys; sys.exit(0)\n')
r = subprocess.run(['wine', EXE, '-i'], input=commands, env=ENV,
                   capture_output=True, text=True, timeout=45)
print('INTERACTIVE EXIT:', r.returncode, '\n' + r.stdout + r.stderr)
assert r.returncode == 0
assert 'Hello, World!' in r.stdout
assert '42' in r.stdout.splitlines()
assert 'LOOP 0' in r.stdout and 'LOOP 1' in r.stdout
assert 'AFTER_ERROR' in r.stdout
assert 'ZeroDivisionError' in r.stderr
assert 'Could not import _pyrepl' not in r.stderr
print('PASS: interactive commands, multiline input, state and error recovery')

# Read an actual CRLF .py file using the same command the user attempted.
script = Path(os.environ['TMPDIR']) / 'RustPython Hello Test.py'
script.write_bytes(b'print("Hello, World!")\r\n')
r = subprocess.run(['wine', EXE, str(script)], env=ENV, capture_output=True,
                   text=True, timeout=45)
print('SCRIPT EXIT:', r.returncode, repr(r.stdout), r.stderr)
assert r.returncode == 0 and r.stdout.strip() == 'Hello, World!'
print('PASS: script file with spaces and CRLF prints Hello, World!')
