"""Regression for the actual Win98SE stack-limit loader failure."""
from pathlib import Path
import sys
from check_imports import imports

exe = Path(sys.argv[1])
banned = {'GetCurrentThreadStackLimits', 'SetThreadStackGuarantee',
          'ConvertFiberToThread', 'IsThreadAFiber'}
found = sorted(name for _, name in imports(exe) if name in banned)
assert not found, f'Win98-incompatible static stack imports: {found}'
print('PASS: no static imports of the four post-Win98 stack/fiber APIs')
