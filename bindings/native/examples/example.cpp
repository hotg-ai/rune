#include "rune.h"
#include <exception>
#include <iostream>
#include <fstream>
#include <string>
#include <string_view>
#include <vector>

class Logger
{
public:
    Error_t *on_log(LogRecord_t record)
    {
        std::cerr << "[" << rune_log_level_name(record.level) << " " << str(record.target) << "\n";
        return nullptr;
    }

private:
    std::string_view str(slice_raw_uint8_t slice)
    {
        return std::string_view{(const char *)(slice.ptr), slice.len};
    }
};

BoxDynFnMut1_Error_ptr_LogRecord_t logger_as_closure(Logger *instance)
{
    auto on_log = [](void *state, LogRecord_t record) -> Error_t * {
        return reinterpret_cast<Logger *>(state)->on_log(record);
    };
    auto deleter = [](void *state)
    { delete (Logger *)state; };

    return BoxDynFnMut1_Error_ptr_LogRecord_t{instance, on_log, deleter};
}

std::vector<char> read_file(std::string filename)
{
    std::ifstream file;
    file.exceptions(std::ifstream::badbit | std::ifstream::failbit);
    file.open(filename, std::ios::binary | std::ios::ate);

    auto size = file.tellg();
    file.seekg(0, std::ios::beg);

    std::vector<char> buffer(size);

    if (file.read(buffer.data(), size))
    {
        return buffer;
    }
    else
    {
        throw std::runtime_error("Unable to read the file");
    }
}

int main(int argc, char **argv)
{
    if (argc != 2)
    {
        std::cerr << "Usage: " << argv[0] << " <rune>"
                  << "\n";
        return 1;
    }

    auto rune = read_file(argv[1]);
    slice_ref_uint8_t wasm{
        (uint8_t *)rune.data(),
        rune.size(),
    };

    RunicosBaseImage_t *image = rune_image_new();

    // We can't pass our Logger directly to rune_image_set_log() so we need to
    // wrap it in a vtable and pass ownership of a new Logger.
    auto log = logger_as_closure(new Logger());
    rune_image_set_log(image, log);

    WasmerRuntime *runtime;
    auto error = rune_wasmer_runtime_load(wasm, image, &runtime);
    if (error)
    {
        const char *error_msg = rune_error_to_string_verbose(error);
        std::cerr << error_msg << "\n";
        delete error_msg;
        return 1;
    }

    rune_wasmer_runtime_free(runtime);

    return 0;
}
