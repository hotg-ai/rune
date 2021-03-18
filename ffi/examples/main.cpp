#include <algorithm>
#include <cstdint>
#include <iostream>
#include <fstream>
#include <vector>
#include <iterator>
#include <cassert>
#include <string_view>
#include "./rune.h"

void log(void *, const char *msg, int len)
{
    std::string_view message{msg, (size_t)len};
    std::cout << message << std::endl;
}

uintptr_t round_down(uintptr_t n, uintptr_t m)
{
    return (n / m) * m;
}

int make_random(void *, Capability *cap)
{
    *cap = {
        .user_data = nullptr,
        .generate = [](void *, char *buffer, int len) {
            // Fill the buffer with a well-known value
            auto begin = (float *)buffer;
            auto end = (float *)round_down((uintptr_t)(buffer + len), sizeof(float));
            std::fill(begin, end, 42.0);

            return len; },
        .set_parameter = [](void *, const char *, int, const char *, int, Type) { return 1; },
        .destroy = nullptr,
    };
    return 0;
}

int make_serial(void *, Output *out)
{
    *out = {
        .user_data = nullptr,
        .consume = [](void *d, const char *b, int l) { log(d, b, l); return 0; },
        .destroy = nullptr,
    };
    return 0;
}

void print_error(Error *error, std::string_view preamble)
{
    const char *msg = rune_error_msg(error);
    std::cerr << preamble << ": " << msg << std::endl;

    delete msg;
    rune_error_free(error);
}

int main(int argc, char **argv)
{
    if (argc == 1)
    {
        std::cerr << "Usage: " << argv[0] << " <filename>" << std::endl;
        return 1;
    }

    std::ifstream file{argv[1], std::ios::binary};
    file.unsetf(std::ios::skipws);

    file.seekg(0, std::ios::end);
    auto length = file.tellg();
    file.seekg(0, std::ios::beg);

    std::vector<uint8_t> buffer;
    buffer.reserve(length);

    buffer.insert(buffer.begin(),
                  std::istream_iterator<uint8_t>{file},
                  std::istream_iterator<uint8_t>{});

    Callbacks cb{
        .user_data = nullptr,
        .log = log,
        .random = make_random,
        .accelerometer = nullptr,
        .image = nullptr,
        .serial = make_serial,
        .destroy = nullptr,
    };

    auto result = rune_runtime_load(buffer.data(), buffer.size(), cb);

    Runtime *runtime;

    switch (result.tag)
    {
    case RuntimeResult_Tag::Ok:
        runtime = result.ok;
        break;
    case RuntimeResult_Tag::Err:
        print_error(result.err, "Unable to load the runtime");
        return 1;

    default:
        std::cerr << "An error occurred" << std::endl;
        return 1;
    }

    auto error = rune_runtime_call(runtime);

    if (error)
    {
        print_error(error, "Call failed");
        return 1;
    }

    rune_runtime_free(runtime);

    return 0;
}
