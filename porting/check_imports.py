"""Read actual PE32 imports; reject stacker's post-Win98 static imports."""
import struct
import sys
from pathlib import Path


def imports(path):
    data = Path(path).read_bytes()
    pe = struct.unpack_from('<I', data, 0x3c)[0]
    assert data[pe:pe+4] == b'PE\0\0'
    sections = struct.unpack_from('<H', data, pe+6)[0]
    size = struct.unpack_from('<H', data, pe+20)[0]
    opt = pe+24
    assert struct.unpack_from('<H', data, opt)[0] == 0x10b
    ranges = []
    for i in range(sections):
        start = opt+size+i*40
        vsize, rva, rawsize, raw = struct.unpack_from('<IIII', data, start+8)
        ranges.append((rva, max(vsize, rawsize), raw))
    def off(rva):
        for base, length, raw in ranges:
            if base <= rva < base+length:
                return raw+rva-base
        raise ValueError(f'unmapped RVA {rva:x}')
    def text(pos):
        return data[pos:data.index(b'\0', pos)].decode('ascii')
    rva = struct.unpack_from('<I', data, opt+104)[0]
    if not rva:
        return []
    pos = off(rva)
    result = []
    while True:
        original, _, _, name, thunk = struct.unpack_from('<IIIII', data, pos)
        if not name:
            break
        dll = text(off(name))
        entry = off(original or thunk)
        while True:
            value = struct.unpack_from('<I', data, entry)[0]
            if not value:
                break
            symbol = f'#{value & 0xffff}' if value & 0x80000000 else text(off(value)+2)
            result.append((dll, symbol))
            entry += 4
        pos += 20
    return result


if __name__ == '__main__':
    entries = imports(sys.argv[1])
    for dll, symbol in entries:
        print(f'{dll}:{symbol}')
    banned = {'ConvertFiberToThread', 'IsThreadAFiber', 'SetThreadStackGuarantee'}
    found = sorted(symbol for _, symbol in entries if symbol in banned)
    assert not found, f'post-Win98 stacker imports remain: {found}'
    print('PASS: no static imports for the three post-Win98 stacker APIs')
