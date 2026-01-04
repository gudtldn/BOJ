// 알파벳 전부 쓰기
// https://www.acmicpc.net/problem/11091

#include <iostream>
#include <cstdint>
#include <string>
#include <array>
#include <ranges>


void solution()
{
    uint32_t n;
    std::cin >> n;

    for (uint32_t i = 0; i < n; ++i)
    {
        std::string str;
        std::getline(std::cin >> std::ws, str);

        std::array<bool, 26> alphabet_used = { false, };
        for (const char ch : str)
        {
            if (ch >= 'a' && ch <= 'z')
            {
                alphabet_used[ch - 'a'] = true;
            }
            else if (ch >= 'A' && ch <= 'Z')
            {
                alphabet_used[ch - 'A'] = true;
            }
        }

        std::string result;
        result.reserve(26);

        for (auto [idx, used] : alphabet_used | std::views::enumerate)
        {
            if (!used)
            {
                result += static_cast<char>('a' + idx);
            }
        }

        if (result.empty())
        {
            std::cout << "pangram";
        }
        else
        {
            std::cout << "missing " << result;
        }
    }
}

int main()
{
    std::cin.tie(nullptr);
    std::cout.tie(nullptr);
    std::ios::sync_with_stdio(false);

    solution();

    return 0;
}
