#include <cstdint>
#include <iostream>
#include <fstream>
#include <vector>
#include <iterator>
#include <cassert>
#include <string_view>
#include "./rune.h"

void log(void *data, const char *msg, int len)
{
    std::cout << msg << std::endl;
}

int fill_random(void *data, char *buffer, int len)
{
    for (int i = 0; i < len; i++)
    {
        buffer[i] = rand();
    }

    return len;
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

    auto wasm = argv[1];

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

    Environment env{
        .log = log,
        .fill_random = fill_random,
    };

    auto result = rune_runtime_load(buffer.data(), buffer.size(), env);

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
