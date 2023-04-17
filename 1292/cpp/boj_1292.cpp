#include <iostream>

using namespace std;

int main()
{
    int n = 1;
    int seq[1035];
    for (int i = 0; i < 1035; i += n-1)
    {
        for (int j = 0; j < n; j++)
        {
            seq[i+j] = n;
        }
        n++;
    }

    int a, b;
    cin >> a >> b;

    int sum = 0;
    for (; a <= b; a++)
    {
        sum += seq[a-1];
    }
    
    cout << sum;

    return 0;
}