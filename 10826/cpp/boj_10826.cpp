#include <iostream>
#include <vector>

using namespace std;

class BigInt
{
private:
    vector<unsigned> num;

public:
    BigInt() : num(1, 0) {}
    BigInt(unsigned n) : num(1, 0)
    {
        while (n)
        {
            num.push_back(n % 10);
            n /= 10;
        }
    }

    BigInt operator+(const BigInt &rhs) const
    {
        BigInt res;
        res.num.pop_back();

        size_t len = max(num.size(), rhs.num.size());
        unsigned carry = 0;
        for (size_t i = 0; i < len; i++)
        {
            unsigned sum = carry;
            if (i < num.size())
                sum += num[i];
            if (i < rhs.num.size())
                sum += rhs.num[i];

            res.num.push_back(sum % 10);
            carry = sum / 10;
        }

        if (carry)
            res.num.push_back(carry);

        return res;
    }

    friend ostream &operator<<(ostream &os, const BigInt &bi)
    {
        for (auto it = bi.num.rbegin(); it != bi.num.rend(); it++)
            os << *it;
        return os;
    }
};

int main()
{
    unsigned n;
    cin >> n;

    vector<BigInt> fibo(n+1, 0);
    fibo[0] = 0;
    fibo[1] = 1;

    for (size_t i = 2; i <= n; i++)
    {
        fibo[i] = fibo[i-1] + fibo[i-2];
    }

    cout << fibo[n] << '\n';

    return 0;
}