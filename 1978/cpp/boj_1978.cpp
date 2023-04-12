#include <iostream>
#include <cmath>
#include <vector>
#include <algorithm>

using namespace std;

// 틀림
int main()
{
    vector<int> primes;
    vector<bool> is_prime(1001, true);

    for (int i = 2; i <= sqrt(1000); i++)
    {
        if (is_prime[i])
        {
            for (int j = i * i; j <= 1000; j += i)
            {
                is_prime[j] = false;
            }
        }
    }

    for (int i = 3; i <= 1000; i++)
    {
        if (is_prime[i])
        {
            primes.push_back(i);
        }
    }

    int n;
    cin >> n;

    vector<int> n_vec;

    for (int i = 0; i < n; i++)
    {
        int temp;
        cin >> temp;

        n_vec.push_back(temp);
    }

    vector<int> intersection_vec;
    set_intersection(n_vec.begin(), n_vec.end(), primes.begin(), primes.end(), inserter(intersection_vec, intersection_vec.begin()));

    cout << intersection_vec.size();

    return 0;
}