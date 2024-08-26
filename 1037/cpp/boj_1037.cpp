// https://www.acmicpc.net/problem/1037
#include <iostream>
#include <vector>
#include <algorithm>
#define FASTIO std::cin.tie(nullptr); std::cout.tie(nullptr); std::ios::sync_with_stdio(false);

int main()
{
    using namespace std;
    FASTIO

    int n;
    cin >> n;

    vector<int> vec(n);

    for (int& i : vec)
        cin >> i;

    sort(vec.begin(), vec.end());

    cout << vec.front() * vec.back();
    return 0;
}
