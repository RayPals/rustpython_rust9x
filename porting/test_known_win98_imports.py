from check_imports import imports
import sys

banned = {'GetCurrentThreadStackLimits', 'SetThreadStackGuarantee',
          'ConvertFiberToThread', 'IsThreadAFiber', 'GetModuleHandleExW',
          'GetTickCount64', 'Module32FirstW', 'Module32NextW',
          'Process32FirstW', 'Process32NextW', 'ImpersonateAnonymousToken'}
found = sorted(symbol for _, symbol in imports(sys.argv[1]) if symbol in banned)
assert not found, f'Known Win98-incompatible imports remain: {found}'
print('PASS: none of the 11 identified post-Win98 imports remain')
