#include <array>
#include <cstdint>
#include <iostream>
#include <print>


constexpr size_t MAX_N = 80;
constexpr std::array FIBO_SIDES = [] static -> std::array<uint64_t, MAX_N + 1>
{
    std::array<uint64_t, MAX_N + 1> arr{};
    arr[1] = 1;

    if constexpr (MAX_N >= 2)
    {
        arr[2] = 1;
    }

    size_t i = 3;
    while (i <= MAX_N)
    {
        arr[i] = arr[i - 1] + arr[i - 2];
        ++i;
    }
    return arr;
}();

int main()
{
    size_t n;
    std::cin >> n;

    std::println("{}", (FIBO_SIDES[n] * 4) + (FIBO_SIDES[n - 1] * 2));
}
