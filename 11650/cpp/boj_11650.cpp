#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main()
{
    int n;
    cin >> n;

    vector<pair<int, int>> vec(n);
    for (auto &v : vec)
    {
        cin >> v.first >> v.second;
    }
    sort(vec.begin(), vec.end());

    for (const auto &v : vec)
    {
        cout << v.first << ' ' << v.second << '\n';
    }
}