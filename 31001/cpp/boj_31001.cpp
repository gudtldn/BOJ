// https://www.acmicpc.net/problem/31001

#include <iostream>
#include <unordered_map>
#include <string>
#include <vector>

using namespace std;
using ll = long long;

using StockInfo = unordered_map<string, ll>;
using GroupInfo = unordered_map<ll, vector<string>>;


int main()
{
    cin.tie(nullptr);
    cout.tie(nullptr);
    ios::sync_with_stdio(false);

    ll n, money, q;
    cin >> n >> money >> q;

    StockInfo stock_info {};
    GroupInfo group_info {};
    for (int i = 0; i < n; i++)
    {
        ll group, price;
        string stock_name;
        cin >> group >> stock_name >> price;
        stock_info[stock_name] = price;
        if (group_info.find(group) == group_info.end())
            group_info[group] = vector<string>{stock_name};
        else
            group_info[group].push_back(stock_name);
    }

    StockInfo my_stock_info {};
    for (int i = 0; i < q; i++)
    {
        int query;
        cin >> query;
        switch (query)
        {
            // 입력된 회사의 주식을 amount주 만큼 사기
            case 1:
            {
                string stock_name;
                ll amount;
                cin >> stock_name >> amount;

                if (money >= stock_info[stock_name] * amount)
                {
                    money -= stock_info[stock_name] * amount;
                    if (my_stock_info.find(stock_name) == my_stock_info.end())
                        my_stock_info[stock_name] = amount;
                    else
                        my_stock_info[stock_name] += amount;
                }
                break;
            }

            // 입력된 회사의 주식을 amount주 만큼 팔기
            case 2:
            {
                string stock_name;
                ll amount;
                cin >> stock_name >> amount;

                amount = min(amount, my_stock_info[stock_name]);
                money += stock_info[stock_name] * amount;
                my_stock_info[stock_name] -= amount;
                break;
            }

            // 입력된 회사의 주식 가격을 price만큼 올리기
            case 3:
            {
                string stock_name;
                ll price;
                cin >> stock_name >> price;
                stock_info[stock_name] += price;
                break;
            }

            // 입력된 그룹의 회사들의 주식 가격을 price만큼 올리기
            case 4:
            {
                ll group, price;
                cin >> group >> price;
                for (const auto& stock_name : group_info[group])
                    stock_info[stock_name] += price;
                break;
            }

            // 입력된 그룹의 회사들의 주식 가격을 percent만큼 올리기
            case 5:
            {
                ll group, percent;
                cin >> group >> percent;
                for (const auto& stock_name : group_info[group])
                    stock_info[stock_name] = (stock_info[stock_name] * (100 + percent) / 100) / 10 * 10;
                break;
            }

            // 내 돈 출력
            case 6:
            {
                cout << money << '\n';
                break;
            }

            // 내 자산가치 출력
            case 7:
            {
                ll asset = money;
                for (const auto& [stock_name, amount] : my_stock_info)
                    asset += stock_info[stock_name] * amount;
                cout << asset << '\n';
                break;
            }
        }
    }
}
