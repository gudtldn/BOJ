#include <iostream>

using namespace std;

int main()
{
    int t, s;
    cin >> t >> s;

    if (s == 1)
    {
        cout << 280;
        return 0;
    }

    if (t < 12) // 아침
    {
        cout << 280;
    }
    else if (12 <= t && t <= 16) // 점심
    {
        cout << 320;
    }
    else // 저녁
    {
        cout << 280;
    }

    return 0;
}