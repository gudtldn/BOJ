// 집합
// https://www.acmicpc.net/problem/11723

#include <iostream>
#include <cstdint>
#include <string>
#include <bitset>


void solution()
{
    using namespace std;
    bitset<20> set;

    uint32_t m;
    cin >> m;

    for (uint32_t i = 0; i < m; ++i)
    {
        string command;
        cin >> command;

        if (command == "add")
        {
            uint32_t x;
            cin >> x;
            set.set(x - 1);
        }
        else if (command == "remove")
        {
            uint32_t x;
            cin >> x;
            set.reset(x - 1);
        }
        else if (command == "check")
        {
            uint32_t x;
            cin >> x;
            cout << set.test(x - 1) << '\n';
        }
        else if (command == "toggle")
        {
            uint32_t x;
            cin >> x;
            set.flip(x - 1);
        }
        else if (command == "all")
        {
            set.set();
        }
        else if (command == "empty")
        {
            set.reset();
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
