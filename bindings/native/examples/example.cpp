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
    RuneResult_t *on_log(LogRecord_t record)
    {
        std::cerr << "[" << rune_log_level_name(record.level) << " " << str(record.target) << "\n";
        return rune_result_RuneResult_new_ok(0);
    }

private:
    std::string_view str(slice_raw_uint8_t slice)
    {
        return std::string_view{(const char *)(slice.ptr), slice.len};
    }
};

BoxDynFnMut1_RuneResult_ptr_LogRecord_t logger_as_closure(Logger *instance)
{
    return BoxDynFnMut1_RuneResult_ptr_LogRecord_t{
        .env_ptr = instance,
        .call = [](void *state, LogRecord_t record) -> RuneResult_t * {
            return reinterpret_cast<Logger *>(state)->on_log(record);
        },
        .free = [](void *state)
        { delete (Logger *)state; },
    };
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

CapabilityResult_t *make_raw()
{
    Capability_t capability{
        .user_data = nullptr,
        .set_parameter = [](void *state) {},
        .free = nullptr,
    };

    return rune_result_CapabilityResult_new_ok(capability);
}

struct Capability
{
    template <typename Func>
    static BoxDynFnMut0_CapabilityResult_ptr_t from_factory(Func f)
    {
        return BoxDynFnMut0_CapabilityResult_ptr_t{
            .env_ptr = new Func{f},
            .call = [](void *state)
            {
                auto f = (Func *)state;
                return (*f)();
            },
            .free = [](void *state)
            { delete (Func *)state; },
        };
    }
};

RunicosBaseImage_t *make_image()
{
    RunicosBaseImage_t *image = rune_image_new();

    // We can't pass our Logger directly to rune_image_set_log() so we need to
    // wrap it in a vtable and pass ownership of a new Logger.
    auto log = logger_as_closure(new Logger());
    rune_image_set_log(image, log);

    rune_image_set_raw(image, Capability::from_factory(make_raw));

    return image;
}

void print_error(Error *error)
{
    char *error_msg = rune_error_to_string_verbose(error);
    std::cerr << error_msg << "\n";
    rune_string_free(error_msg);
    rune_error_free(error);
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

    auto image = make_image();
    auto result = rune_wasmer_runtime_load(wasm, image);

    if (rune_result_WasmerRuntimeResult_is_err(result))
    {
        print_error(rune_result_WasmerRuntimeResult_take_err(result));
        return 1;
    }

    auto runtime = rune_result_WasmerRuntimeResult_take_ok(result);

    auto call_result = rune_wasmer_runtime_call(runtime);

    if (rune_result_RuneResult_is_err(call_result))
    {
        print_error(rune_result_RuneResult_take_err(call_result));
        rune_wasmer_runtime_free(runtime);
        return 1;
    }
    else
    {
        rune_result_RuneResult_free(call_result);
    }

    rune_wasmer_runtime_free(runtime);

    return 0;
}
