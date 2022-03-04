#include <gtest/gtest.h>
#include "rune.h"

// Demonstrate some basic assertions.
TEST(HelloTest, BasicAssertions)
{
    Config config{};
    Runtime *runtime;
    Error *error = rune_runtime_load(&config, &runtime);

    if (error)
    {
        auto msg = rune_error_to_string_verbose(error);
        FAIL() << msg;
    }
}
