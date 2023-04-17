#include <iostream>

using namespace std;

int main()
{
    int a, b;
    cin >> a >> b;

    int max = b, n = b;
    for (int i = 0; i < 9; i++)
    {
        cin >> a >> b;

        n = n-a+b;
        if (n > max)
        {
            max = n;
        }
    }

    cout << max;
}