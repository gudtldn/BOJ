#include <iostream>
#include <string>

using namespace std;

int main()
{
    int t;
    cin >> t;

    while (t--)
    {
        string n;
        cin >> n;

        string n_square = to_string(stoi(n) * stoi(n));
        cout << (n_square.ends_with(n) ? "YES" : "NO") << "\n";
    }
}