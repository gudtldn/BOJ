#include <iostream>
#include <vector>
#include <map>

using namespace std;

int mean(vector<int> vec)
{
    int sum = 0;
    for (int v : vec)
    {
        sum += v;
    }

    return sum / vec.size();
}

int mode(vector<int> vec)
{
    map<int, int> dict;
    for (int v : vec)
    {
        if (dict.find(v) != dict.end())
        {
            dict[v]++;
        }
        else
        {
            dict.insert({v, 1});
        }
    }

    pair<int, int> max(-1, -1);
    for (auto i : dict)
    {
        if (i.second > max.second)
        {
            max = i;
        }
    }

    return max.first;
}

int main()
{
    int a;
    vector<int> numbers;
    
    for (int i = 0; i < 10; i++)
    {
        cin >> a;
        numbers.push_back(a);
    }

    cout << mean(numbers) << "\n" << mode(numbers);

    return 0;
}