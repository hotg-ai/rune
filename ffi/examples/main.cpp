#include <algorithm>
#include <cstdint>
#include <iostream>
#include <fstream>
#include <vector>
#include <iterator>
#include <cassert>
#include <string_view>
#include "rune.h"
#include "./rune_sdk.hpp"

class RandomCapability : public rune::BaseCapability
{
    int generate(char *buffer, int len) override
    {
        // Fill the buffer with a well-known value
        auto begin = (float *)buffer;
        auto end = (float *)round_down((uintptr_t)(buffer + len), sizeof(float));
        std::fill(begin, end, 42.0);

        return len;
    }
};

class SerialOutput : public rune::BaseOutput
{
public:
    int consume(const char *buffer, int len) override
    {
        std::string_view message{buffer, (size_t)len};

        std::cout << "Serial: " << message << std::endl;
        return len;
    }
};

class DummyEnvironment : public rune::BaseEnvironment
{
public:
    rune::BaseCapability *random() override { return new RandomCapability{}; }
    rune::BaseOutput *serial() override { return new SerialOutput{}; }
};

void print_error(rune::Error *error, std::string_view preamble)
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

    auto cb = rune::make_callbacks(DummyEnvironment{});

    auto result = rune::rune_runtime_load(buffer.data(), buffer.size(), cb);

    rune::Runtime *runtime;

    switch (result.tag)
    {
    case rune::RuntimeResult::Tag::Ok:
        runtime = result.ok._0;
        break;
    case rune::RuntimeResult::Tag::Err:
        print_error(result.err._0, "Unable to load the runtime");
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
