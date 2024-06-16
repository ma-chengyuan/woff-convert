#include <cstdint>

#include "woff2/decode.h"
#include "woff2/encode.h"
#include "woff2/output.h"

#include <string>

extern "C" {
size_t woff2_MaxWOFF2CompressedSize(const uint8_t *data, size_t length) {
    return woff2::MaxWOFF2CompressedSize(data, length);
}

// Compresses the font into the target buffer. *result_length should be at least
// the value returned by MaxWOFF2CompressedSize(), upon return, it is set to the
// actual compressed size. Returns true on successful compression.
bool woff2_ConvertTTFToWOFF2(const uint8_t *data, size_t length,
                             uint8_t *result, size_t *result_length,
                             int32_t quality) {
    woff2::WOFF2Params params;
    params.brotli_quality = quality;
    return woff2::ConvertTTFToWOFF2(data, length, result, result_length,
                                    params);
}
// Compute the size of the final uncompressed font, or 0 on error.
size_t woff2_ComputeWOFF2FinalSize(const uint8_t *data, size_t length) {
    return woff2::ComputeWOFF2FinalSize(data, length);
}

// Decompresses the font into the target buffer. The result_length should
// be the same as determined by ComputeFinalSize(). Returns true on successful
// decompression.
// DEPRECATED; please prefer the version that takes a WOFF2Out*
bool woff2_ConvertWOFF2ToTTF(uint8_t *result, size_t result_length,
                             const uint8_t *data, size_t length) {
    return woff2::ConvertWOFF2ToTTF(result, result_length, data, length);
}

std::string *woff2_ConvertWOFF2ToTTFString(const uint8_t *data, size_t length,
                                           size_t *result_length) {
    const auto initial_size = std::min(
        woff2::ComputeWOFF2FinalSize(data, length), woff2::kDefaultMaxSize);
    const auto s = new std::string(initial_size, '\0');
    woff2::WOFF2StringOut output{s};
    const auto success = woff2::ConvertWOFF2ToTTF(data, length, &output);
    if (!success) {
        delete s;
        return nullptr;
    }
    *result_length = output.Size();
    return s;
}

void woff2_ConvertWOFF2ToTTFStringFinalize(uint8_t *result,
                                           size_t result_length,
                                           std::string *s) {
    std::memcpy(result, s->data(), result_length);
    delete s;
}
}

// Polyfill for missing functions in wasm32-unknown-emscripten target
// Emscripten will fill them in for us usually, but not if we are compiling and
// linking from Rust...

#ifdef __EMSCRIPTEN__

extern "C" {
void *malloc(size_t size);

void free(void *ptr);

void exit(int status);

void abort() { exit(1); }

// Used by
// https://github.com/google/woff2/blob/0f4d304faa1c62994536dc73510305c7357da8d4/src/buffer.h#L98
uint16_t ntohs(uint16_t netshort) { return (netshort >> 8) | (netshort << 8); }

// Used by
// https://github.com/google/woff2/blob/0f4d304faa1c62994536dc73510305c7357da8d4/src/buffer.h#L123
uint32_t ntohl(uint32_t netlong) {
    return ((netlong >> 24) & 0xff) | ((netlong >> 8) & 0xff00) |
           ((netlong << 8) & 0xff0000) | ((netlong << 24) & 0xff000000);
}

// Used by
// https://github.com/google/woff2/blob/0f4d304faa1c62994536dc73510305c7357da8d4/src/font.cc#L45-L48
wchar_t *wmemchr(const wchar_t *ptr, wchar_t ch, size_t count) {
    for (size_t i = 0; i < count; i++) {
        if (ptr[i] == ch) {
            return (wchar_t *)&ptr[i];
        }
    }
    return nullptr;
}
}

void *operator new(size_t size) { return malloc(size); }

void *operator new[](size_t size) { return malloc(size); }

void operator delete(void *ptr) noexcept { free(ptr); }

void operator delete[](void *ptr) noexcept { free(ptr); }

#endif