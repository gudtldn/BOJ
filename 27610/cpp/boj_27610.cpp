#include <iostream>
#include <string>
#include <map>

using namespace std;

int main()
{
    map<string, int> fruit = {{"STRAWBERRY", 0}, {"BANANA", 0}, {"LIME", 0}, {"PLUM", 0}};
    int n;
    cin >> n;

    while (n--)
    {
        string s;
        int x;
        cin >> s >> x;
        fruit[s] += x;
    }

    for (const auto& f : fruit)
    {
        if (f.second == 5)
        {
            cout << "YES";
            return 0;
        }
    }
    cout << "NO";
    return 0;
}