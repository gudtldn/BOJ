#include <iostream>
#include <string>
#include <vector>
#include <set>
#include <algorithm>

using namespace std;

int main()
{
    ios::sync_with_stdio(false);
    cin.tie(NULL);

    int n;
    cin >> n;

    set<string> logs;
    while (n--)
    {
        string name, log;
        cin >> name >> log;

        if (log == "enter")
            logs.insert(name);
        else
            logs.erase(name);
    }

    vector<string> v_logs(logs.begin(), logs.end());
    sort(v_logs.rbegin(), v_logs.rend());
    for (const string& name : v_logs)
    {
        cout << name << '\n';
    }
}