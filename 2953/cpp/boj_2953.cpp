#include <iostream>

using namespace std;

int main()
{
    int max[2] = {-1, -1};
    for (int i = 0; i < 5; i++)
    {
        int sum = 0;
        for (int j = 0; j < 4; j++)
        {
            int a;
            cin >> a;

            sum += a;
        }

        if (sum > max[1])
        {
            max[0] = i;
            max[1] = sum;
        }
    }

    cout << max[0]+1 << " " << max[1];

    return 0;
}