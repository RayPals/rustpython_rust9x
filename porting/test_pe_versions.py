"""Regression check for the Win9x 'expects a newer Windows' loader gate.
This verifies only PE header versions, not imports or runtime compatibility.
"""
import struct
import sys
from pathlib import Path


def check(path):
    data = Path(path).read_bytes()
    assert data[:2] == b'MZ', 'not a DOS/PE executable'
    pe = struct.unpack_from('<I', data, 0x3c)[0]
    assert data[pe:pe + 4] == b'PE\0\0', 'missing PE signature'
    assert struct.unpack_from('<H', data, pe + 4)[0] == 0x14c, 'not x86'
    opt = pe + 24
    assert struct.unpack_from('<H', data, opt)[0] == 0x10b, 'not PE32'
    os_version = struct.unpack_from('<HH', data, opt + 40)
    subsystem_version = struct.unpack_from('<HH', data, opt + 48)
    assert os_version <= (4, 0), f'OS version {os_version} excludes Win9x'
    assert subsystem_version == (4, 0), f'subsystem version {subsystem_version}, expected 4.0'
    assert struct.unpack_from('<H', data, opt + 68)[0] == 3, 'not a console executable'
    print(f'PASS: {path}: PE32 x86, OS={os_version}, console subsystem={subsystem_version}')


if __name__ == '__main__':
    check(sys.argv[1])
