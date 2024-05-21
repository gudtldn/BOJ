#include <iostream>
#include <string>
#include <sstream>

using namespace std;

static constexpr char stroke[] = {'3', '2', '1', '2', '3', '3', '2', '3', '3', '2', '2', '1', '2', '2', '1', '2', '2', '2', '1', '2', '1', '1', '1', '2', '2', '1'};

int main()
{
    string a, b;
    cin >> a >> b;

    string comp;
    for (size_t i = 0; i < a.size(); ++i)
    {
        comp += stroke[a[i] - 'A'];
        comp += stroke[b[i] - 'A'];
    }

    while (comp.size() > 2)
    {
        string temp;
        for (size_t i = 0; i < comp.size() - 1; ++i)
        {
            temp += (comp[i] + comp[i + 1] - '0' - '0') % 10 + '0';
        }
        comp = temp;
    }

    cout << comp << endl;
    return 0;
}
