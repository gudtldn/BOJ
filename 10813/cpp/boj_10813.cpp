#include <iostream>
#include <vector>

using namespace std;

void swap(int &a, int &b)
{
    int temp = a;

    a = b;
    b = temp;
}

int main()
{
    int n, m;
    cin >> n >> m;

    vector<int> vec;
    for (int i = 1; i <= n; i++)
        vec.push_back(i);

    for (int i = 0; i < m; i++)
    {
        int x, y;
        cin >> x >> y;

        swap(vec[x - 1], vec[y - 1]);
    }

    for (auto v : vec)
    {
        cout << v << " ";
    }

    return 0;
}