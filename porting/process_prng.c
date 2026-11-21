/* x86 ProcessPrng compatibility using the legacy Windows CryptoAPI.
 * No deterministic fallback: report failure if the OS provider is unavailable.
 * ProcessPrng returns BOOL, not NTSTATUS (zero is FAILURE).
 * Keep provider handles local so concurrent callers do not share mutable state.
 */
#include <windows.h>
#include <wincrypt.h>

BOOL __stdcall rust9x_process_prng(void *buffer, unsigned int length)
{
    HCRYPTPROV provider = 0;
    BOOL result;
    if (length == 0) return TRUE;
    if (buffer == 0) return FALSE;
    if (!CryptAcquireContextA(&provider, 0, 0, PROV_RSA_FULL,
                             CRYPT_VERIFYCONTEXT)) return FALSE;
    result = CryptGenRandom(provider, length, (BYTE *)buffer);
    CryptReleaseContext(provider, 0);
    return result;
}

/* Match windows-link's x86 dllimport slot without FORCE:UNRESOLVED. */
BOOL (__stdcall *rust9x_process_prng_import)(void *, unsigned int)
    __asm__("__imp__ProcessPrng@8") = rust9x_process_prng;
