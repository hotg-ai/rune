#include "rune.h"
#include <exception>
#include <iostream>
#include <fstream>
#include <string>
#include <string_view>
#include <vector>

// Helper type for instantiating results with different variants.
template <typename RustType>
struct Result
{
    using OkVariant = decltype(RustType::ok);
    using Value = decltype(OkVariant::value);
    using ErrVariant = decltype(RustType::err);
    using Error = decltype(ErrVariant::error);

    static RustType ok(Value value)
    {
        return RustType{
            .ok = OkVariant{
                .tag = RESULT_TAG_OK,
                .value = value,
            },
        };
    }

    static RustType err(Error error)
    {
        return RustType{
            .err = ErrVariant{
                .tag = RESULT_TAG_ERR,
                .error = error,
            }};
    }
};

template <typename RustType>
bool result_is_ok(const RustType &result)
{
    return result.ok.tag == RESULT_TAG_OK;
}

template <typename RustType>
bool result_is_err(const RustType &result)
{
    return result.err.tag == RESULT_TAG_ERR;
}

template <typename RustType>
auto result_get_value(RustType &result)
{
    if (result_is_ok(result))
    {
        return result.ok.value;
    }
    else
    {
        throw std::runtime_error("Result contains an error");
    }
}

template <typename RustType>
auto result_get_err(RustType &result)
{
    if (result_is_err(result))
    {
        return result.err.error;
    }
    else
    {
        throw std::runtime_error("Result contains a value");
    }
}

class Logger
{
public:
    Result_uint8_Error_ptr_t on_log(LogRecord_t record)
    {
        std::cerr << "[" << rune_log_level_name(record.level) << " " << str(record.target) << "\n";
        return Result<Result_uint8_Error_ptr_t>::ok(0);
    }

private:
    std::string_view str(slice_raw_uint8_t slice)
    {
        return std::string_view{(const char *)(slice.ptr), slice.len};
    }
};

BoxDynFnMut1_Result_uint8_Error_ptr_LogRecord_t logger_as_closure(Logger *instance)
{
    return BoxDynFnMut1_Result_uint8_Error_ptr_LogRecord_t{
        .env_ptr = instance,
        .call = [](void *state, LogRecord_t record) -> Result_uint8_Error_ptr_t
        {
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

Result_Capability_Error_ptr_t make_raw()
{
    Capability_t capability{
        .user_data = nullptr,
        .set_parameter = [](void *state) {},
        .free = nullptr,
    };

    return Result<Result_Capability_Error_ptr_t>::ok(capability);
}

struct Capability
{
    template <typename Func>
    static BoxDynFnMut0_Result_Capability_Error_ptr_t from_factory(Func f)
    {
        return BoxDynFnMut0_Result_Capability_Error_ptr_t{
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

    if (!result_is_ok(result))
    {
        auto error = result_get_err(result);
        const char *error_msg = rune_error_to_string_verbose(error);
        std::cerr << error_msg << "\n";
        delete error_msg;
        rune_error_free(error);
        return 1;
    }

    auto runtime = result_get_value(result);

    rune_wasmer_runtime_call(runtime);

    rune_wasmer_runtime_free(runtime);

    return 0;
}
