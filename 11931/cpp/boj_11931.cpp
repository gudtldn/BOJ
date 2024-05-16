#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    std::cout.tie(nullptr);

    int n;
    std::cin >> n;

    std::vector<int> vec(n);
    for (int& value : vec) {
        std::cin >> value;
    }

    std::sort(vec.rbegin(), vec.rend());
    for (const int& value : vec) {
        std::cout << value << '\n';
    }

    return 0;
}