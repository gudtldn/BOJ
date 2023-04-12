#include <iostream>

using namespace std;

int fib(int n);

int main()
{
    int n;
    cin >> n;

    cout << fib(n) << " " << n - 2;

    return 0;
}

int fib(int n)
{
    if (n == 1 || n == 2)
    {
        return 1;
    }

    int a = 0,
        b = 1;

    for (int i = 0; i < n; i++)
    {
        int temp_a = a;
        a = b;
        b = temp_a + b;
    }

    return a;
}
