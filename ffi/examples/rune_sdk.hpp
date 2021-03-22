#pragma once

#include <algorithm>
#include <cstdint>
#include <iostream>
#include <fstream>
#include <vector>
#include <iterator>
#include <memory>
#include <cassert>
#include <string_view>
#include "./rune.h"

namespace
{
    uintptr_t round_down(uintptr_t n, uintptr_t m)
    {
        return (n / m) * m;
    }
}

namespace rune
{
    class BaseCapability
    {
    public:
        virtual int generate(char *buffer, int len) = 0;
        virtual int set_parameter(std::string_view name, uint8_t value) { return -1; }
        virtual int set_parameter(std::string_view name, int16_t value) { return -1; }
        virtual int set_parameter(std::string_view name, int32_t value) { return -1; }
        virtual int set_parameter(std::string_view name, float value) { return -1; }
        virtual ~BaseCapability() {}
    };

    class BaseOutput
    {
    public:
        virtual int consume(const char *buffer, int len) = 0;
        virtual ~BaseOutput() {}
    };

    class BaseEnvironment
    {
    public:
        virtual void log(int level, std::string_view target, std::string_view message)
        {
            const char *level_names[] = {
                "",
                "ERROR",
                "WARN ",
                "INFO ",
                "DEBUG",
                "TRACE",
            };
            const char *level_name = 1 <= level && level <= 5 ? level_names[level] : "INFO ";

            std::cout << "[" << level_name << " " << target << "] " << message << std::endl;
        }

        virtual BaseCapability *random() { return nullptr; }
        virtual BaseCapability *accelerometer() { return nullptr; }
        virtual BaseCapability *image() { return nullptr; }
        virtual BaseCapability *sound() { return nullptr; }

        virtual BaseOutput *serial() { return nullptr; }

        virtual ~BaseEnvironment() {}
    };

    int set_capability_parameter(void *raw_item, const char *key_buffer, int key_len, const char *data, int data_len, Type type)
    {
        assert(raw_item);
        BaseCapability &item = (BaseCapability &)raw_item;
        std::string_view key{key_buffer, (size_t)key_len};

        switch (type)
        {
        case Type::Byte:
            assert((size_t)data_len >= sizeof(uint8_t));
            return item.set_parameter(key, *(uint8_t *)data);
        case Type::Short:
            assert((size_t)data_len >= sizeof(int16_t));
            return item.set_parameter(key, *(int16_t *)data);
        case Type::Integer:
            assert((size_t)data_len >= sizeof(int32_t));
            return item.set_parameter(key, *(int32_t *)data);
        case Type::Float:
            assert((size_t)data_len >= sizeof(float));
            return item.set_parameter(key, *(float *)data);

        default:
            return -1;
        }
    }

    int make_capability(BaseCapability *capability, Capability *dest)
    {
        if (!capability)
        {
            return -1;
        }

        *dest = {
            .user_data = capability,
            .generate = [](void *item, char *buffer, int len) {
            auto t = (BaseCapability *)item;
            return t->generate(buffer, len); },
            .set_parameter = set_capability_parameter,
            .destroy = [](void *item) { delete (BaseCapability *)item; },
        };

        return 0;
    }

    int make_output(BaseOutput *output, Output *dest)
    {
        if (!output)
        {
            return -1;
        }

        *dest = {
            .user_data = output,
            .consume = [](void *item, const char *buffer, int len) {
            auto t = (BaseOutput *)item;
            return t->consume(buffer, len); },
            .destroy = [](void *item) { delete (BaseOutput *)item; },
        };

        return 0;
    }

    template <typename T>
    Callbacks make_callbacks(T &&environment)
    {
        return Callbacks{
            .user_data = new T{environment},
            .log = [](void *env, int level, const char *target_buffer, int target_len, const char *msg_buffer, int msg_len) {
                std::string_view msg{msg_buffer, (size_t)msg_len};
                std::string_view target{target_buffer, (size_t)target_len};
                ((BaseEnvironment *)env)->log(level, target, msg); },
            .random = [](void *env, Capability *cp) {
                auto cap = ((BaseEnvironment *)env)->random();
                return make_capability(cap, cp); },
            .accelerometer = [](void *env, Capability *cp) {
                auto cap = ((BaseEnvironment *)env)->accelerometer();
                return make_capability(cap, cp); },
            .image = [](void *env, Capability *cp) {
                auto cap = ((BaseEnvironment *)env)->image();
                return make_capability(cap, cp); },
            .sound = [](void *env, Capability *cp) {
                auto cap = ((BaseEnvironment *)env)->sound();
                return make_capability(cap, cp); },
            .serial = [](void *env, Output *out) {
                auto output = ((BaseEnvironment *)env)->serial();
                return make_output(output, out); },
            .destroy = [](void *env) { delete (BaseEnvironment *)env; },
        };
    }
} // namespace rune
