#include <iostream>
#include <vector>
#include <cstdint>
#include "tensorflow/lite/experimental/microfrontend/lib/noise_reduction.h"

int main()
{
    NoiseReductionState state = {};
    std::cin >> state.smoothing_bits >> state.even_smoothing >> state.odd_smoothing >> state.min_signal_remaining >> state.num_channels;

    state.estimate = (uint32_t *)calloc(state.num_channels, sizeof(uint32_t));

    std::vector<uint32_t> items{};

    uint32_t value = 0;

    while (std::cin >> value)
    {
        items.push_back(value);
    }

    NoiseReductionApply(&state, &items[0]);

    for (auto item : items)
    {
        std::cout << item << "\n";
    }

    return 0;
}
